/// The Persistence Layer stores Data in EEPROM an distributes it to NMEA and Can Bus interfaces
///
/// The persistent layer stores the data in the EEPROM and distributes the data to the NMEA and Can
/// bus interfaces. Data points that can be processed must be recorded by the PersistenceId. The
/// persist_restore_item() method writes the data read from the EEPROM to the CoreModel. The
/// persist_store_item() method stores model data in the EEPROM.
///
/// The set_id() method receives data from the NMEA and CAN bus interfaces and from the editor,
/// saves it in the EEPROM if necessary and distributes the data to interfaces if required. The
/// distribution of the data to the interfaces is controlled via the enum Echo:
///   - Echo::None -> no distribution
///   - Echo::Nmea -> forwarding to the NMEA interface
///   - Echo::Can -> forwarding to the Can bus interface
///   - Echo::NmeaAndCan -> forwarding to NMEA and Can Bus
///
/// The module also ensures that the Nmea interface and the EEPROM are not overloaded by too much
/// data. This is achieved by initially storing the data in an index set and only forwarding it
/// after a pause of incoming data of at least 500 ms.
use heapless::Vec;
use num_enum::FromPrimitive;

use super::{VarioModeControl, MAX_PERS_IDS};
use crate::{
    basic_config::PERSISTENCE_TIMEOUT,
    controller::{
        helpers::{CanConfigId, IntToDuration},
        RemoteConfig,
    },
    flight_physics::polar_store,
    system_of_units::Speed,
    utils::Variant,
    view::viewable::{centerview::CenterView, lineview::LineView},
    CoreController, CoreModel, IdleEvent, Mass, PersistenceItem, Pressure, ResetReason, Rotation,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u16)]
pub enum PersistenceId {
    Volume = 0,
    McCready = 1,
    WaterBallast = 2,
    PilotWeight = 3,
    Glider = 4,
    VarioModeControl = 5,
    DisplayTheme = 6, // Dark = 0, Bright = 1
    Qnh = 7,
    Bugs = 8,
    Display = 9,
    TcClimbRate = 10,
    TcSpeedToFly = 11,
    Info1 = 12,
    Info2 = 13,
    Rotation = 14,
    CenterFrequency = 15,
    CenterViewCircling = 16,
    CenterViewStraight = 17,
    EmptyMass = 18,
    MaxBallast = 19,
    ReferenceWeight = 20,
    PolarValueV1 = 21,
    PolarValueV2 = 22,
    PolarValueV3 = 23,
    PolarValueSi1 = 24,
    PolarValueSi2 = 25,
    PolarValueSi3 = 26,
    LastItem = 27, // Items smaller than this are stored in eeprom

    UserProfile = 65533, // Special function Ids
    DeleteAll = 65534,
    #[default]
    DoNotStore = 65535,
}

const DELETE_CONFIG_LIST: &[PersistenceId] = &[
    PersistenceId::Volume,
    PersistenceId::McCready,
    PersistenceId::WaterBallast,
    PersistenceId::PilotWeight,
    PersistenceId::VarioModeControl,
    PersistenceId::DisplayTheme,
    PersistenceId::Qnh,
    PersistenceId::Bugs,
    PersistenceId::Display,
    PersistenceId::TcClimbRate,
    PersistenceId::TcSpeedToFly,
    PersistenceId::Info1,
    PersistenceId::Info2,
    PersistenceId::Rotation,
    PersistenceId::CenterFrequency,
    PersistenceId::CenterViewCircling,
    PersistenceId::CenterViewStraight,
];

const SPECIFIC_POLAR_SETTINGS: &[PersistenceId] = &[
    PersistenceId::EmptyMass,
    PersistenceId::MaxBallast,
    PersistenceId::ReferenceWeight,
    PersistenceId::PolarValueV1,
    PersistenceId::PolarValueV2,
    PersistenceId::PolarValueV3,
    PersistenceId::PolarValueSi1,
    PersistenceId::PolarValueSi2,
    PersistenceId::PolarValueSi3,
];

#[derive(PartialEq)]
pub enum Echo {
    None,
    Nmea,
    Can,
    NmeaAndCan,
}

