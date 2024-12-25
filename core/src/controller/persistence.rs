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
use crate::{utils::Variant, view::viewable::*, Rotation};

use heapless::Vec;

use super::{VarioModeControl, MAX_PERS_IDS};
use crate::{
    basic_config::PERSISTENCE_TIMEOUT,
    controller::helpers::{CanConfigId, IntToDuration},
    eeprom,
    system_of_units::Speed,
    CoreController, CoreModel, Mass, PersistenceItem, Pressure,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PersistenceId {
    DoNotStore = 65535,
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
    LastItem,
}

impl From<u16> for PersistenceId {
    fn from(src: u16) -> Self {
        if src < eeprom::MAX_ITEM_COUNT as u16 && src < PersistenceId::LastItem as u16 {
            // Safety: Only valid or possible values are transmuted
            unsafe { core::mem::transmute::<u16, PersistenceId>(src) }
        } else {
            panic!()
        }
    }
}

#[derive(PartialEq)]
pub enum Echo {
    None,
    Nmea,
    Can,
    NmeaAndCan,
}

impl CoreController {
    //// Restore Items from EEPROM
    ///
    /// This method is called directly from the idle-loop during start-up
    pub fn persist_restore_item(&mut self, cm: &mut CoreModel, item: PersistenceItem) {
        match item.id {
            PersistenceId::Volume => cm.config.volume = item.to_i8(),
            PersistenceId::McCready => cm.config.mc_cready = Speed::from_m_s(item.to_f32()),
            PersistenceId::WaterBallast => {
                cm.glider_data.water_ballast = Mass::from_kg(item.to_f32())
            }
            PersistenceId::PilotWeight => {
                cm.glider_data.pilot_weight = Mass::from_kg(item.to_f32())
            }
            PersistenceId::Glider => cm.config.glider_idx = item.to_i32(),
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
            PersistenceId::Info1 => cm.config.info1_content = LineView::from(item.to_u32()),
            PersistenceId::Info2 => cm.config.info2_content = LineView::from(item.to_u32()),
            PersistenceId::Rotation => cm.control.rotation = Rotation::from(item.to_u32()),
            PersistenceId::CenterFrequency => cm.config.snd_center_freq = item.to_f32(),
            _ => (),
        }
    }

    /// Store Content of PersistenceId in EEPROM
    ///
    /// This method pushs the content into a queue, which is connected to the hardware
    pub fn persist_store_item(&mut self, cm: &mut CoreModel, id: PersistenceId) {
        let p_item = match id {
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
            PersistenceId::Display => {
                PersistenceItem::from_u8(id, cm.config.last_display_active as u8)
            }
            PersistenceId::TcClimbRate => {
                PersistenceItem::from_f32(id, cm.config.av2_climb_rate_tc)
            }
            PersistenceId::TcSpeedToFly => {
                PersistenceItem::from_f32(id, cm.config.av_speed_to_fly_tc)
            }
            PersistenceId::Info1 => PersistenceItem::from_u32(id, cm.config.info1_content as u32),
            PersistenceId::Info2 => PersistenceItem::from_u32(id, cm.config.info2_content as u32),
            PersistenceId::Rotation => PersistenceItem::from_u32(id, cm.control.rotation as u32),
            PersistenceId::CenterFrequency => PersistenceItem::from_f32(id, cm.config.snd_center_freq),
            _ => PersistenceItem::do_not_store(),
        };
        self.send_idle_event(crate::IdleEvent::EepromItem(p_item));
    }

    pub fn persist_set(
        &mut self,
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
                if let Variant::I32(glider_idx) = variant {
                    cm.config.glider_idx = glider_idx;
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
                    cm.config.info1_content = LineView::from(info as u32);
                }
            }
            PersistenceId::Info2 => {
                if let Variant::I32(info) = variant {
                    cm.config.info2_content = LineView::from(info as u32);
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
            _ => (),
        }
        self.persist_finish_push(cm, id, echo);
    }

    fn persist_finish_push(&mut self, cm: &mut CoreModel, id: PersistenceId, echo: Echo) {
        if echo == Echo::Nmea || echo == Echo::NmeaAndCan {
            // Buffer NMEA datagrams in IndexSet
            let _ = self.nmea_vals.insert(id); // send only last content
        }
        if echo == Echo::Can || echo == Echo::NmeaAndCan {
            // Queue directly to canbus
            if let Some(frame) = cm.can_frame_sys_config(CanConfigId::from(id)) {
                let _ = self.p_tx_frames.enqueue(frame);
            }
        }
        self.scheduler
            .after(crate::Timer::PersistSetting, PERSISTENCE_TIMEOUT.millis());
        let _ = self.pers_vals.insert(id);
    }
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
        cc.persist_store_item(cm, id);
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
