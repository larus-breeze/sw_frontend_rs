/// The Persistence Layer stores Data in EEPROM an distributes it to NMEA and Can Bus interfaces
///
/// Data points that can be processed must be recorded by the PersistenceId. The
/// restore_item() method writes the data read from the EEPROM to the CoreModel. 
///
/// The persist_set() method receives data from the NMEA and CAN bus interfaces and from the editor,
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

use super::{
    helpers::{GearPins, InPinFunction, InTogglePinFunction, OutPinFunction},
    DataSource, VarioModeControl, MAX_PERS_IDS,
};
use crate::{
    basic_config::PERSISTENCE_TIMEOUT,
    controller::{
        helpers::{CanConfigId, IntToDuration},
        RemoteConfig,
    },
    flight_physics::polar_store,
    model::{UnitHorizontalSpeed, UnitVerticalSpeed, UnitHeight},
    system_of_units::Speed,
    utils::Variant,
    view::viewable::{centerview::CenterView, vario_infoview::{LineView, Info3View}},
    CoreController, CoreModel, FloatToSpeed, IdleEvent, Mass, PersistenceItem, Pressure,
    ResetReason, Rotation, VarioMode,
};

/// It is not permitted to change the sequence or assignment, as the number references the memory 
/// location in the EEPROM. New memory data may only ever be inserted before the LastItem.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, defmt::Format)]
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
    Info1Vario = 12,
    Info2Vario = 13,
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
    GliderSymbol = 27,
    BatteryGood = 28,
    BatteryLow = 29,
    DrainPinConfig = 30,
    FlowEmpty = 31,
    FlowSlope = 32,
    FlashControl = 33,
    SpeedToFlyPinConfig = 34,
    GearPinConfig = 35,
    AirbrakesPinConfig = 36,
    GearAlarmMode = 37,
    AlarmVolume = 38,
    StfUpperLimit = 39,
    StfLowerLimit = 40,
    AvgClimbeRateSrc = 41,
    StfClimbrateAlt = 42,   // no longer in use
    TcCircleHysteresis = 43,
    EnergyArrowMult = 44,
    VarioUpperLimit = 45,
    VarioLowerLimit = 46,
    Info1Stf = 47,
    Info2Stf = 48,
    Info3Vario = 49,
    Info3Stf = 50,
    UnitHorizontalSpeed = 51,
    UnitVerticalSpeed = 52,
    UnitHeight = 53,
    LastItem = 54, // Items smaller than this are stored in eeprom

    // Special function Ids
    VarioMode = 65532,
    UserProfile = 65533, 
    DeleteAll = 65534,
    #[default]
    DoNotStore = 65535,
}

/// This list defines which Ids are stored only in user profile 0
const PROFILE_0_IDS: &[PersistenceId] = &[
    PersistenceId::Glider,
    PersistenceId::EmptyMass,
    PersistenceId::MaxBallast,
    PersistenceId::ReferenceWeight,
    PersistenceId::PolarValueV1,
    PersistenceId::PolarValueV2,
    PersistenceId::PolarValueV3,
    PersistenceId::PolarValueSi1,
    PersistenceId::PolarValueSi2,
    PersistenceId::PolarValueSi3,
    PersistenceId::DrainPinConfig,
    PersistenceId::FlowEmpty,
    PersistenceId::FlowSlope,
    PersistenceId::FlashControl,
    PersistenceId::SpeedToFlyPinConfig,
    PersistenceId::GearPinConfig,
    PersistenceId::AirbrakesPinConfig,
    PersistenceId::GearAlarmMode,
];

/// This list defines which data is destroyed when a profile is deleted
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
    PersistenceId::Info1Vario,
    PersistenceId::Info2Vario,
    PersistenceId::Rotation,
    PersistenceId::CenterFrequency,
    PersistenceId::CenterViewCircling,
    PersistenceId::CenterViewStraight,
    PersistenceId::GliderSymbol,
    PersistenceId::BatteryGood,
    PersistenceId::BatteryLow,
    PersistenceId::FlowEmpty,
    PersistenceId::FlowSlope,
    PersistenceId::AlarmVolume,
    PersistenceId::StfUpperLimit,
    PersistenceId::StfLowerLimit,
    PersistenceId::AvgClimbeRateSrc,
    PersistenceId::StfClimbrateAlt,
    PersistenceId::TcCircleHysteresis,
    PersistenceId::EnergyArrowMult,
    PersistenceId::Info1Stf,
    PersistenceId::Info2Stf,
    PersistenceId::Info3Vario,
    PersistenceId::Info3Stf,
];