/// Restore Items from EEPROM
///
/// This method is called directly from the idle-loop during start-up
pub fn restore_item(cc: &mut CoreController, cm: &mut CoreModel, item: PersistenceItem) {
    match item.id {
        PersistenceId::UserProfile => cm.config.user_profile = item.to_u8(),
        PersistenceId::Volume => cm.config.volume = item.to_i8(),
        PersistenceId::McCready => cm.config.mc_cready = Speed::from_m_s(item.to_f32()),
        PersistenceId::WaterBallast => cm.glider_data.water_ballast = Mass::from_kg(item.to_f32()),
        PersistenceId::PilotWeight => cm.glider_data.pilot_weight = Mass::from_kg(item.to_f32()),
        PersistenceId::Glider => {
            let raw_idx = item.to_i32();
            cm.config.glider_idx = raw_idx;
            cm.glider_data.basic_glider_data = polar_store::POLARS[raw_idx as usize];
            cc.recalc_glider(cm);
        }
        PersistenceId::VarioModeControl => {
            cm.control.vario_mode_control = VarioModeControl::from(item.to_u8())
        }
        PersistenceId::DisplayTheme => {
            cm.config.theme = if item.to_i32() == 0 {
                &cm.device_const.dark_theme
            } else {
                &cm.device_const.bright_theme
            }
        }
        PersistenceId::Qnh => {
            let qnh = Pressure::from_hpa(item.to_f32());
            cm.sensor.pressure_altitude.set_qnh(qnh)
        }
        PersistenceId::Bugs => cm.glider_data.bugs = item.to_f32(),
        PersistenceId::Display => cm.config.display_active = item.to_u8().into(),
        PersistenceId::TcClimbRate => cm.config.av2_climb_rate_tc = item.to_f32(),
        PersistenceId::TcSpeedToFly => cm.config.av_speed_to_fly_tc = item.to_f32(),
        PersistenceId::Info1 => cm.config.info1 = LineView::from(item.to_u8()),
        PersistenceId::Info2 => cm.config.info2 = LineView::from(item.to_u8()),
        PersistenceId::Rotation => cm.control.rotation = Rotation::from(item.to_u8()),
        PersistenceId::CenterFrequency => cm.config.snd_center_freq = item.to_f32(),
        PersistenceId::CenterViewCircling => {
            cm.config.center_circling = CenterView::from(item.to_u8())
        }
        PersistenceId::CenterViewStraight => {
            cm.config.center_straignt = CenterView::from(item.to_u8())
        }
        PersistenceId::EmptyMass => cm.glider_data.basic_glider_data.empty_mass = item.to_f32(),
        PersistenceId::MaxBallast => cm.glider_data.basic_glider_data.max_ballast = item.to_f32(),
        PersistenceId::ReferenceWeight => {
            cm.glider_data.basic_glider_data.reference_weight = item.to_f32()
        }
        PersistenceId::PolarValueV1 => {
            cm.glider_data.basic_glider_data.polar_values[0][0] = item.to_f32()
        }
        PersistenceId::PolarValueV2 => {
            cm.glider_data.basic_glider_data.polar_values[1][0] = item.to_f32()
        }
        PersistenceId::PolarValueV3 => {
            cm.glider_data.basic_glider_data.polar_values[2][0] = item.to_f32()
        }
        PersistenceId::PolarValueSi1 => {
            cm.glider_data.basic_glider_data.polar_values[0][1] = item.to_f32()
        }
        PersistenceId::PolarValueSi2 => {
            cm.glider_data.basic_glider_data.polar_values[1][1] = item.to_f32()
        }
        PersistenceId::PolarValueSi3 => {
            cm.glider_data.basic_glider_data.polar_values[2][1] = item.to_f32()
        }

        _ => (),
    }
}

