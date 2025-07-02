use super::{Content, EditableFuncs, EnumParams, F32Params, Params};
use crate::{
    controller::{GearPins, InPinFunction, InTogglePinFunction, OutPinFunction},
    controller::{
        ONE_PIN_MODE, PIN_IN_CLOSE, PIN_IN_OPEN, PIN_IN_TOGGLE, PIN_NONE, PIN_OUT_CLOSE,
        PIN_OUT_OPEN, TWO_PIN_MODE,
    },
    model::control::{VARIO_MODE_CONTROL_AUTO, VARIO_MODE_CONTROL_NMEA, VARIO_MODE_CONTROL_PIN},
    model::VarioModeControl,
    persist,
    utils::{TString, Variant},
    CoreController, CoreModel, Echo, PersistenceId, Rotation,
};

pub struct DrainPinConfig;
const PIN_PARAMS: Params = Params::Enum(EnumParams {
    variants: [PIN_NONE, PIN_IN_CLOSE, PIN_IN_OPEN, "", ""],
});

impl EditableFuncs for DrainPinConfig {
    fn name() -> &'static str {
        "Drain Pin Config"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cc.drain_control.pin_function().as_str(),
        ))
    }

    fn params() -> Params {
        PIN_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U8(InPinFunction::from(val.as_str()) as u8),
                PersistenceId::DrainPinConfig,
                Echo::None,
            )
        }
    }
}

pub struct FactoryReset;
const DO_NOT_CHANGE: &str = "Do not change";
const FACTORY_RESET: &str = "Delete all";
const DO_NOT_CHANGE_2: &str = "Do not change ";

impl EditableFuncs for FactoryReset {
    fn name() -> &'static str {
        "Factory Reset"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        let s = match cm.control.reset_config {
            1 => FACTORY_RESET,
            _ => DO_NOT_CHANGE,
        };
        Content::Enum(TString::<16>::from_str(s))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [DO_NOT_CHANGE, FACTORY_RESET, DO_NOT_CHANGE_2, "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            cm.control.reset_config = match val.as_str() {
                DO_NOT_CHANGE => 0,
                FACTORY_RESET => 1,
                _ => 2,
            };
            if cm.control.editor.enter_pushed {
                match cm.control.reset_config {
                    1 => persist::factory_reset(cc),
                    _ => (),
                }
            }
        }
    }
}

pub struct FlashControl;
impl EditableFuncs for FlashControl {
    fn name() -> &'static str {
        "Flash Control"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cc.flash_control.pin_function().as_str(),
        ))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [PIN_NONE, PIN_OUT_CLOSE, PIN_OUT_OPEN, "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U8(OutPinFunction::from(val.as_str()) as u8),
                PersistenceId::FlashControl,
                Echo::None,
            )
        }
    }
}

pub struct FlowEmpty;
impl EditableFuncs for FlowEmpty {
    fn name() -> &'static str {
        "Lowest Flow"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::F32(Some(cc.drain_control.flow_rate_offset))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 1.0,
            max: 200.0,
            small_inc: 0.1,
            big_inc: 1.0,
            dec_places: 1,
            unit: "l/min",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::FlowEmpty,
                Echo::None,
            );
        }
    }
}

pub struct FlowSlope;
impl EditableFuncs for FlowSlope {
    fn name() -> &'static str {
        "Flow Slope"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::F32(Some(cc.drain_control.flow_rate_slope))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: -1.0,
            max: 1.0,
            small_inc: 0.001,
            big_inc: 0.010,
            dec_places: 3,
            unit: "l/(min*kg*s)",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::FlowSlope,
                Echo::None,
            );
        }
    }
}

pub struct GearPinConfig;
impl EditableFuncs for GearPinConfig {
    fn name() -> &'static str {
        "Gear Pin Config"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cc.gear_alarm_control.gear_pin_function().as_str(),
        ))
    }

    fn params() -> Params {
        PIN_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U8(InPinFunction::from(val.as_str()) as u8),
                PersistenceId::GearPinConfig,
                Echo::None,
            )
        }
    }
}

