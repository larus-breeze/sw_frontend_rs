use vario_display::CoreModel;
use bxcan::Id;

use crate:: {
    CKeyEvents, CoreController,
    driver::CRxFrames,
};
use defmt::*;

pub struct DevController {
    core_controller: CoreController,
    c_key_event: CKeyEvents, // key event queue
    c_rx_frames: CRxFrames, // can bus rx queue
}

impl DevController {
    pub fn new(core_model: &mut CoreModel, c_key_event: CKeyEvents, c_rx_frames: CRxFrames) -> Self {
        let core_controller = CoreController::new(core_model);
        DevController {
            core_controller,
            c_key_event,
            c_rx_frames,
        }
    }

    pub fn tick(&mut self, core_model: &mut CoreModel) {
        while let Some(key_event) = self.c_key_event.dequeue() {
            self.core_controller.key_action(core_model, &key_event);
        }
        while let Some(frame) = self.c_rx_frames.dequeue() {
            match frame.id() {
                Id::Standard(id) => trace!("Standard id {}", id.as_raw()),
                Id::Extended(id) => trace!("Extended id {}", id.as_raw()), 
            }
        }
        self.core_controller.time_action(core_model);
    }

}