/// Store Content of PersistenceId in EEPROM
///
/// This method pushs the content into a queue, which is connected to the hardware
pub fn store_item(cc: &mut CoreController, cm: &mut CoreModel, id: PersistenceId) {
    let p_item = match id {
        PersistenceId::UserProfile => PersistenceItem::from_u8(id, cm.config.user_profile),
        PersistenceId::Volume => PersistenceItem::from_i8(id, cm.config.volume),
        PersistenceId::McCready => PersistenceItem::from_f32(id, cm.config.mc_cready.to_m_s()),
        PersistenceId::WaterBallast => {
            PersistenceItem::from_f32(id, cm.glider_data.water_ballast.to_kg())
        }
        PersistenceId::PilotWeight => {
            PersistenceItem::from_f32(id, cm.glider_data.pilot_weight.to_kg())
        }
        PersistenceId::Glider => PersistenceItem::from_i32(id, cm.config.glider_idx),
        PersistenceId::VarioModeControl => {
            PersistenceItem::from_u8(id, cm.control.vario_mode_control as u8)
        }
        PersistenceId::DisplayTheme => {
            let mode = if cm.config.theme == &cm.device_const.dark_theme {
                0
            } else {
                1
            };
            PersistenceItem::from_i32(id, mode)
        }
        PersistenceId::Bugs => PersistenceItem::from_f32(id, cm.glider_data.bugs),
        PersistenceId::Qnh => {
            PersistenceItem::from_f32(id, cm.sensor.pressure_altitude.qnh().to_hpa())
        }
        PersistenceId::Display => PersistenceItem::from_u8(id, cm.config.last_display_active as u8),
        PersistenceId::TcClimbRate => PersistenceItem::from_f32(id, cm.config.av2_climb_rate_tc),
        PersistenceId::TcSpeedToFly => PersistenceItem::from_f32(id, cm.config.av_speed_to_fly_tc),
        PersistenceId::Info1 => PersistenceItem::from_u32(id, cm.config.info1 as u32),
        PersistenceId::Info2 => PersistenceItem::from_u32(id, cm.config.info2 as u32),
        PersistenceId::Rotation => PersistenceItem::from_u32(id, cm.control.rotation as u32),
        PersistenceId::CenterFrequency => PersistenceItem::from_f32(id, cm.config.snd_center_freq),
        PersistenceId::CenterViewCircling => {
            PersistenceItem::from_u32(id, cm.config.center_circling as u32)
        }
        PersistenceId::CenterViewStraight => {
            PersistenceItem::from_u32(id, cm.config.center_straignt as u32)
        }
        PersistenceId::EmptyMass => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.empty_mass)
        }
        PersistenceId::MaxBallast => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.max_ballast)
        }
        PersistenceId::ReferenceWeight => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.reference_weight)
        }
        PersistenceId::PolarValueV1 => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.polar_values[0][0])
        }
        PersistenceId::PolarValueV2 => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.polar_values[1][0])
        }
        PersistenceId::PolarValueV3 => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.polar_values[2][0])
        }
        PersistenceId::PolarValueSi1 => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.polar_values[0][1])
        }
        PersistenceId::PolarValueSi2 => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.polar_values[1][1])
        }
        PersistenceId::PolarValueSi3 => {
            PersistenceItem::from_f32(id, cm.glider_data.basic_glider_data.polar_values[2][1])
        }
        _ => PersistenceItem::do_not_store(),
    };
    cc.send_idle_event(IdleEvent::SetEepromItem(p_item));
}

pub fn persist_set(
    cc: &mut CoreController,
    cm: &mut CoreModel,
    variant: Variant,
    id: PersistenceId,
    echo: Echo,
) {
    match id {
        PersistenceId::Volume => {
            if let Variant::I8(volume) = variant {
                cm.config.volume = volume;
            }
        }
        PersistenceId::McCready => {
            if let Variant::Speed(mc_cready) = variant {
                cm.config.mc_cready = mc_cready;
            }
        }
        PersistenceId::WaterBallast => {
            if let Variant::Mass(water_ballast) = variant {
                cm.glider_data.water_ballast = water_ballast;
            }
        }
        PersistenceId::PilotWeight => {
            if let Variant::Mass(pilot_weight) = variant {
                cm.glider_data.pilot_weight = pilot_weight;
            }
        }
        PersistenceId::Glider => {
            if let Variant::Usize(raw_idx) = variant {
                cm.config.glider_idx = raw_idx as i32;
                cm.glider_data.basic_glider_data = polar_store::POLARS[raw_idx];
                cc.recalc_glider(cm);
                cc.send_idle_event(IdleEvent::ClearEepromItems(SPECIFIC_POLAR_SETTINGS));
            }
        }
        PersistenceId::VarioModeControl => {
            if let Variant::VarioModeControl(vario_mode_control) = variant {
                cm.control.vario_mode_control = vario_mode_control;
            }
        }
        PersistenceId::DisplayTheme => {
            if let Variant::Str(theme_name) = variant {
                cm.config.theme = if theme_name == "Bright" {
                    &cm.device_const.bright_theme
                } else {
                    &cm.device_const.dark_theme
                };
            }
        }
        PersistenceId::Qnh => {
            if let Variant::Pressure(qnh) = variant {
                cm.sensor.pressure_altitude.set_qnh(qnh);
            }
        }
        PersistenceId::Bugs => {
            if let Variant::F32(bugs) = variant {
                cm.glider_data.bugs = bugs;
            }
        }
        PersistenceId::Display => {
            if let Variant::DisplayActive(display_active) = variant {
                cm.config.display_active = display_active;
                cm.config.last_display_active = display_active;
            }
        }
        PersistenceId::TcClimbRate => {
            if let Variant::F32(tc_climb_rate) = variant {
                cm.config.av2_climb_rate_tc = tc_climb_rate;
            }
        }
        PersistenceId::TcSpeedToFly => {
            if let Variant::F32(av_speed_to_fly_tc) = variant {
                cm.config.av_speed_to_fly_tc = av_speed_to_fly_tc;
            }
        }
        PersistenceId::Info1 => {
            if let Variant::I32(info) = variant {
                cm.config.info1 = LineView::from(info as u8);
            }
        }
        PersistenceId::Info2 => {
            if let Variant::I32(info) = variant {
                cm.config.info2 = LineView::from(info as u8);
            }
        }
        PersistenceId::Rotation => {
            if let Variant::Rotation(rotation) = variant {
                cm.control.rotation = rotation;
            }
        }
        PersistenceId::CenterFrequency => {
            if let Variant::F32(frequency) = variant {
                cm.config.snd_center_freq = frequency;
            }
        }
        PersistenceId::CenterViewCircling => {
            if let Variant::I32(view) = variant {
                cm.config.center_circling = CenterView::from(view as u8);
            }
        }
        PersistenceId::CenterViewStraight => {
            if let Variant::I32(view) = variant {
                cm.config.center_straignt = CenterView::from(view as u8);
            }
        }
        PersistenceId::EmptyMass => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.empty_mass = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::MaxBallast => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.max_ballast = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::ReferenceWeight => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.reference_weight = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::PolarValueV1 => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.polar_values[0][0] = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::PolarValueV2 => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.polar_values[1][0] = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::PolarValueV3 => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.polar_values[2][0] = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::PolarValueSi1 => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.polar_values[0][1] = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::PolarValueSi2 => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.polar_values[1][1] = value;
                cc.recalc_glider(cm);
            }
        }
        PersistenceId::PolarValueSi3 => {
            if let Variant::F32(value) = variant {
                cm.glider_data.basic_glider_data.polar_values[2][1] = value;
                cc.recalc_glider(cm);
            }
        }

        _ => (),
    }
    finish_push(cc, cm, id, echo);
}

