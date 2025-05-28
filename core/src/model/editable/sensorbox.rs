use super::{CmdParams, Content, EditableFuncs, F32Params, Params};
use crate::{
    controller::{persist::send_can_config_frame, CanConfigId, RemoteConfig},
    utils::TString,
    CoreController, CoreModel,
};

pub struct SensTiltRoll;
const SENS_TILT_PARAMS: Params = Params::F32(F32Params {
    min: -179.0,
    max: 359.0,
    small_inc: 0.1,
    big_inc: 1.0,
    dec_places: 1,
    unit: "°",
});

impl EditableFuncs for SensTiltRoll {
    fn name() -> &'static str {
        "Sensor Tilt Roll"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::SensTiltRoll, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        SENS_TILT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::SensTiltRoll, RemoteConfig::Set);
    }
}

pub struct SensTiltPitch;
impl EditableFuncs for SensTiltPitch {
    fn name() -> &'static str {
        "Sensor Tilt Pitch"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::SensTiltPitch, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        SENS_TILT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::SensTiltPitch, RemoteConfig::Set);
    }
}

pub struct SensTiltYaw;
impl EditableFuncs for SensTiltYaw {
    fn name() -> &'static str {
        "Sensor Tilt Yaw"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::SensTiltYaw, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        SENS_TILT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::SensTiltYaw, RemoteConfig::Set);
    }
}

pub struct PitotOffset;
const PA_PARAMS: Params = Params::F32(F32Params {
    min: -50.0,
    max: 50.0,
    small_inc: 0.1,
    big_inc: 1.0,
    dec_places: 1,
    unit: "Pa",
});

impl EditableFuncs for PitotOffset {
    fn name() -> &'static str {
        "Pitot Offset"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::PitotOffset, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        PA_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::PitotOffset, RemoteConfig::Set);
    }
}

pub struct PitotSpan;
impl EditableFuncs for PitotSpan {
    fn name() -> &'static str {
        "Pitot Span"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::PitotSpan, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.7,
            max: 1.3,
            small_inc: 0.001,
            big_inc: 0.01,
            dec_places: 3,
            unit: "",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::PitotSpan, RemoteConfig::Set);
    }
}

pub struct QnhDelta;
impl EditableFuncs for QnhDelta {
    fn name() -> &'static str {
        "QNH Delta"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::QnhDelta, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        PA_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::QnhDelta, RemoteConfig::Set);
    }
}

pub struct MagAutoCalib;
impl EditableFuncs for MagAutoCalib {
    fn name() -> &'static str {
        "Mag Auto Calib"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::MagAutoCalib, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 2.0,
            small_inc: 1.0,
            big_inc: 1.0,
            dec_places: 0,
            unit: "",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::MagAutoCalib, RemoteConfig::Set);
    }
}

pub struct VarioTc;
const TC_PARAMS: Params = Params::F32(F32Params {
    min: 1.0,
    max: 100.0,
    small_inc: 0.1,
    big_inc: 1.0,
    dec_places: 1,
    unit: "",
});

impl EditableFuncs for VarioTc {
    fn name() -> &'static str {
        "Vario TC"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::VarioTc, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        TC_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::VarioTc, RemoteConfig::Set);
    }
}

pub struct VarioIntTc;
impl EditableFuncs for VarioIntTc {
    fn name() -> &'static str {
        "Vario Avg TC"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::VarioIntTc, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        TC_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::VarioIntTc, RemoteConfig::Set);
    }
}

pub struct WindTc;
impl EditableFuncs for WindTc {
    fn name() -> &'static str {
        "Wind Tc"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::WindTc, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        TC_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::WindTc, RemoteConfig::Set);
    }
}

pub struct MeanWindTc;
impl EditableFuncs for MeanWindTc {
    fn name() -> &'static str {
        "Wind Avg TC"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::MeanWindTc, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        TC_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::MeanWindTc, RemoteConfig::Set);
    }
}

pub struct GnssConfig;
impl EditableFuncs for GnssConfig {
    fn name() -> &'static str {
        "GNSS Config"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::GnssConfig, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 1.0,
            max: 3.0,
            small_inc: 1.0,
            big_inc: 1.0,
            dec_places: 0,
            unit: "",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::GnssConfig, RemoteConfig::Set);
    }
}