pub struct AirbrakesPinConfig;
impl EditableFuncs for AirbrakesPinConfig {
    fn name() -> &'static str {
        "Airbrakes Pin Config"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cc.gear_alarm_control.airbrakes_pin_function().as_str(),
        ))
    }

    fn params() -> Params {
        PIN_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U8(InPinFunction::from(val.as_str()) as u8),
                PersistenceId::AirbrakesPinConfig,
                Echo::None,
            )
        }
    }
}

pub struct GearAlarmModeConfig;
impl EditableFuncs for GearAlarmModeConfig {
    fn name() -> &'static str {
        "Gear Alarm Config"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cc.gear_alarm_control.gear_pin_mode().as_str(),
        ))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [ONE_PIN_MODE, TWO_PIN_MODE, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U8(GearPins::from(val.as_str()) as u8),
                PersistenceId::GearAlarmMode,
                Echo::None,
            )
        }
    }
}

pub struct ResetConfig;
const DEFAULT_CONFIG: &str = "Default Config";

impl EditableFuncs for ResetConfig {
    fn name() -> &'static str {
        "Config Reset"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        let s = match cm.control.reset_config {
            0 => DEFAULT_CONFIG,
            _ => DO_NOT_CHANGE,
        };
        Content::Enum(TString::<16>::from_str(s))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [DEFAULT_CONFIG, DO_NOT_CHANGE, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            cm.control.reset_config = match val.as_str() {
                DEFAULT_CONFIG => 0,
                FACTORY_RESET => 1,
                _ => 2,
            };
            if cm.control.editor.enter_pushed {
                match cm.control.reset_config {
                    0 => persist::delete_config(cc),
                    _ => (),
                }
            }
        }
    }
}

pub struct Rotation_;
impl EditableFuncs for Rotation_ {
    fn name() -> &'static str {
        "Display Rotation"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(cm.control.rotation.name()))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [
                Rotation::Rotate0.name(),
                Rotation::Rotate90.name(),
                Rotation::Rotate180.name(),
                Rotation::Rotate270.name(),
                "",
            ],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U32(Rotation::from(val.as_str()) as u32),
                PersistenceId::Rotation,
                Echo::None,
            );
        }
    }
}

pub struct SpeedToFlyPinConfig;
impl EditableFuncs for SpeedToFlyPinConfig {
    fn name() -> &'static str {
        "StF Pin Config"
    }

    fn content(_cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cc.speed_to_fly_control.pin_function().as_str(),
        ))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [PIN_NONE, PIN_IN_CLOSE, PIN_IN_OPEN, PIN_IN_TOGGLE, ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::U8(InTogglePinFunction::from(val.as_str()) as u8),
                PersistenceId::SpeedToFlyPinConfig,
                Echo::None,
            )
        }
    }
}

pub struct UserProfile;
const USER_1: &str = "User 1";
const USER_2: &str = "User 2";
const USER_3: &str = "User 3";
const USER_4: &str = "User 4";

impl EditableFuncs for UserProfile {
    fn name() -> &'static str {
        "User Profile"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        let s = match cm.config.user_profile {
            0 => USER_1,
            1 => USER_2,
            2 => USER_3,
            3 => USER_4,
            _ => DO_NOT_CHANGE,
        };
        Content::Enum(TString::<16>::from_str(s))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [USER_1, USER_2, USER_3, USER_4, DO_NOT_CHANGE],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
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
    }
}

pub struct VarioModeControl_;
impl EditableFuncs for VarioModeControl_ {
    fn name() -> &'static str {
        "Vario Control"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cm.control.vario_mode_control.as_str(),
        ))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [
                VARIO_MODE_CONTROL_AUTO,
                VARIO_MODE_CONTROL_PIN,
                VARIO_MODE_CONTROL_NMEA,
                "",
                "",
            ],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            let mode = VarioModeControl::from(val.as_str());
            persist::persist_set(
                cc,
                cm,
                Variant::U32(mode as u32),
                PersistenceId::VarioModeControl,
                Echo::NmeaAndCan,
            );
        }
    }
}
