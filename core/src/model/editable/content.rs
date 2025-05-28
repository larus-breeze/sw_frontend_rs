use super::{
    Content, DisplayActive, Editable, DEFAULT_CONFIG, DO_NOT_CHANGE, FACTORY_RESET, OFF, ON,
    COMMAND_SENT, USER_1, USER_2, USER_3, USER_4,
};
use crate::{
    controller::{persist::send_can_config_frame, CanConfigId, RemoteConfig},
    polar_store,
    utils::TString,
    view::viewable::{centerview::CenterType, lineview::Placement},
    CoreController, CoreModel,
};

impl Editable {
    pub fn content(&self, cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        match self {
            Editable::Bugs => Content::F32(Some((cm.glider_data.bugs - 1.0) * 100.0)),
            Editable::Display => match cm.config.last_display_active {
                DisplayActive::Horizon => Content::Enum(TString::<16>::from_str("Horizon")),
                _ => Content::Enum(TString::<16>::from_str("Vario")),
            },
            Editable::Glider => {
                let sorted_idx = polar_store::to_sorted_idx(cm.config.glider_idx as usize);
                Content::List(sorted_idx as i32)
            }
            Editable::McCready => Content::F32(Some(cm.config.mc_cready.to_m_s())),
            Editable::None => Content::String(TString::<12>::from_str("")),
            Editable::PilotWeight => Content::F32(Some(cm.glider_data.pilot_weight.to_kg())),
            Editable::Return => Content::String(TString::<12>::from_str("")),
            Editable::TcClimbRate => Content::F32(Some(cm.config.av2_climb_rate_tc)),
            Editable::TcSpeedToFly => Content::F32(Some(cm.config.av_speed_to_fly_tc)),
            Editable::Theme => {
                if cm.config.theme == &cm.device_const.dark_theme {
                    Content::Enum(TString::<16>::from_str("Dark"))
                } else {
                    Content::Enum(TString::<16>::from_str("Bright"))
                }
            }
            Editable::VarioModeControl => Content::Enum(TString::<16>::from_str(
                cm.control.vario_mode_control.as_str(),
            )),
            Editable::Volume => Content::F32(Some(cm.config.volume as f32)),
            Editable::AlarmVolume => Content::F32(Some(cm.control.alarm_volume as f32)),
            Editable::WaterBallast => Content::F32(Some(cm.glider_data.water_ballast.to_kg())),
            Editable::Info1 => Content::List(cm.config.info1.sorted_as_i32(Placement::Top)),
            Editable::Info2 => Content::List(cm.config.info2.sorted_as_i32(Placement::Bottom)),
            Editable::Rotation => {
                Content::Enum(TString::<16>::from_str(cm.control.rotation.name()))
            }
            Editable::CenterFrequency => Content::F32(Some(cm.config.snd_center_freq)),
            Editable::CenterViewCircling => Content::List(
                cm.config
                    .center_circling
                    .sorted_as_i32(CenterType::Circling),
            ),
            Editable::CenterViewStraight => Content::List(
                cm.config
                    .center_straignt
                    .sorted_as_i32(CenterType::Straight),
            ),
            Editable::ResetConfig => {
                let s = match cm.control.reset_config {
                    0 => DEFAULT_CONFIG,
                    1 => FACTORY_RESET,
                    _ => DO_NOT_CHANGE,
                };
                Content::Enum(TString::<16>::from_str(s))
            }
            Editable::UserProfile => {
                let s = match cm.config.user_profile {
                    0 => USER_1,
                    1 => USER_2,
                    2 => USER_3,
                    3 => USER_4,
                    _ => DO_NOT_CHANGE,
                };
                Content::Enum(TString::<16>::from_str(s))
            }
            Editable::EmptyMass => Content::F32(Some(cm.glider_data.basic_glider_data.empty_mass)),
            Editable::MaxBallast => {
                Content::F32(Some(cm.glider_data.basic_glider_data.max_ballast))
            }
            Editable::ReferenceWeight => {
                Content::F32(Some(cm.glider_data.basic_glider_data.reference_weight))
            }
            Editable::PolarValueV1 => {
                Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[0][0]))
            }
            Editable::PolarValueV2 => {
                Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[1][0]))
            }
            Editable::PolarValueV3 => {
                Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[2][0]))
            }
            Editable::PolarValueSi1 => {
                Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[0][1]))
            }
            Editable::PolarValueSi2 => {
                Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[1][1]))
            }
            Editable::PolarValueSi3 => {
                Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[2][1]))
            }
            Editable::GliderSymbol => {
                if cm.config.glider_symbol {
                    Content::Enum(TString::<16>::from_str(ON))
                } else {
                    Content::Enum(TString::<16>::from_str(OFF))
                }
            }
            Editable::BatteryGood => Content::F32(Some(cm.config.battery_good)),
            Editable::BatteryLow => Content::F32(Some(cm.config.battery_low)),

            Editable::DrainPinConfig => Content::Enum(TString::<16>::from_str(
                cc.drain_control.pin_function().as_str(),
            )),
            Editable::FlowEmpty => Content::F32(Some(cc.drain_control.flow_rate_offset)),
            Editable::FlowSlope => Content::F32(Some(cc.drain_control.flow_rate_slope)),

            Editable::FlashControl => Content::Enum(TString::<16>::from_str(
                cc.flash_control.pin_function().as_str(),
            )),
            Editable::SpeedToFlyPinConfig => Content::Enum(TString::<16>::from_str(
                cc.speed_to_fly_control.pin_function().as_str(),
            )),
            Editable::GearPinConfig => Content::Enum(TString::<16>::from_str(
                cc.gear_alarm_control.gear_pin_function().as_str(),
            )),
            Editable::AirbrakesPinConfig => Content::Enum(TString::<16>::from_str(
                cc.gear_alarm_control.airbrakes_pin_function().as_str(),
            )),
            Editable::GearAlarmModeConfig => Content::Enum(TString::<16>::from_str(
                cc.gear_alarm_control.gear_pin_mode().as_str(),
            )),
            Editable::StfUpperLimit => Content::F32(Some(cm.config.stf_upper_limit.to_km_h())),
            Editable::StfLowerLimit => Content::F32(Some(cm.config.stf_lower_limit.to_km_h())),

            // Edit sensorbox values via CAN bus
            Editable::SensTiltRoll => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::SensTiltRoll, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::SensTiltPitch => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::SensTiltPitch, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::SensTiltYaw => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::SensTiltYaw, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::PitotOffset => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::PitotOffset, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::PitotSpan => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::PitotSpan, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::QnhDelta => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::QnhDelta, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::MagAutoCalib => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::MagAutoCalib, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::VarioTc => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::VarioTc, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::VarioIntTc => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::VarioIntTc, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::WindTc => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::WindTc, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::MeanWindTc => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::MeanWindTc, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::GnssConfig => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::GnssConfig, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::AntBaselen => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::AntBaselen, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::AntSlaveDown => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::AntSlaveDown, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::AntSlaveRight => {
                // We have ask sensorbox for values and have no value at the moment
                send_can_config_frame(cm, cc, CanConfigId::AntSlaveRight, RemoteConfig::Get);
                Content::F32(None)
            }
            Editable::CmdMeas1
            | Editable::CmdMeas2
            | Editable::CmdMeas3
            | Editable::CmdCalcOrientation
            | Editable::CmdFineTuneOrientation 
            | Editable::CmdResetSensorbox => {
                Content::Command(TString::<16>::from_str(COMMAND_SENT))
            }
        }
    }
}
