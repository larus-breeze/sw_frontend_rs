use crate::{
    controller,
    model::VarioModeControl,
    themes::{BRIGHT_MODE, DARK_MODE},
    utils::TString,
    CoreController, CoreModel, Echo, Editable, FloatToMass, FloatToSpeed, PersistenceId,
    POLAR_COUNT,
};

use super::DisplayActive;

#[derive(Clone, Copy)]
pub struct F32Params {
    pub text: TString<16>,
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
    pub text: TString<16>,
    pub variants: [TString<12>; MAX_ENUM_VARIANTS],
}

#[derive(Clone, Copy)]
pub struct StringParams {
    pub text: TString<16>,
}

#[derive(Clone, Copy)]
pub struct PolarParams {
    pub text: TString<16>,
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

pub fn get_params(cm: &mut CoreModel, target: Editable) {
    cm.control.editor.params = match target {
        Editable::Glider => Params::Polar(PolarParams {
            text: TString::<16>::from_str("Glider"),
            max: POLAR_COUNT as i32 - 1,
        }),
        Editable::McCready => Params::F32(F32Params {
            text: TString::<16>::from_str("Mac Cready"),
            min: 0.0,
            max: 5.0,
            small_inc: 0.1,
            big_inc: 0.5,
            dec_places: 1,
            unit: TString::<5>::from_str("m/s"),
        }),
        Editable::PilotWeight => Params::F32(F32Params {
            text: TString::<16>::from_str("Pilot Weight"),
            min: 0.0,
            max: 250.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: TString::<5>::from_str("kg"),
        }),
        Editable::VarioModeControl => Params::Enum(EnumParams {
            text: TString::<16>::from_str("Vario Control"),
            variants: [
                TString::<12>::from_str("Auto"),
                TString::<12>::from_str("SpeedToFly"),
                TString::<12>::from_str("Vario"),
                TString::<12>::from_str(""),
                TString::<12>::from_str(""),
            ],
        }),
        Editable::Volume => Params::F32(F32Params {
            text: TString::<16>::from_str("Volume"),
            min: 0.0,
            max: 50.0,
            small_inc: 1.0,
            big_inc: 1.0,
            dec_places: 0,
            unit: TString::<5>::from_str(""),
        }),
        Editable::WaterBallast => Params::F32(F32Params {
            text: TString::<16>::from_str("Water Ballast"),
            min: 0.0,
            max: 250.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: TString::<5>::from_str("kg"),
        }),
        Editable::Theme => Params::Enum(EnumParams {
            text: TString::<16>::from_str("Theme"),
            variants: [
                TString::<12>::from_str("Dark"),
                TString::<12>::from_str("Bright"),
                TString::<12>::from_str(""),
                TString::<12>::from_str(""),
                TString::<12>::from_str(""),
            ],
        }),
        Editable::Display => Params::Enum(EnumParams {
            text: TString::<16>::from_str("Display"),
            variants: [
                TString::<12>::from_str("Vario"),
                TString::<12>::from_str("Horizon"),
                TString::<12>::from_str(""),
                TString::<12>::from_str(""),
                TString::<12>::from_str(""),
            ],
        }),
        Editable::None => Params::String(StringParams {
            text: TString::<16>::from_str("None"),
        }),
    };
}

pub fn get_content(cm: &mut CoreModel, target: Editable) {
    cm.control.editor.content = match target {
        Editable::Glider => Content::Polar(cm.config.glider_idx),
        Editable::McCready => Content::F32(cm.config.mc_cready.to_m_s()),
        Editable::PilotWeight => Content::F32(cm.glider_data.pilot_weight.to_kg()),
        Editable::VarioModeControl => match cm.control.vario_mode_control {
            VarioModeControl::Auto => Content::Enum(TString::<12>::from_str("Auto")),
            VarioModeControl::Vario => Content::Enum(TString::<12>::from_str("Vario")),
            VarioModeControl::SpeedToFly => Content::Enum(TString::<12>::from_str("SpeedToFly")),
        },
        Editable::Volume => Content::F32(cm.config.volume as f32),
        Editable::WaterBallast => Content::F32(cm.glider_data.water_ballast.to_kg()),
        Editable::Theme => {
            if cm.config.theme == &DARK_MODE {
                Content::Enum(TString::<12>::from_str("Dark"))
            } else {
                Content::Enum(TString::<12>::from_str("Bright"))
            }
        }
        Editable::Display => match cm.config.display_active {
            DisplayActive::Horizon => Content::Enum(TString::<12>::from_str("Horizon")),
            _ => Content::Enum(TString::<12>::from_str("Vario")),
        },
        Editable::None => Content::String(TString::<12>::from_str("")),
    };
}

pub fn set_enum_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    val: &TString<12>,
    target: Editable,
) {
    match target {
        Editable::VarioModeControl => {
            let mode = match val.as_str() {
                "Vario" => VarioModeControl::Vario,
                "SpeedToFly" => VarioModeControl::SpeedToFly,
                _ => VarioModeControl::Auto,
            };
            cc.persist_set_vario_mode_control(cm, mode, Echo::NmeaAndCan);
        }
        Editable::Theme => {
            cm.config.theme = match val.as_str() {
                "Bright" => &BRIGHT_MODE,
                _ => &DARK_MODE,
            };
            cc.persist_push_id(PersistenceId::DisplayTheme);
        }
        Editable::Display => match val.as_str() {
            "Horizon" => controller::set_horizon_active(cm),
            _ => controller::set_vario_active(cm),
        },
        _ => (),
    }
}

pub fn set_f32_content(cm: &mut CoreModel, cc: &mut CoreController, val: f32, target: Editable) {
    match target {
        Editable::McCready => cc.persist_set_maccready(cm, val.m_s(), Echo::NmeaAndCan),
        Editable::PilotWeight => cc.persist_set_pilot_weight(cm, val.kg(), Echo::NmeaAndCan),
        Editable::Volume => cc.persist_set_volume(cm, val as i8, Echo::NmeaAndCan),
        Editable::WaterBallast => cc.persist_set_water_ballast(cm, val.kg(), Echo::NmeaAndCan),
        _ => (),
    }
}

pub fn set_polar_content(cm: &mut CoreModel, cc: &mut CoreController, val: i32, target: Editable) {
    match target {
        Editable::Glider => cc.persist_set_glider_idx(cm, val, Echo::None),
        _ => (),
    }
}
