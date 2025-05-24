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
///   - Extend necessary mehtods content(), name(), params() and set_content()
///   - Add new editables to the menu structure (src/model/menu)
mod content;
mod name;
mod params;
mod set_content;

use crate::{
    model::VarioModeControl,
    polar_store,
    utils::TString,
    view::viewable::{
        centerview::{CenterType, CenterView},
        lineview::{LineView, Placement},
    },
};

use super::DisplayActive;
pub(crate) use params::{EnumParams, F32Params, ListParams, CmdParams, Params, MAX_ENUM_VARIANTS};
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
    GliderSymbol,
    BatteryGood,
    BatteryLow,
    DrainPinConfig,
    FlowEmpty,
    FlowSlope,
    FlashControl,
    SpeedToFlyPinConfig,
    GearPinConfig,
    AirbrakesPinConfig,
    GearAlarmModeConfig,
    AlarmVolume,
    StfUpperLimit,
    StfLowerLimit,
    AvgClimbRateSrc,

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

    CmdMeas1, // These are sensorbox commands
    CmdMeas2,
    CmdMeas3,
    CmdCalcOrientation,
    CmdFineTuneOrientation,
}

const DEFAULT_CONFIG: &str = "Default Config";
const FACTORY_RESET: &str = "Factory Reset";
const DO_NOT_CHANGE: &str = "Do not change";

const COMMAND_SENT: &str = "Command sent";

const USER_1: &str = "User 1";
const USER_2: &str = "User 2";
const USER_3: &str = "User 3";
const USER_4: &str = "User 4";

const ON: &str = "On";
const OFF: &str = "Off";

#[derive(Clone, Copy)]
pub enum Content {
    F32(Option<f32>),
    Enum(TString<16>),
    String(TString<12>),
    List(i32),
    Command(TString<16>),
}

impl Editable {
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
            Params::Cmd(_params) => {
                if let Content::Command(msg) = content {
                    conv.write_str(msg.as_str()).unwrap();
                }
            }
        }
        TString::<20>::from_str(conv.as_str())
    }
}
