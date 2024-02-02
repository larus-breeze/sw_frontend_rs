use crate::{
    controller::Result,
    model::CoreModel,
    utils::{DeviceEvent, IdleEvent, KeyEvent},
    SdCardCmd,
};

pub struct SwUpdateController {}

impl SwUpdateController {
    pub fn new() -> Self {
        SwUpdateController {}
    }

    pub fn device_action(&mut self, core_model: &mut CoreModel, _device_event: &DeviceEvent) {
        core_model.control.firmware_update_state = DeviceEvent::UploadInProgress;
        core_model.send_idle_event(IdleEvent::SdCardItem(SdCardCmd::SwUpdateAccepted))
    }

    pub fn key_action(&mut self, _core_model: &mut CoreModel, _key_event: &KeyEvent) -> Result {
        Result::Nothing
    }
}
