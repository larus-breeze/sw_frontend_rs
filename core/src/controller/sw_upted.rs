use crate::{
    utils::{KeyEvent, StorageItem, DeviceEvent},
    model::CoreModel,
    controller::Result, SdCardCmd,
};

pub struct SwUpdateController {
}

impl SwUpdateController {
    pub fn new() -> Self {
        SwUpdateController {  }
    }

    pub fn device_action(&mut self, core_model: &mut CoreModel, device_event: &DeviceEvent) {
        core_model.control.firmware_update_state = *device_event;
    }


    pub fn key_action(&mut self, core_model: &mut CoreModel, key_event: &KeyEvent) -> Result {
        match core_model.control.firmware_update_state {
            DeviceEvent::FwAvailable(_) => {
                match key_event {
                    KeyEvent::Btn1 => 
                        core_model.storage_item(StorageItem::SdCardItem(SdCardCmd::SwUpdateAccepted)),
                    _ => {
                        core_model.config.display_active = core_model.config.last_display_active;
                        core_model.storage_item(StorageItem::SdCardItem(SdCardCmd::SwUpdateCanceld));
                    }
                }
            },
            _ => (),
        }
        Result::Nothing
    }
}


