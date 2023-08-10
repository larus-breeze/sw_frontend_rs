use vario_display::CoreModel;

use crate::{driver::CRxFrames, CKeyEvents, CoreController};
use defmt::*;
use bxcan::Id;

pub struct DevController {
    core_controller: CoreController,
    c_key_event: CKeyEvents, // key event queue
    c_rx_frames: CRxFrames,  // can bus rx queue
    frame_cntr: u32,
}

impl DevController {
    pub fn new(
        core_model: &mut CoreModel,
        c_key_event: CKeyEvents,
        c_rx_frames: CRxFrames,
    ) -> Self {
        let core_controller = CoreController::new(core_model);
        DevController {
            core_controller,
            c_key_event,
            c_rx_frames,
            frame_cntr: 0,
        }
    }

    pub fn tick(&mut self, core_model: &mut CoreModel) {
        while let Some(key_event) = self.c_key_event.dequeue() {
            self.core_controller.key_action(core_model, &key_event);
        }
        while let Some(frame) = self.c_rx_frames.dequeue() {
            self.core_controller.read_can_frame(core_model, &frame);
            if self.frame_cntr % 97 == 0 {
                match frame.id() {
                    Id::Standard(id) => trace!("{} Frames, Standard id {}", self.frame_cntr, id.as_raw()),
                    Id::Extended(id) => trace!("Extended id {}", id.as_raw()), // will never happen
                };
            } 
            self.frame_cntr += 1;
        }
        self.core_controller.time_action(core_model);
    }
}