/// The following data is deleted when a new glider is selected
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

/// Check if PersistenceId is in the list of profile 0
pub fn profile_always_0(id: PersistenceId) -> bool {
    let mut is_unique = false;
    for iter_id in PROFILE_0_IDS {
        if *iter_id == id {
            is_unique = true;
            break;
        }
    }
    is_unique
}

#[derive(PartialEq)]
pub enum Echo {
    None,
    Nmea,
    Can,
    NmeaAndCan,
}

/// Store item content into data model
///
/// This method is also called directly from the idle-loop during start-up
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
        PersistenceId::Bugs => cm.glider_data.set_bugs(item.to_f32()),
        PersistenceId::Display => cm.config.display_active = item.to_u8().into(),
        PersistenceId::TcClimbRate => cm.config.av2_climb_rate_tc = item.to_f32(),
        PersistenceId::TcSpeedToFly => cm.config.av_speed_to_fly_tc = item.to_f32(),
        PersistenceId::Info1Vario => cm.config.info1_vario = LineView::from(item.to_u8()),
        PersistenceId::Info2Vario => cm.config.info2_vario = LineView::from(item.to_u8()),
        PersistenceId::Rotation => cm.control.rotation = Rotation::from(item.to_u8()),
        PersistenceId::CenterFrequency => cm.config.snd_center_freq = item.to_f32(),
        PersistenceId::CenterViewCircling => {
            cm.config.center_circling = CenterView::from(item.to_u8())
        }
        PersistenceId::CenterViewStraight => {
            cm.config.center_straight = CenterView::from(item.to_u8())
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
        PersistenceId::GliderSymbol => cm.config.glider_symbol = item.to_bool(),
        PersistenceId::BatteryGood => cm.config.battery_good = item.to_f32(),
        PersistenceId::BatteryLow => cm.config.battery_low = item.to_f32(),
        PersistenceId::DrainPinConfig => cc
            .drain_control
            .set_pin_function(InPinFunction::from(item.to_u8()), cm),
        PersistenceId::FlowEmpty => cc.drain_control.flow_rate_offset = item.to_f32(),
        PersistenceId::FlowSlope => cc.drain_control.flow_rate_slope = item.to_f32(),
        PersistenceId::FlashControl => cc
            .flash_control
            .set_pin_function(OutPinFunction::from(item.to_u8())),
        PersistenceId::SpeedToFlyPinConfig => cc
            .speed_to_fly_control
            .set_pin_function(InTogglePinFunction::from(item.to_u8())),
        PersistenceId::GearPinConfig => cc
            .gear_alarm_control
            .set_gear_pin_function(InPinFunction::from(item.to_u8())),
        PersistenceId::AirbrakesPinConfig => cc
            .gear_alarm_control
            .set_airbrakes_pin_function(InPinFunction::from(item.to_u8())),
        PersistenceId::GearAlarmMode => cc
            .gear_alarm_control
            .set_gear_pin_mode(GearPins::from(item.to_u8())),
        PersistenceId::AlarmVolume => cm.control.alarm_volume = item.to_i8(),
        PersistenceId::StfUpperLimit => cm.config.stf_upper_limit = item.to_f32().m_s(),
        PersistenceId::StfLowerLimit => cm.config.stf_lower_limit = item.to_f32().m_s(),
        PersistenceId::AvgClimbeRateSrc => {
            cm.control.avg_climb_rate_src = DataSource::from(item.to_u8())
        }
        PersistenceId::StfClimbrateAlt => (),
        PersistenceId::TcCircleHysteresis => cm.config.circle_hysteresis_tc = item.to_i8(),
        PersistenceId::EnergyArrowMult => cm.control.energy_arrow_mult = item.to_f32(),
        PersistenceId::VarioUpperLimit => cm.config.vario_upper_limit = item.to_f32().m_s(),
        PersistenceId::VarioLowerLimit => cm.config.vario_lower_limit = item.to_f32().m_s(),
        PersistenceId::Info1Stf => cm.config.info1_stf = LineView::from(item.to_u8()),
        PersistenceId::Info2Stf => cm.config.info2_stf = LineView::from(item.to_u8()),
        PersistenceId::Info3Vario => cm.config.info3_vario = Info3View::from(item.to_u8()),
        PersistenceId::Info3Stf => cm.config.info3_stf = Info3View::from(item.to_u8()),
        PersistenceId::UnitHorizontalSpeed => cm.config.unit_horizontal_spped = UnitHorizontalSpeed::from(item.to_u8()),
        PersistenceId::UnitVerticalSpeed => cm.config.unit_vertical_spped = UnitVerticalSpeed::from(item.to_u8()),
        PersistenceId::UnitHeight => cm.config.unit_height = UnitHeight::from(item.to_u8()),

        PersistenceId::VarioMode => cm.control.vario_mode = VarioMode::from(item.to_u8()),

        PersistenceId::DeleteAll => (),
        PersistenceId::DoNotStore => (),
        PersistenceId::LastItem => (),
    }
}

