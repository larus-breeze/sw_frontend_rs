use super::{
    DisplayActive, Editable, VarioModeControl, DEFAULT_CONFIG, DO_NOT_CHANGE, FACTORY_RESET, ON,
    USER_1, USER_2, USER_3, USER_4,
};
use crate::{
    controller::persist,
    flight_physics::polar_store,
    persist::send_can_config_frame,
    utils::{TString, Variant},
    view::viewable::{
        centerview::{CenterType, CenterView},
        lineview::{LineView, Placement},
    },
    CoreController, CoreModel, Echo, FloatToMass, FloatToSpeed, GearPins, InPinFunction,
    InTogglePinFunction, OutPinFunction, PersistenceId, RemoteConfig, Rotation,
};

impl Editable {
    pub fn set_enum_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: &TString<16>) {
        match self {
            Editable::Display => match val.as_str() {
                "Horizon" => persist::persist_set(
                    cc,
                    cm,
                    Variant::DisplayActive(DisplayActive::Horizon),
                    PersistenceId::Display,
                    Echo::None,
                ),
                _ => persist::persist_set(
                    cc,
                    cm,
                    Variant::DisplayActive(DisplayActive::Vario),
                    PersistenceId::Display,
                    Echo::None,
                ),
            },
            Editable::Theme => persist::persist_set(
                cc,
                cm,
                Variant::Str(val.as_str()),
                PersistenceId::DisplayTheme,
                Echo::None,
            ),
            Editable::VarioModeControl => {
                let mode = VarioModeControl::from(val.as_str());
                persist::persist_set(
                    cc,
                    cm,
                    Variant::VarioModeControl(mode),
                    PersistenceId::VarioModeControl,
                    Echo::NmeaAndCan,
                );
            }
            Editable::Rotation => persist::persist_set(
                cc,
                cm,
                Variant::Rotation(Rotation::from(val.as_str())),
                PersistenceId::Rotation,
                Echo::None,
            ),
            Editable::ResetConfig => {
                cm.control.reset_config = match val.as_str() {
                    DEFAULT_CONFIG => 0,
                    FACTORY_RESET => 1,
                    _ => 2,
                };
                if cm.control.editor.enter_pushed {
                    match cm.control.reset_config {
                        0 => persist::delete_config(cc),
                        1 => persist::factory_reset(cc),
                        _ => (),
                    }
                }
            }
            Editable::UserProfile => {
                if cm.control.editor.enter_pushed && val.as_str() != DO_NOT_CHANGE {
                    cm.config.user_profile = match val.as_str() {
                        USER_1 => 0,
                        USER_2 => 1,
                        USER_3 => 2,
                        USER_4 => 3,
                        _ => 0,
                    };
                    persist::user_profile(cc, cm); // store value and reset device
                }
            }
            Editable::GliderSymbol => persist::persist_set(
                cc,
                cm,
                Variant::Bool(val.as_str() == ON),
                PersistenceId::GliderSymbol,
                Echo::None,
            ),
            Editable::DrainPinConfig => persist::persist_set(
                cc,
                cm,
                Variant::U8(InPinFunction::from(val.as_str()) as u8),
                PersistenceId::DrainPinConfig,
                Echo::None,
            ),
            Editable::GearAlarmModeConfig => persist::persist_set(
                cc,
                cm,
                Variant::U8(GearPins::from(val.as_str()) as u8),
                PersistenceId::GearAlarmMode,
                Echo::None,
            ),
            Editable::GearPinConfig => persist::persist_set(
                cc,
                cm,
                Variant::U8(InPinFunction::from(val.as_str()) as u8),
                PersistenceId::GearPinConfig,
                Echo::None,
            ),
            Editable::AirbrakesPinConfig => persist::persist_set(
                cc,
                cm,
                Variant::U8(InPinFunction::from(val.as_str()) as u8),
                PersistenceId::AirbrakesPinConfig,
                Echo::None,
            ),
            Editable::FlashControl => persist::persist_set(
                cc,
                cm,
                Variant::U8(OutPinFunction::from(val.as_str()) as u8),
                PersistenceId::FlashControl,
                Echo::None,
            ),
            Editable::SpeedToFlyPinConfig => persist::persist_set(
                cc,
                cm,
                Variant::U8(InTogglePinFunction::from(val.as_str()) as u8),
                PersistenceId::SpeedToFlyPinConfig,
                Echo::None,
            ),
            _ => (),
        }
    }


    pub fn set_cmd_content(&self, cm: &mut CoreModel, cc: &mut CoreController) {
        match self {
            Editable::CmdMeas1 => {
                send_can_config_frame(
                    cm,
                    cc,
                    crate::CanConfigId::CmdMeasure1,
                    RemoteConfig::Get,
                );
            }
            Editable::CmdMeas2 => {
                send_can_config_frame(
                    cm,
                    cc,
                    crate::CanConfigId::CmdMeasure2,
                    RemoteConfig::Get,
                );
            }
            Editable::CmdMeas3 => {
                send_can_config_frame(
                    cm,
                    cc,
                    crate::CanConfigId::CmdMeasure3,
                    RemoteConfig::Get,
                );
            }
            Editable::CmdCalcOrientation => {
                send_can_config_frame(
                    cm,
                    cc,
                    crate::CanConfigId::CmdCalcSensorOrientation,
                    RemoteConfig::Get,
                );
            }
            Editable::CmdFineTuneOrientation => {
                send_can_config_frame(
                    cm,
                    cc,
                    crate::CanConfigId::CmdFineTuneCalibration,
                    RemoteConfig::Get,
                );
            }
            Editable::CmdResetSensorbox => {
                send_can_config_frame(
                    cm,
                    cc,
                    crate::CanConfigId::CmdReset,
                    RemoteConfig::Get,
                );
            }
            _ => (),
        }
    }

    pub fn set_f32_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: f32) {
        match self {
            Editable::Bugs => persist::persist_set(
                cc,
                cm,
                Variant::F32(1.0 + val / 100.0),
                PersistenceId::Bugs,
                Echo::NmeaAndCan,
            ),
            Editable::McCready => persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.m_s()),
                PersistenceId::McCready,
                Echo::NmeaAndCan,
            ),
            Editable::PilotWeight => persist::persist_set(
                cc,
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::PilotWeight,
                Echo::NmeaAndCan,
            ),
            Editable::TcClimbRate => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::TcClimbRate,
                Echo::Can,
            ),
            Editable::TcSpeedToFly => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::TcSpeedToFly,
                Echo::Can,
            ),
            Editable::Volume => persist::persist_set(
                cc,
                cm,
                Variant::I8(val as i8),
                PersistenceId::Volume,
                Echo::NmeaAndCan,
            ),
            Editable::AlarmVolume => persist::persist_set(
                cc,
                cm,
                Variant::I8(val as i8),
                PersistenceId::AlarmVolume,
                Echo::None,
            ),
            Editable::WaterBallast => persist::persist_set(
                cc,
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::WaterBallast,
                Echo::NmeaAndCan,
            ),
            Editable::CenterFrequency => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::CenterFrequency,
                Echo::Can,
            ),
            Editable::EmptyMass => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::EmptyMass,
                Echo::Can,
            ),
            Editable::MaxBallast => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::MaxBallast,
                Echo::Can,
            ),
            Editable::ReferenceWeight => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::ReferenceWeight,
                Echo::Can,
            ),
            Editable::PolarValueV1 => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueV1,
                Echo::Can,
            ),
            Editable::PolarValueV2 => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueV2,
                Echo::Can,
            ),
            Editable::PolarValueV3 => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueV3,
                Echo::Can,
            ),
            Editable::PolarValueSi1 => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueSi1,
                Echo::Can,
            ),
            Editable::PolarValueSi2 => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueSi2,
                Echo::Can,
            ),
            Editable::PolarValueSi3 => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueSi3,
                Echo::Can,
            ),
            Editable::BatteryGood => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::BatteryGood,
                Echo::None,
            ),
            Editable::BatteryLow => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::BatteryLow,
                Echo::None,
            ),
            Editable::FlowEmpty => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::FlowEmpty,
                Echo::None,
            ),
            Editable::FlowSlope => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::FlowSlope,
                Echo::None,
            ),
            Editable::StfUpperLimit => persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.km_h()),
                PersistenceId::StfUpperLimit,
                Echo::None,
            ),
            Editable::StfLowerLimit => persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.km_h()),
                PersistenceId::StfLowerLimit,
                Echo::None,
            ),

            Editable::SensTiltRoll => {
                send_can_config_frame(cm, cc, crate::CanConfigId::SensTiltRoll, RemoteConfig::Set);
            }
            Editable::SensTiltPitch => {
                send_can_config_frame(cm, cc, crate::CanConfigId::SensTiltPitch, RemoteConfig::Set);
            }
            Editable::SensTiltYaw => {
                send_can_config_frame(cm, cc, crate::CanConfigId::SensTiltYaw, RemoteConfig::Set);
            }
            Editable::PitotOffset => {
                send_can_config_frame(cm, cc, crate::CanConfigId::PitotOffset, RemoteConfig::Set);
            }
            Editable::PitotSpan => {
                send_can_config_frame(cm, cc, crate::CanConfigId::PitotSpan, RemoteConfig::Set);
            }
            Editable::QnhDelta => {
                send_can_config_frame(cm, cc, crate::CanConfigId::QnhDelta, RemoteConfig::Set);
            }
            Editable::MagAutoCalib => {
                send_can_config_frame(cm, cc, crate::CanConfigId::MagAutoCalib, RemoteConfig::Set);
            }
            Editable::VarioTc => {
                send_can_config_frame(cm, cc, crate::CanConfigId::VarioTc, RemoteConfig::Set);
            }
            Editable::VarioIntTc => {
                send_can_config_frame(cm, cc, crate::CanConfigId::VarioIntTc, RemoteConfig::Set);
            }
            Editable::WindTc => {
                send_can_config_frame(cm, cc, crate::CanConfigId::WindTc, RemoteConfig::Set);
            }
            Editable::MeanWindTc => {
                send_can_config_frame(cm, cc, crate::CanConfigId::MeanWindTc, RemoteConfig::Set);
            }
            Editable::GnssConfig => {
                send_can_config_frame(cm, cc, crate::CanConfigId::GnssConfig, RemoteConfig::Set);
            }
            Editable::AntBaselen => {
                send_can_config_frame(cm, cc, crate::CanConfigId::AntBaselen, RemoteConfig::Set);
            }
            Editable::AntSlaveDown => {
                send_can_config_frame(cm, cc, crate::CanConfigId::AntSlaveDown, RemoteConfig::Set);
            }
            Editable::AntSlaveRight => {
                send_can_config_frame(cm, cc, crate::CanConfigId::AntSlaveRight, RemoteConfig::Set);
            }
            _ => (),
        }
    }

    #[allow(clippy::single_match)]
    pub fn set_list_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: i32) {
        match self {
            Editable::Glider => {
                let raw_idx = polar_store::to_raw_idx(val as usize);
                persist::persist_set(
                    cc,
                    cm,
                    Variant::Usize(raw_idx),
                    PersistenceId::Glider,
                    Echo::None,
                )
            }
            Editable::Info1 => {
                let variant = LineView::from_sorted(val as usize, Placement::Top) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::Info1,
                    Echo::None,
                )
            }
            Editable::Info2 => {
                let variant = LineView::from_sorted(val as usize, Placement::Bottom) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::Info2,
                    Echo::None,
                )
            }
            Editable::CenterViewCircling => {
                let variant = CenterView::from_sorted(val as usize, CenterType::Circling) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::CenterViewCircling,
                    Echo::None,
                )
            }
            Editable::CenterViewStraight => {
                let variant = CenterView::from_sorted(val as usize, CenterType::Straight) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::CenterViewStraight,
                    Echo::None,
                )
            }
            _ => (),
        }
    }
}
