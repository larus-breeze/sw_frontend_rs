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
use crate::{
    model::VarioModeControl,
    utils::{TString, Variant},
    view::viewable::{centerview::{CenterType, CenterView}, lineview::{LineView, Placement}},
    CoreController, CoreModel, Echo, FloatToMass, FloatToSpeed, PersistenceId, Polar, Rotation,
    POLARS, POLAR_COUNT,
};

use super::DisplayActive;
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
}


const DEFAULT_CONFIG: &str = "Default Config";
const FACTORY_RESET: &str = "Factory Reset";
const DO_NOT_CHANGE: &str = "Do not change";

const USER_1: &str = "User 1";
const USER_2: &str = "User 2";
const USER_3: &str = "User 3";
const USER_4: &str = "User 4";

#[derive(Clone, Copy)]
pub struct F32Params {
    pub min: f32,
    pub max: f32,
    pub small_inc: f32,
    pub big_inc: f32,
    pub dec_places: u8,
    pub unit: TString<5>,
}

pub const MAX_ENUM_VARIANTS: usize = 5;

#[derive(Clone, Copy)]
pub struct EnumParams {
    pub variants: [TString<16>; MAX_ENUM_VARIANTS],
}

#[derive(Clone, Copy)]
pub struct StringParams {
    pub content: TString<12>,
}

#[derive(Clone, Copy)]
pub struct ListParams {
    pub max: i32,
}

#[derive(Clone, Copy)]
pub enum Params {
    F32(F32Params),
    Enum(EnumParams),
    String(StringParams),
    List(ListParams),
}

#[derive(Clone, Copy)]
pub enum Content {
    F32(f32),
    Enum(TString<16>),
    String(TString<12>),
    List(i32),
}

impl Editable {
    pub fn params(&self) -> Params {
        match self {
            Editable::Bugs => Params::F32(F32Params {
                min: 0.0,
                max: 100.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("%"),
            }),
            Editable::Display => Params::Enum(EnumParams {
                variants: [
                    TString::<16>::from_str("Vario"),
                    TString::<16>::from_str("Horizon"),
                    TString::<16>::from_str(""),
                    TString::<16>::from_str(""),
                    TString::<16>::from_str(""),
                ],
            }),
            Editable::Glider => Params::List(ListParams {
                max: POLAR_COUNT as i32 - 1,
            }),
            Editable::McCready => Params::F32(F32Params {
                min: 0.0,
                max: 5.0,
                small_inc: 0.1,
                big_inc: 0.1,
                dec_places: 1,
                unit: TString::<5>::from_str("m/s"),
            }),
            Editable::None => Params::String(StringParams {
                content: TString::<12>::from_str(""),
            }),
            Editable::PilotWeight => Params::F32(F32Params {
                min: 0.0,
                max: 250.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("kg"),
            }),
            Editable::Return => Params::String(StringParams {
                content: TString::<12>::from_str(""),
            }),
            Editable::TcClimbRate => Params::F32(F32Params {
                min: 15.0,
                max: 120.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("s"),
            }),
            Editable::TcSpeedToFly => Params::F32(F32Params {
                min: 1.0,
                max: 60.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("s"),
            }),
            Editable::Theme => Params::Enum(EnumParams {
                variants: [
                    TString::<16>::from_str("Dark"),
                    TString::<16>::from_str("Bright"),
                    TString::<16>::from_str(""),
                    TString::<16>::from_str(""),
                    TString::<16>::from_str(""),
                ],
            }),
            Editable::VarioModeControl => Params::Enum(EnumParams {
                variants: [
                    TString::<16>::from_str("Auto"),
                    TString::<16>::from_str("SpeedToFly"),
                    TString::<16>::from_str("Vario"),
                    TString::<16>::from_str(""),
                    TString::<16>::from_str(""),
                ],
            }),
            Editable::Volume => Params::F32(F32Params {
                min: 0.0,
                max: 50.0,
                small_inc: 1.0,
                big_inc: 3.0,
                dec_places: 0,
                unit: TString::<5>::from_str(""),
            }),
            Editable::WaterBallast => Params::F32(F32Params {
                min: 0.0,
                max: 250.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("kg"),
            }),
            Editable::Info1 => Params::List(ListParams {
                max: LineView::max(Placement::Top) as i32,
            }),
            Editable::Info2 => Params::List(ListParams {
                max: LineView::max(Placement::Bottom) as i32,
            }),
            Editable::Rotation => Params::Enum(EnumParams {
                variants: [
                    TString::<16>::from_str(Rotation::Rotate0.name()),
                    TString::<16>::from_str(Rotation::Rotate90.name()),
                    TString::<16>::from_str(Rotation::Rotate180.name()),
                    TString::<16>::from_str(Rotation::Rotate270.name()),
                    TString::<16>::from_str(""),
                ],
            }),
            Editable::CenterFrequency => Params::F32(F32Params {
                min: 500.0,
                max: 1000.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("Hz"),
            }),
            Editable::CenterViewCircling => Params::List(ListParams {
                max: CenterView::max(CenterType::Circling) as i32
            }),
            Editable::CenterViewStraight => Params::List(ListParams {
                max: CenterView::max(CenterType::Straight) as i32
            }),
            Editable::ResetConfig => Params::Enum(EnumParams {
                variants: [
                    TString::<16>::from_str(DEFAULT_CONFIG),
                    TString::<16>::from_str(FACTORY_RESET),
                    TString::<16>::from_str(DO_NOT_CHANGE),
                    TString::<16>::from_str(""),
                    TString::<16>::from_str(""),
                ],
            }),
            Editable::UserProfile => Params::Enum(EnumParams {
                variants: [
                    TString::<16>::from_str(USER_1),
                    TString::<16>::from_str(USER_2),
                    TString::<16>::from_str(USER_3),
                    TString::<16>::from_str(USER_4),
                    TString::<16>::from_str(DO_NOT_CHANGE),
                ],
            }),
        }
    }