pub struct AntBaselen;
const ANT_PARAMS: Params = Params::F32(F32Params {
    min: -10.0,
    max: 10.0,
    small_inc: 0.01,
    big_inc: 0.1,
    dec_places: 2,
    unit: "m",
});

impl EditableFuncs for AntBaselen {
    fn name() -> &'static str {
        "Ant Base Len"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::AntBaselen, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        ANT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::AntBaselen, RemoteConfig::Set);
    }
}

pub struct AntSlaveDown;
impl EditableFuncs for AntSlaveDown {
    fn name() -> &'static str {
        "Ant Slave Len"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::AntSlaveDown, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        ANT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::AntSlaveDown, RemoteConfig::Set);
    }
}

pub struct AntSlaveRight;
impl EditableFuncs for AntSlaveRight {
    fn name() -> &'static str {
        "Ant Slave Right"
    }

    fn content(cm: &mut CoreModel, cc: &mut CoreController) -> Content {
        send_can_config_frame(cm, cc, CanConfigId::AntSlaveRight, RemoteConfig::Get);
        Content::F32(None)
    }

    fn params() -> Params {
        ANT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::AntSlaveRight, RemoteConfig::Set);
    }
}

pub struct CmdMeas1;
const COMMAND_SENT: &str = "Command sent";

impl EditableFuncs for CmdMeas1 {
    fn name() -> &'static str {
        "Left Wing down"
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Command(TString::<16>::from_str(COMMAND_SENT))
    }

    fn params() -> Params {
        Params::Cmd(CmdParams {
            content: TString::<16>::from_str(COMMAND_SENT),
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::CmdMeasure1, RemoteConfig::Get);
    }
}

pub struct CmdMeas2;
impl EditableFuncs for CmdMeas2 {
    fn name() -> &'static str {
        "Right Wing down"
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Command(TString::<16>::from_str(COMMAND_SENT))
    }

    fn params() -> Params {
        Params::Cmd(CmdParams {
            content: TString::<16>::from_str(COMMAND_SENT),
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::CmdMeasure2, RemoteConfig::Get);
    }
}

pub struct CmdMeas3;
impl EditableFuncs for CmdMeas3 {
    fn name() -> &'static str {
        "Wings straight"
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Command(TString::<16>::from_str(COMMAND_SENT))
    }

    fn params() -> Params {
        Params::Cmd(CmdParams {
            content: TString::<16>::from_str(COMMAND_SENT),
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::CmdMeasure3, RemoteConfig::Get);
    }
}

pub struct CmdCalcOrientation;
impl EditableFuncs for CmdCalcOrientation {
    fn name() -> &'static str {
        "Calc Orientation"
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Command(TString::<16>::from_str(COMMAND_SENT))
    }

    fn params() -> Params {
        Params::Cmd(CmdParams {
            content: TString::<16>::from_str(COMMAND_SENT),
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(
            cm,
            cc,
            crate::CanConfigId::CmdCalcSensorOrientation,
            RemoteConfig::Get,
        );
    }
}

pub struct CmdFineTuneOrientation;
impl EditableFuncs for CmdFineTuneOrientation {
    fn name() -> &'static str {
        "Straight Flight"
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Command(TString::<16>::from_str(COMMAND_SENT))
    }

    fn params() -> Params {
        Params::Cmd(CmdParams {
            content: TString::<16>::from_str(COMMAND_SENT),
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(
            cm,
            cc,
            crate::CanConfigId::CmdFineTuneCalibration,
            RemoteConfig::Get,
        );
    }
}

pub struct CmdResetSensorbox;
impl EditableFuncs for CmdResetSensorbox {
    fn name() -> &'static str {
        "Reset Sensorbox"
    }

    fn content(_cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Command(TString::<16>::from_str(COMMAND_SENT))
    }

    fn params() -> Params {
        Params::Cmd(CmdParams {
            content: TString::<16>::from_str(COMMAND_SENT),
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, _content: Content) {
        send_can_config_frame(cm, cc, crate::CanConfigId::CmdReset, RemoteConfig::Get);
    }
}
