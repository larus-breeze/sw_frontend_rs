/// Elements that can be changed by the user
///
/// Editables are always saved in the model and can be changed by the user. These can be
/// parameters, display selection, time constants or other data. With the help of this module,
/// the implemented editor is able to display and change such data, save it and, if necessary,
/// output it at the NMEA and CAN interfaces.
///
/// New elements are added in the following steps:
///   - First, the persistence layer is extended (src/controller/persistence.rs)
///     - Extend PersistenceId
///     - Extend persist_restore_item(), persist_store_item() and persist_set_id()
///   - Then the enum Editable is extended by the new element (see below)
///   - Extend necessary mehtods params(), name(), content(), set_...()
mod params;
mod set_content;

use crate::{
    controller::{persist::send_can_config_frame, CanConfigId, RemoteConfig}, model::VarioModeControl, polar_store, utils::TString, 
    view::viewable::{
        centerview::{CenterType, CenterView},
        lineview::{LineView, Placement},
    }, CoreController, CoreModel
};

use super::DisplayActive;
pub(crate) use params::{EnumParams, F32Params, ListParams, Params, MAX_ENUM_VARIANTS};
use tfmt::Convert;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Editable {
    Bugs,
    Display,
    Glider,
    McCready,
    None,
    PilotWeight,
    Return,
    TcClimbRate,
    TcSpeedToFly,
    Theme,
    VarioModeControl,
    Volume,
    WaterBallast,
    Info1,
    Info2,
    Rotation,
    CenterFrequency,
    CenterViewCircling,
    CenterViewStraight,
    ResetConfig,
    UserProfile,
    EmptyMass,
    MaxBallast,
    ReferenceWeight,
    PolarValueV1,
    PolarValueV2,
    PolarValueV3,
    PolarValueSi1,
    PolarValueSi2,
    PolarValueSi3,

    SensTiltRoll, // These are sensorbox settings
    SensTiltPitch,
    SensTiltYaw,
    PitotOffset,
    PitotSpan,
    QnhDelta,
    MagAutoCalib,
    VarioTc,
    VarioIntTc,
    WindTc,
    MeanWindTc,
    GnssConfig,
    AntBaselen,
    AntSlaveDown,
    AntSlaveRight,
}

const DEFAULT_CONFIG: &str = "Default Config";
const FACTORY_RESET: &str = "Factory Reset";
const DO_NOT_CHANGE: &str = "Do not change";

const USER_1: &str = "User 1";
const USER_2: &str = "User 2";
const USER_3: &str = "User 3";
const USER_4: &str = "User 4";

#[derive(Clone, Copy)]
pub enum Content {
    F32(Option<f32>),
    Enum(TString<16>),
    String(TString<12>),
    List(i32),
}

impl Editable {
    pub fn name(&self) -> &'static str {
        match self {
            Editable::Bugs => "Bugs",
            Editable::Display => "Display",
            Editable::Glider => "Glider",
            Editable::McCready => "Mac Cready",
            Editable::None => "None",
            Editable::PilotWeight => "Pilot Weight",
            Editable::Return => "Return",
            Editable::TcClimbRate => "TC Climb Rate",
            Editable::TcSpeedToFly => "TC Speed to Fly",
            Editable::Theme => "Theme",
            Editable::VarioModeControl => "Vario Control",
            Editable::Volume => "Volume",
            Editable::WaterBallast => "Water Ballast",
            Editable::Info1 => "Info 1 Content",
            Editable::Info2 => "Info 2 Content",
            Editable::Rotation => "Display Rotation",
            Editable::CenterFrequency => "Center Frequency",
            Editable::CenterViewCircling => "Center Circling",
            Editable::CenterViewStraight => "Center Straight",
            Editable::ResetConfig => "Configuration",
            Editable::UserProfile => "User Profile",
            Editable::EmptyMass => "Empty Mass",
            Editable::MaxBallast => "Max Ballast",
            Editable::ReferenceWeight => "Reference Weight",
            Editable::PolarValueV1 => "Polar V 1",
            Editable::PolarValueV2 => "Polar V 2",
            Editable::PolarValueV3 => "Polar V 3",
            Editable::PolarValueSi1 => "Polar Si 1",
            Editable::PolarValueSi2 => "Polar Si 2",
            Editable::PolarValueSi3 => "Polar Si 3",

            Editable::SensTiltRoll => "Sensor Tilt Roll",
            Editable::SensTiltPitch => "Sensor Tilt Pitch",
            Editable::SensTiltYaw => "Sensor Tilt Yaw",
            Editable::PitotOffset => "Pitot Offset",
            Editable::PitotSpan => "Pitot Span",
            Editable::QnhDelta => "QNH Delta",
            Editable::MagAutoCalib => "Mag Auto Calib",
            Editable::VarioTc => "Vario TC",
            Editable::VarioIntTc => "Vario Int TC",
            Editable::WindTc => "Wind TC",
            Editable::MeanWindTc => "Mean Wind TC",
            Editable::GnssConfig => "GNSS Config",
            Editable::AntBaselen => "Ant Base Len",
            Editable::AntSlaveDown => "Ant Slave Down",
            Editable::AntSlaveRight => "Ant Slave Right",
        
        }
    }

    pub fn value_as_str(&self, content: Content) -> TString<20> {
        let mut conv = Convert::<20>::new(b' ');
        let params = self.params();

        match params {
            Params::Enum(_params) => {
                if let Content::Enum(val) = content {
                    conv.write_str(val.as_str()).unwrap();
                }
            }
            Params::F32(params) => {
                if let Content::F32(opt_val) = content {
                    conv.write_str(params.unit).unwrap();
                    conv.write_u8(b' ').unwrap();
                    if let Some(val) = opt_val {
                        conv.f32(val, params.dec_places as usize).unwrap();
                    }
                }
            }
            Params::List(_params) => {
                if let Content::List(val) = content {
                    match self {
                        Editable::Glider => {
                            let raw_idx = polar_store::to_raw_idx(val as usize);
                            let name = polar_store::from_raw_idx(raw_idx).name;
                            conv.write_str(name).unwrap()
                        }
                        Editable::Info1 => conv
                            .write_str(LineView::from_sorted(val as usize, Placement::Top).name())
                            .unwrap(),
                        Editable::Info2 => conv
                            .write_str(
                                LineView::from_sorted(val as usize, Placement::Bottom).name(),
                            )
                            .unwrap(),
                        Editable::CenterViewCircling => conv
                            .write_str(
                                CenterView::from_sorted(val as usize, CenterType::Circling).name(),
                            )
                            .unwrap(),
                        Editable::CenterViewStraight => conv
                            .write_str(
                                CenterView::from_sorted(val as usize, CenterType::Straight).name(),
                            )
                            .unwrap(),
                        _ => (),
                    }
                }
            }
            Params::String(_params) => {
                if let Content::String(val) = content {
                    conv.write_str(val.as_str()).unwrap();
                }
            }
        }
        TString::<20>::from_str(conv.as_str())
    }

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
            Editable::VarioModeControl => match cm.control.vario_mode_control {
                VarioModeControl::Auto => Content::Enum(TString::<16>::from_str("Auto")),
                VarioModeControl::Vario => Content::Enum(TString::<16>::from_str("Vario")),
                VarioModeControl::SpeedToFly => {
                    Content::Enum(TString::<16>::from_str("SpeedToFly"))
                }
            },
            Editable::Volume => Content::F32(Some(cm.config.volume as f32)),
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
            Editable::MaxBallast => Content::F32(Some(cm.glider_data.basic_glider_data.max_ballast)),
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
        }
    }
}
