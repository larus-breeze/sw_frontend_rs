use super::{
    Editable, DEFAULT_CONFIG, DO_NOT_CHANGE, FACTORY_RESET, USER_1, USER_2, USER_3, USER_4,
};
use crate::{
    flight_physics::polar_store,
    utils::TString,
    view::viewable::{
        centerview::{CenterType, CenterView},
        lineview::{LineView, Placement},
    },
    Rotation,
};

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
                max: polar_store::size() as i32 - 1,
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
                max: CenterView::max(CenterType::Circling) as i32,
            }),
            Editable::CenterViewStraight => Params::List(ListParams {
                max: CenterView::max(CenterType::Straight) as i32,
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
            Editable::EmptyMass => Params::F32(F32Params {
                min: 100.0,
                max: 850.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("kg"),
            }),
            Editable::MaxBallast => Params::F32(F32Params {
                min: 0.0,
                max: 300.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("kg"),
            }),
            Editable::ReferenceWeight => Params::F32(F32Params {
                min: 100.0,
                max: 1000.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("kg"),
            }),
            Editable::PolarValueV1 => Params::F32(F32Params {
                min: 50.0,
                max: 250.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("km/h"),
            }),
            Editable::PolarValueV2 => Params::F32(F32Params {
                min: 50.0,
                max: 250.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("km/h"),
            }),
            Editable::PolarValueV3 => Params::F32(F32Params {
                min: 50.0,
                max: 250.0,
                small_inc: 1.0,
                big_inc: 10.0,
                dec_places: 0,
                unit: TString::<5>::from_str("km/h"),
            }),
            Editable::PolarValueSi1 => Params::F32(F32Params {
                min: -5.0,
                max: 0.0,
                small_inc: 0.01,
                big_inc: 0.1,
                dec_places: 2,
                unit: TString::<5>::from_str("m/s"),
            }),
            Editable::PolarValueSi2 => Params::F32(F32Params {
                min: -5.0,
                max: 0.0,
                small_inc: 0.01,
                big_inc: 0.1,
                dec_places: 2,
                unit: TString::<5>::from_str("m/s"),
            }),
            Editable::PolarValueSi3 => Params::F32(F32Params {
                min: -5.0,
                max: 0.0,
                small_inc: 0.01,
                big_inc: 0.1,
                dec_places: 2,
                unit: TString::<5>::from_str("m/s"),
            }),
        }
    }
}
