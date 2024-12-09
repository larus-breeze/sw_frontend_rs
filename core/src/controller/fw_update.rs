use crate::{model::CoreModel, utils::DeviceEvent};

pub struct SwUpdateController {}

impl SwUpdateController {
    pub fn new() -> Self {
        SwUpdateController {}
    }

    pub fn device_action(&mut self, core_model: &mut CoreModel, _device_event: &DeviceEvent) {
        core_model.control.firmware_update_state = DeviceEvent::UploadInProgress;
    }
}
