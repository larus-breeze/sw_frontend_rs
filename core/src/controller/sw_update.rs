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

    pub fn device_action(&mut self, core_model: &mut CoreModel, device_event: &DeviceEvent) {
        core_model.control.firmware_update_state = *device_event;
    }

    pub fn key_action(&mut self, core_model: &mut CoreModel, key_event: &KeyEvent) -> Result {
        if let DeviceEvent::FwAvailable(_) = core_model.control.firmware_update_state {
            match key_event {
                KeyEvent::Btn1 => {
                    core_model.send_idle_event(IdleEvent::SdCardItem(SdCardCmd::SwUpdateAccepted))
                }
                _ => {
                    core_model.config.display_active = core_model.config.last_display_active;
                    core_model.send_idle_event(IdleEvent::SdCardItem(SdCardCmd::SwUpdateCanceled));
                }
            }
        }
        Result::Nothing
    }
}
