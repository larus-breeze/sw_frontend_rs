use corelib::{basic_config::MAX_RX_FRAMES, CRxFrames, CoreModel, Event}; // sensor

use crate::{driver::QEvents, CoreController};

pub struct DevController {
    core_controller: CoreController,
    q_events: &'static QEvents,            // key event queue
    c_rx_frames: CRxFrames<MAX_RX_FRAMES>, // can bus rx queue
}

impl DevController {
    pub fn new(
        core_model: &mut CoreModel,
        q_events: &'static QEvents,
        c_rx_frames: CRxFrames<MAX_RX_FRAMES>,
    ) -> Self {
        let core_controller = CoreController::new(core_model);
        DevController {
            core_controller,
            q_events,
            c_rx_frames,
        }
    }

    pub fn tick(&mut self, core_model: &mut CoreModel) {
        while let Some(event) = self.q_events.dequeue() {
            match event {
                Event::KeyItem(key_event) => {
                    self.core_controller.key_action(core_model, &key_event)
                }
                Event::DeviceItem(device_event) => self
                    .core_controller
                    .device_action(core_model, &device_event),
            }
        }
        while let Some(frame) = self.c_rx_frames.dequeue() {
            self.core_controller.read_can_frame(core_model, &frame);
        }

        self.core_controller.time_action(core_model);
    }
}
