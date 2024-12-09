use crate::{
    model::VarioModeControl,
    utils::TString,
    view::helpers::themes::{BRIGHT_MODE, DARK_MODE},
    CoreController, CoreModel, Echo, FloatToMass, FloatToSpeed, PersistenceId, Polar, POLARS,
    POLAR_COUNT,
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
    Theme,
    VarioModeControl,
    Volume,
    WaterBallast,
}

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
    pub variants: [TString<12>; MAX_ENUM_VARIANTS],
}

#[derive(Clone, Copy)]
pub struct StringParams {
    pub content: TString<12>,
}

#[derive(Clone, Copy)]
pub struct PolarParams {
    pub max: i32,
}

#[derive(Clone, Copy)]
pub enum Params {
    F32(F32Params),
    Enum(EnumParams),
    String(StringParams),
    Polar(PolarParams),
}

#[derive(Clone, Copy)]
pub enum Content {
    F32(f32),
    Enum(TString<12>),
    String(TString<12>),
    Polar(i32),
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
                    TString::<12>::from_str("Vario"),
                    TString::<12>::from_str("Horizon"),
                    TString::<12>::from_str(""),
                    TString::<12>::from_str(""),
                    TString::<12>::from_str(""),
                ],
            }),
            Editable::Glider => Params::Polar(PolarParams {
                max: POLAR_COUNT as i32 - 1,
            }),
            Editable::McCready => Params::F32(F32Params {
                min: 0.0,
                max: 5.0,
                small_inc: 0.1,
                big_inc: 0.5,
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
            Editable::Theme => Params::Enum(EnumParams {
                variants: [
                    TString::<12>::from_str("Dark"),
                    TString::<12>::from_str("Bright"),
                    TString::<12>::from_str(""),
                    TString::<12>::from_str(""),
                    TString::<12>::from_str(""),
                ],
            }),
            Editable::VarioModeControl => Params::Enum(EnumParams {
                variants: [
                    TString::<12>::from_str("Auto"),
                    TString::<12>::from_str("SpeedToFly"),
                    TString::<12>::from_str("Vario"),
                    TString::<12>::from_str(""),
                    TString::<12>::from_str(""),
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
            Editable::Theme => TString::<16>::from_str("Theme"),
            Editable::VarioModeControl => TString::<16>::from_str("Vario Control"),
            Editable::Volume => TString::<16>::from_str("Volume"),
            Editable::WaterBallast => TString::<16>::from_str("Water Ballast"),
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
            Params::Polar(_params) => {
                if let Content::Polar(val) = content {
                    conv.write_str(POLARS[val as usize].name).unwrap();
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
                DisplayActive::Horizon => Content::Enum(TString::<12>::from_str("Horizon")),
                _ => Content::Enum(TString::<12>::from_str("Vario")),
            },
            Editable::Glider => Content::Polar(cm.config.glider_idx),
            Editable::McCready => Content::F32(cm.config.mc_cready.to_m_s()),
            Editable::None => Content::String(TString::<12>::from_str("")),
            Editable::PilotWeight => Content::F32(cm.glider_data.pilot_weight.to_kg()),
            Editable::Return => Content::String(TString::<12>::from_str("")),
            Editable::Theme => {
                if cm.config.theme == &DARK_MODE {
                    Content::Enum(TString::<12>::from_str("Dark"))
                } else {
                    Content::Enum(TString::<12>::from_str("Bright"))
                }
            }
            Editable::VarioModeControl => match cm.control.vario_mode_control {
                VarioModeControl::Auto => Content::Enum(TString::<12>::from_str("Auto")),
                VarioModeControl::Vario => Content::Enum(TString::<12>::from_str("Vario")),
                VarioModeControl::SpeedToFly => {
                    Content::Enum(TString::<12>::from_str("SpeedToFly"))
                }
            },
            Editable::Volume => Content::F32(cm.config.volume as f32),
            Editable::WaterBallast => Content::F32(cm.glider_data.water_ballast.to_kg()),
        }
    }

    pub fn set_enum_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: &TString<12>) {
        match self {
            Editable::Display => match val.as_str() {
                "Horizon" => cc.persist_set_display(cm, DisplayActive::Horizon, Echo::None),
                _ => cc.persist_set_display(cm, DisplayActive::Vario, Echo::None),
            },
            Editable::Theme => {
                cm.config.theme = match val.as_str() {
                    "Bright" => &BRIGHT_MODE,
                    _ => &DARK_MODE,
                };
                cc.persist_push_id(PersistenceId::DisplayTheme);
            }
            Editable::VarioModeControl => {
                let mode = match val.as_str() {
                    "Vario" => VarioModeControl::Vario,
                    "SpeedToFly" => VarioModeControl::SpeedToFly,
                    _ => VarioModeControl::Auto,
                };
                cc.persist_set_vario_mode_control(cm, mode, Echo::NmeaAndCan);
            }
            _ => (),
        }
    }

    pub fn set_f32_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: f32) {
        match self {
            Editable::Bugs => cc.persist_set_bugs(cm, 1.0 + val / 100.0, Echo::NmeaAndCan),
            Editable::McCready => cc.persist_set_maccready(cm, val.m_s(), Echo::NmeaAndCan),
            Editable::PilotWeight => cc.persist_set_pilot_weight(cm, val.kg(), Echo::NmeaAndCan),
            Editable::Volume => cc.persist_set_volume(cm, val as i8, Echo::NmeaAndCan),
            Editable::WaterBallast => cc.persist_set_water_ballast(cm, val.kg(), Echo::NmeaAndCan),
            _ => (),
        }
    }

    #[allow(clippy::single_match)]
    pub fn set_polar_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: i32) {
        match self {
            Editable::Glider => {
                cc.polar = Polar::new(&POLARS[val as usize], &mut cm.glider_data);
                cc.persist_set_glider_idx(cm, val, Echo::None)
            }
            _ => (),
        }
    }
}
