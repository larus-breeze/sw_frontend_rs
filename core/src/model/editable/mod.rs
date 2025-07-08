/// Elements that can be changed by the user
///
/// Editables are always saved in the model or controllerand can be changed by the user. These
/// can be parameters, display selection, time constants or other data. With the help of this
/// module, the implemented editor is able to display and change such data, save it and, if
/// necessary, output it at the NMEA and CAN interfaces.
///
/// New elements are added with the following steps:
///   - First, the persistence layer is extended (src/controller/persistence.rs)
///     - Extend PersistenceId
///     - Extend restore_item()
///   - Then the enum Editable is extended by the new element (see below)
///   - Create a empty struct with same name as enum variant and implement trait EditableFuncs
///   - Add reference to fn Editable::this()
///   - Add the new editable to the menu structure (src/model/menu)
mod model;
mod controller;
mod glider_data;
mod sensorbox;

use model::*;
use controller::*;
use glider_data::*;
use sensorbox::*;

use crate::{utils::TString, CoreController, CoreModel};
use tfmt::Convert;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Editable {
    // model
    AlarmVolume,
    AvgClimbRateSrc,
    BatteryGood,
    BatteryLow,
    CenterFrequency,
    CenterViewCircling,
    CenterViewStraight,
    Display,
    Glider,
    GliderSymbol,
    Info1,
    Info2,
    Info3,
    McCready,
    StfUpperLimit,
    StfLowerLimit,
    TcCircleHysteresis,
    TcClimbRate,
    TcSpeedToFly,
    Theme,
    Volume,

    // controller
    DrainPinConfig,
    EnergyArrowMult,
    FactoryReset,
    FlashControl,
    FlowEmpty,
    FlowSlope,
    GearPinConfig,
    AirbrakesPinConfig,
    GearAlarmModeConfig,
    ResetConfig,
    Rotation,
    SpeedToFlyPinConfig,
    UserProfile,
    VarioModeControl,

    // glider_data
    Bugs,
    PilotWeight,
    WaterBallast,
    EmptyMass,
    MaxBallast,
    ReferenceWeight,
    PolarValueV1,
    PolarValueV2,
    PolarValueV3,
    PolarValueSi1,
    PolarValueSi2,
    PolarValueSi3,

    // sensorbox
    SensTiltRoll,
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
    VarioPressTc,
    CmdMeas1,
    CmdMeas2,
    CmdMeas3,
    CmdCalcOrientation,
    CmdFineTuneOrientation,
    CmdResetSensorbox,

    // general
    None,
    Return,
}

#[derive(Clone, Copy)]
pub enum Content {
    F32(Option<f32>),
    Enum(TString<16>),
    String(TString<12>),
    List(i32),
    Command(TString<16>),
}

#[derive(Clone, Copy)]
pub struct F32Params {
    pub min: f32,
    pub max: f32,
    pub small_inc: f32,
    pub big_inc: f32,
    pub dec_places: u8,
    pub unit: &'static str,
}

pub const MAX_ENUM_VARIANTS: usize = 5;

#[derive(Clone, Copy)]
pub struct EnumParams {
    pub variants: [&'static str; MAX_ENUM_VARIANTS],
}

#[derive(Clone, Copy)]
pub struct StringParams {
    pub content: TString<16>,
}

#[derive(Clone, Copy)]
pub struct ListParams {
    pub max: i32,
}

#[derive(Clone, Copy)]
pub struct CmdParams {
    pub content: TString<16>,
}

#[derive(Clone, Copy)]
pub enum Params {
    F32(F32Params),
    Enum(EnumParams),
    String(StringParams),
    List(ListParams),
    Cmd(CmdParams),
}

struct EditableFptrs {
    name: fn() -> &'static str,
    content: fn(&mut CoreModel, &mut CoreController) -> Content,
    content_as_str: fn(&mut Convert<20>, i32),
    params: fn() -> Params,
    set_content: fn(&mut CoreModel, &mut CoreController, Content),
}

trait EditableFuncs {
    fn name() -> &'static str {
        ""
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::String(TString::<12>::from_str(""))
    }

    fn content_as_str(_convert: &mut Convert<20>, _idx: i32) {}

    fn params() -> Params {
        Params::String(StringParams {
            content: TString::<16>::from_str(""),
        })
    }

    fn set_content(_cm: &mut CoreModel, _cc: &mut CoreController, _content: Content) {}

    fn this() -> EditableFptrs {
        EditableFptrs {
            name: Self::name,
            content: Self::content,
            content_as_str: Self::content_as_str,
            params: Self::params,
            set_content: Self::set_content,
        }
    }
}

struct None_;
impl EditableFuncs for None_ {
    fn name() -> &'static str {
        "None"
    }
}

struct Return;
impl EditableFuncs for Return {
    fn name() -> &'static str {
        "Return"
    }
}