pub fn persist_set(
    cc: &mut CoreController,
    cm: &mut CoreModel,
    variant: Variant,
    id: PersistenceId,
    echo: Echo,
) {
    let item = PersistenceItem::from_variant(id, variant);
    restore_item(cc, cm, item);

    if id == PersistenceId::Glider {
        // When we choose a new glider polar, these settings are no longer usefull
        cc.send_idle_event(IdleEvent::ClearEepromItems(SPECIFIC_POLAR_SETTINGS));
    }

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
        // The sensorbox needs fraction of water ballast to be able to generate the NMEA sentence
        if id == PersistenceId::WaterBallast {
            let frame = cm.can_frame_sys_config(CanConfigId::WaterBallastFraction).unwrap();
            let _ = cc.p_tx_frames.enqueue(frame);
        }
    }

    if(id as u16) < (PersistenceId::LastItem as u16) {
        let _ = cc.pers_vals.insert(id, item); // Buffer item to write it to EEPROM
    }

    cc.scheduler
        .after(crate::Timer::PersistSetting, PERSISTENCE_TIMEOUT.millis());

}

pub fn send_can_config_frame(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    config_id: CanConfigId,
    get_set: RemoteConfig,
) {
    // get command is sent immidiatly, set command after a while, if no new changes are made
    match get_set {
        RemoteConfig::Get => {
            let frame = cm.can_frame_remote_config(config_id, get_set);
            if let Some(frame) = frame {
                let _ = cc.p_tx_frames.enqueue(frame);
            }
        }
        RemoteConfig::Set => {
            cc.remote_val = Some((config_id, get_set));
            cc.scheduler
                .after(crate::Timer::PersistSetting, PERSISTENCE_TIMEOUT.millis());
        }
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
    let mut items = Vec::<PersistenceItem, MAX_PERS_IDS>::new();

    // We must first copy the ids into a Vec, because we can't borrow cc twice
    for (_id, item) in cc.pers_vals.iter() {
        let _ = items.push(*item);
    }
    cc.pers_vals.clear();
    while let Some(item) = items.pop() {
        // Store data in EEPROM
        cc.send_idle_event(IdleEvent::SetEepromItem(item));
    }

    let mut ids = Vec::<PersistenceId, MAX_PERS_IDS>::new();
    // We must first copy the ids into a Vec, because we can't borrow cc twice
    for id in cc.nmea_vals.iter() {
        let _ = ids.push(*id);
    }
    cc.nmea_vals.clear();
    while let Some(id) = ids.pop() {
        // Send data via NMEA
        cc.nmea_send_config_data(id);
    }

    // send remote value if necessary
    if let Some((config_id, get_set)) = cc.remote_val {
        let frame = cm.can_frame_remote_config(config_id, get_set);
        if let Some(frame) = frame {
            let _ = cc.p_tx_frames.enqueue(frame);
        }
        cc.remote_val = None;
    }

    // We don't know, if someone has changed glider polar settings
    cc.recalc_glider(cm);
}

pub fn set_vario_mode(cm: &mut CoreModel, cc: &mut CoreController, vario_mode: VarioMode, source: VarioModeControl) {
    let vario_mode = if source == cm.control.vario_mode_control {
        vario_mode
    } else if source == VarioModeControl::Can && cm.control.vario_mode_control == VarioModeControl::Nmea {
        vario_mode
    } else {
        cm.control.vario_mode
    };

    if vario_mode != cm.control.vario_mode {
        let echo = match source {
            VarioModeControl::Auto | VarioModeControl::InputPin => Echo::NmeaAndCan,
            VarioModeControl::Nmea => Echo::Can,
            VarioModeControl::Can => Echo::Nmea,
        };
        persist_set(cc, cm, Variant::U8(vario_mode as u8), PersistenceId::VarioMode, echo);
    }
}