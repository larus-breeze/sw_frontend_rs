use crate::{driver::QEvents, timestamp_ms, CoreController};
use corelib::{
    CPersistenceItems, CRxFrames, CoreModel, PIdleEvents, PTxFrames, basic_config::{MAX_RX_FRAMES, MAX_TX_FRAMES}
};

#[cfg(feature = "test-panic")]
use corelib::FloatToMass;
#[cfg(feature = "test-panic")]
use defmt::trace;

pub struct DevController {
    core_controller: CoreController,
    q_events: &'static QEvents,            // key event queue
    c_rx_frames: CRxFrames<MAX_RX_FRAMES>, // can bus rx queue
}

impl DevController {
    pub fn new(
        core_model: &mut CoreModel,
        q_events: &'static QEvents,
        queue_to_idle_task: PIdleEvents,
        queue_from_idle_task: CPersistenceItems,
        p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
        c_rx_frames: CRxFrames<MAX_RX_FRAMES>,
    ) -> Self {
        let core_controller = CoreController::new(
            core_model, 
            queue_to_idle_task, 
            queue_from_idle_task,
            p_tx_frames
        );
        DevController {
            core_controller,
            q_events,
            c_rx_frames,
        }
    }

    pub fn core(&mut self) -> &mut CoreController {
        &mut self.core_controller
    }

    pub fn set_ms(&mut self, time_ms: u16) {
        self.core_controller.set_ms(time_ms);
    }

    pub fn tick_1ms(&mut self, core_model: &mut CoreModel) -> bool {
        while let Some(event) = self.q_events.dequeue() {
            self.core_controller.event_handler(event, core_model);
        }
        while let Some(frame) = self.c_rx_frames.dequeue() {
            self.core_controller.read_can_frame(core_model, &frame);
        }

        let recalc = self.core_controller.tick_1ms(timestamp_ms(), core_model);

        #[cfg(feature = "test-panic")]
        {
            let cm = core_model;
            if cm.glider_data.pilot_weight > 100.9.kg() && cm.glider_data.pilot_weight < 101.1.kg()
            {
                trace!("panic!");
                panic!();
            }
            if cm.glider_data.pilot_weight > 101.9.kg() && cm.glider_data.pilot_weight < 102.1.kg()
            {
                trace!("watchdog reset");
                loop {}
            }
        }
        recalc
    }
}