impl Editable {
    fn this(&self) -> EditableFptrs {
        match self {
            // model
            Editable::AlarmVolume => AlarmVolume::this(),
            Editable::AvgClimbRateSrc => AvgClimbRateSrc::this(),
            Editable::BatteryGood => BatteryGood::this(),
            Editable::BatteryLow => BatteryLow::this(),
            Editable::CenterFrequency => CenterFrequency::this(),
            Editable::CenterViewCircling => CenterViewCircling::this(),
            Editable::CenterViewStraight => CenterViewStraight::this(),
            Editable::Display => Display::this(),
            Editable::Glider => Glider::this(),
            Editable::GliderSymbol => GliderSymbol::this(),
            Editable::Info1 => Info1::this(),
            Editable::Info2 => Info2::this(),
            Editable::McCready => McCready::this(),
            Editable::Info3 => Info3::this(),
            Editable::StfUpperLimit => StfUpperLimit::this(),
            Editable::StfLowerLimit => StfLowerLimit::this(),
            Editable::TcCircleHysteresis => TcCircleHysteresis::this(),
            Editable::TcClimbRate => TcClimbRate::this(),
            Editable::TcSpeedToFly => TcSpeedToFly::this(),
            Editable::Theme => Theme::this(),
            Editable::Volume => Volume::this(),

            // controller
            Editable::DrainPinConfig => DrainPinConfig::this(),
            Editable::EnergyArrowMult => EnergyArrowMult::this(),
            Editable::FactoryReset => FactoryReset::this(),
            Editable::FlashControl => FlashControl::this(),
            Editable::FlowEmpty => FlowEmpty::this(),
            Editable::FlowSlope => FlowSlope::this(),
            Editable::GearPinConfig => GearPinConfig::this(),
            Editable::AirbrakesPinConfig => AirbrakesPinConfig::this(),
            Editable::GearAlarmModeConfig => GearAlarmModeConfig::this(),
            Editable::ResetConfig => ResetConfig::this(),
            Editable::Rotation => Rotation_::this(),
            Editable::SpeedToFlyPinConfig => SpeedToFlyPinConfig::this(),
            Editable::UserProfile => UserProfile::this(),
            Editable::VarioModeControl => VarioModeControl_::this(),

            // glider_data
            Editable::Bugs => Bugs::this(),
            Editable::PilotWeight => PilotWeight::this(),
            Editable::WaterBallast => WaterBallast::this(),
            Editable::EmptyMass => EmptyMass::this(),
            Editable::MaxBallast => MaxBallast::this(),
            Editable::ReferenceWeight => ReferenceWeight::this(),
            Editable::PolarValueV1 => PolarValueV1::this(),
            Editable::PolarValueV2 => PolarValueV2::this(),
            Editable::PolarValueV3 => PolarValueV3::this(),
            Editable::PolarValueSi1 => PolarValueSi1::this(),
            Editable::PolarValueSi2 => PolarValueSi2::this(),
            Editable::PolarValueSi3 => PolarValueSi3::this(),

            // sensorbox
            Editable::SensTiltRoll => SensTiltRoll::this(),
            Editable::SensTiltPitch => SensTiltPitch::this(),
            Editable::SensTiltYaw => SensTiltYaw::this(),
            Editable::PitotOffset => PitotOffset::this(),
            Editable::PitotSpan => PitotSpan::this(),
            Editable::QnhDelta => QnhDelta::this(),
            Editable::MagAutoCalib => MagAutoCalib::this(),
            Editable::VarioTc => VarioTc::this(),
            Editable::VarioIntTc => VarioIntTc::this(),
            Editable::WindTc => WindTc::this(),
            Editable::MeanWindTc => MeanWindTc::this(),
            Editable::GnssConfig => GnssConfig::this(),
            Editable::AntBaselen => AntBaselen::this(),
            Editable::AntSlaveDown => AntSlaveDown::this(),
            Editable::AntSlaveRight => AntSlaveRight::this(),
            Editable::VarioPressTc => VarioPressTc::this(),
            Editable::CmdMeas1 => CmdMeas1::this(),
            Editable::CmdMeas2 => CmdMeas2::this(),
            Editable::CmdMeas3 => CmdMeas3::this(),
            Editable::CmdCalcOrientation => CmdCalcOrientation::this(),
            Editable::CmdFineTuneOrientation => CmdFineTuneOrientation::this(),
            Editable::CmdResetSensorbox => CmdResetSensorbox::this(),

            // general
            Editable::None => None_::this(),
            Editable::Return => Return::this(),
        }
    }

    pub fn content_as_str(&self, content: Content) -> TString<20> {
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
                    ((self.this()).content_as_str)(&mut conv, val);
                }
            }
            Params::String(_params) => {
                if let Content::String(val) = content {
                    conv.write_str(val.as_str()).unwrap();
                }
            }
            Params::Cmd(_params) => {
                if let Content::Command(msg) = content {
                    conv.write_str(msg.as_str()).unwrap();
                }
            }
        }
        TString::<20>::from_str(conv.as_str())
    }

    pub fn content(&self, cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        ((self.this()).content)(cm, cc)
    }

    pub fn name(&self) -> &'static str {
        ((self.this()).name)()
    }

    pub fn params(&self) -> Params {
        ((self.this()).params)()
    }

    pub fn set_content(&self, cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        ((self.this()).set_content)(cm, cc, content)
    }
}