fn finish_push(cc: &mut CoreController, cm: &mut CoreModel, id: PersistenceId, echo: Echo) {
    if echo == Echo::Nmea || echo == Echo::NmeaAndCan {
        // Buffer NMEA datagrams in IndexSet
        let _ = cc.nmea_vals.insert(id); // send only last content
    }
    if echo == Echo::Can || echo == Echo::NmeaAndCan {
        // Queue directly to canbus
        let frame = cm.can_frame_sys_config(CanConfigId::from(id));
        if let Some(frame) = frame {
            let _ = cc.p_tx_frames.enqueue(frame);
        }
    }
    cc.scheduler
        .after(crate::Timer::PersistSetting, PERSISTENCE_TIMEOUT.millis());
    let _ = cc.pers_vals.insert(id);
}

pub fn send_can_config_frame(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    config_id: CanConfigId,
    get_set: RemoteConfig,
) {
    let frame = cm.can_frame_remote_config(config_id, get_set);
    if let Some(frame) = frame {
        let _ = cc.p_tx_frames.enqueue(frame);
    }
}

pub fn delete_config(cc: &mut CoreController) {
    cc.send_idle_event(IdleEvent::ClearEepromItems(DELETE_CONFIG_LIST));
    cc.send_idle_event(IdleEvent::ResetDevice(ResetReason::ConfigChanged));
}

pub fn factory_reset(cc: &mut CoreController) {
    let item = PersistenceItem::from_i8(PersistenceId::DeleteAll, 0);
    cc.send_idle_event(IdleEvent::SetEepromItem(item));
    cc.send_idle_event(IdleEvent::ResetDevice(ResetReason::ConfigChanged));
}

pub fn user_profile(cc: &mut CoreController, cm: &CoreModel) {
    let item = PersistenceItem::from_u8(PersistenceId::UserProfile, cm.config.user_profile);
    cc.send_idle_event(IdleEvent::SetEepromItem(item));
    cc.send_idle_event(IdleEvent::ResetDevice(ResetReason::ConfigChanged));
}

// This function is called by Timer::PersistSetting after a short period to avoid
// flooting EEPROM and NMEA Interface with to much data
pub fn store_persistence_ids(cm: &mut CoreModel, cc: &mut CoreController) {
    let mut ids = Vec::<PersistenceId, MAX_PERS_IDS>::new();

    // We must first copy the ids into a Vec, because we can't borrow cc twice
    for id in cc.pers_vals.iter() {
        let _ = ids.push(*id);
    }
    cc.pers_vals.clear();
    while let Some(id) = ids.pop() {
        // Store data in EEPROM
        store_item(cc, cm, id);
    }

    ids.clear();
    // We must first copy the ids into a Vec, because we can't borrow cc twice
    for id in cc.nmea_vals.iter() {
        let _ = ids.push(*id);
    }
    cc.nmea_vals.clear();
    while let Some(id) = ids.pop() {
        // Send data via NMEA
        cc.nmea_send_config_data(id);
    }
}