    pub fn name(&self) -> TString<16> {
        match self {
            Editable::Bugs => TString::<16>::from_str("Bugs"),
            Editable::Display => TString::<16>::from_str("Display"),
            Editable::Glider => TString::<16>::from_str("Glider"),
            Editable::McCready => TString::<16>::from_str("Mac Cready"),
            Editable::None => TString::<16>::from_str("None"),
            Editable::PilotWeight => TString::<16>::from_str("Pilot Weight"),
            Editable::Return => TString::<16>::from_str("Return"),
            Editable::TcClimbRate => TString::<16>::from_str("TC Climb Rate"),
            Editable::TcSpeedToFly => TString::<16>::from_str("TC Speed to Fly"),
            Editable::Theme => TString::<16>::from_str("Theme"),
            Editable::VarioModeControl => TString::<16>::from_str("Vario Control"),
            Editable::Volume => TString::<16>::from_str("Volume"),
            Editable::WaterBallast => TString::<16>::from_str("Water Ballast"),
            Editable::Info1 => TString::<16>::from_str("Info 1 Content"),
            Editable::Info2 => TString::<16>::from_str("Info 2 Content"),
            Editable::Rotation => TString::<16>::from_str("Display Rotation"),
            Editable::CenterFrequency => TString::<16>::from_str("Center Frequency"),
            Editable::CenterViewCircling => TString::<16>::from_str("Center Circling"),
            Editable::CenterViewStraight => TString::<16>::from_str("Center Straight"),
            Editable::ResetConfig => TString::<16>::from_str("Configuration"),
            Editable::UserProfile => TString::<16>::from_str("User Profile"),
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
                if let Content::F32(val) = content {
                    conv.write_str(params.unit.as_str()).unwrap();
                    conv.write_u8(b' ').unwrap();
                    conv.f32(val, params.dec_places as usize).unwrap();
                }
            }
            Params::List(_params) => {
                if let Content::List(val) = content {
                    match self {
                        Editable::Glider => conv.write_str(POLARS[val as usize].name).unwrap(),
                        Editable::Info1 => conv
                            .write_str(LineView::from_sorted(val as usize, Placement::Top).name())
                            .unwrap(),
                        Editable::Info2 => conv
                            .write_str(
                                LineView::from_sorted(val as usize, Placement::Bottom).name(),
                            )
                            .unwrap(),
                        Editable::CenterViewCircling => conv
                            .write_str(CenterView::from_sorted(val as usize,CenterType::Circling).name())
                            .unwrap(),
                        Editable::CenterViewStraight => conv
                            .write_str(CenterView::from_sorted(val as usize,CenterType::Straight).name())
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

    pub fn content(&self, cm: &CoreModel) -> Content {
        match self {
            Editable::Bugs => Content::F32((cm.glider_data.bugs - 1.0) * 100.0),
            Editable::Display => match cm.config.last_display_active {
                DisplayActive::Horizon => Content::Enum(TString::<16>::from_str("Horizon")),
                _ => Content::Enum(TString::<16>::from_str("Vario")),
            },
            Editable::Glider => Content::List(cm.config.glider_idx),
            Editable::McCready => Content::F32(cm.config.mc_cready.to_m_s()),
            Editable::None => Content::String(TString::<12>::from_str("")),
            Editable::PilotWeight => Content::F32(cm.glider_data.pilot_weight.to_kg()),
            Editable::Return => Content::String(TString::<12>::from_str("")),
            Editable::TcClimbRate => Content::F32(cm.config.av2_climb_rate_tc),
            Editable::TcSpeedToFly => Content::F32(cm.config.av_speed_to_fly_tc),
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
            Editable::Volume => Content::F32(cm.config.volume as f32),
            Editable::WaterBallast => Content::F32(cm.glider_data.water_ballast.to_kg()),
            Editable::Info1 => Content::List(cm.config.info1.sorted_as_i32(Placement::Top)),
            Editable::Info2 => Content::List(cm.config.info2.sorted_as_i32(Placement::Bottom)),
            Editable::Rotation => {
                Content::Enum(TString::<16>::from_str(cm.control.rotation.name()))
            }
            Editable::CenterFrequency => Content::F32(cm.config.snd_center_freq as f32),
            Editable::CenterViewCircling => Content::List(cm.config.center_circling.sorted_as_i32(CenterType::Circling)),
            Editable::CenterViewStraight => Content::List(cm.config.center_straignt.sorted_as_i32(CenterType::Straight)),
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
        }
    }

    pub fn set_enum_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: &TString<16>) {
        match self {
            Editable::Display => match val.as_str() {
                "Horizon" => cc.persist_set(
                    cm,
                    Variant::DisplayActive(DisplayActive::Horizon),
                    PersistenceId::Display,
                    Echo::None,
                ),
                _ => cc.persist_set(
                    cm,
                    Variant::DisplayActive(DisplayActive::Vario),
                    PersistenceId::Display,
                    Echo::None,
                ),
            },
            Editable::Theme => cc.persist_set(
                cm,
                Variant::Str(val.as_str()),
                PersistenceId::DisplayTheme,
                Echo::None,
            ),
            Editable::VarioModeControl => {
                let mode = match val.as_str() {
                    "Vario" => VarioModeControl::Vario,
                    "SpeedToFly" => VarioModeControl::SpeedToFly,
                    _ => VarioModeControl::Auto,
                };
                cc.persist_set(
                    cm,
                    Variant::VarioModeControl(mode),
                    PersistenceId::VarioModeControl,
                    Echo::NmeaAndCan,
                );
            }
            Editable::Rotation => cc.persist_set(
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
                        0 => cc.persist_delete_config(),
                        1 => cc.persist_factory_reset(),
                        _ => (),
                    }
                }
            },
            Editable::UserProfile => {
                if cm.control.editor.enter_pushed && val.as_str() != DO_NOT_CHANGE {
                    cm.config.user_profile = match val.as_str() {
                        USER_1 => 0,
                        USER_2 => 1,
                        USER_3 => 2,
                        USER_4 => 3,
                        _ => 0,
                    };
                    cc.persist_user_profile(cm); // store value and reset device
                }
           },
            _ => (),
        }
    }

    pub fn set_f32_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: f32) {
        match self {
            Editable::Bugs => cc.persist_set(
                cm,
                Variant::F32(1.0 + val / 100.0),
                PersistenceId::Bugs,
                Echo::NmeaAndCan,
            ),
            Editable::McCready => cc.persist_set(
                cm,
                Variant::Speed(val.m_s()),
                PersistenceId::McCready,
                Echo::NmeaAndCan,
            ),
            Editable::PilotWeight => cc.persist_set(
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::PilotWeight,
                Echo::NmeaAndCan,
            ),
            Editable::TcClimbRate => {
                cc.persist_set(cm, Variant::F32(val), PersistenceId::TcClimbRate, Echo::Can)
            }
            Editable::TcSpeedToFly => cc.persist_set(
                cm,
                Variant::F32(val),
                PersistenceId::TcSpeedToFly,
                Echo::Can,
            ),
            Editable::Volume => cc.persist_set(
                cm,
                Variant::I8(val as i8),
                PersistenceId::Volume,
                Echo::NmeaAndCan,
            ),
            Editable::WaterBallast => cc.persist_set(
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::WaterBallast,
                Echo::NmeaAndCan,
            ),
            Editable::CenterFrequency => cc.persist_set(
                cm,
                Variant::F32(val),
                PersistenceId::CenterFrequency,
                Echo::Can,
            ),
            _ => (),
        }
    }

   
    #[allow(clippy::single_match)]
    pub fn set_list_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: i32) {
        match self {
            Editable::Glider => {
                cc.polar = Polar::new(&POLARS[val as usize], &mut cm.glider_data);
                cc.persist_set(cm, Variant::I32(val), PersistenceId::Glider, Echo::None)
            }
            Editable::Info1 => {
                let variant = LineView::from_sorted(val as usize, Placement::Top) as i32;
                cc.persist_set(cm, Variant::I32(variant), PersistenceId::Info1, Echo::None)
            }
            Editable::Info2 => {
                let variant = LineView::from_sorted(val as usize, Placement::Bottom) as i32;
                cc.persist_set(cm, Variant::I32(variant), PersistenceId::Info2, Echo::None)
            }
            Editable::CenterViewCircling => {
                let variant = CenterView::from_sorted(val as usize, CenterType::Circling) as i32;
                cc.persist_set(cm, Variant::I32(variant), PersistenceId::CenterViewCircling, Echo::None)
            }
            Editable::CenterViewStraight => {
                let variant = CenterView::from_sorted(val as usize, CenterType::Straight) as i32;
                cc.persist_set(cm, Variant::I32(variant), PersistenceId::CenterViewStraight, Echo::None)
            }
            _ => (),
        }
    }
}
