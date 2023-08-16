use crate::{
    driver::{DevDuration, DevInstant, PTxFrames},
    DevDisplay,
};
use vario_display::*;

pub struct DevView {
    core_view: CoreView<DevDisplay>,
    next_wake_up: DevInstant,
    p_tx_frames: PTxFrames, // can bus tx queue
}

impl DevView {
    pub fn new(display: DevDisplay, p_tx_frames: PTxFrames) -> Self {
        let core_view = CoreView::new(display);
        DevView {
            core_view,
            next_wake_up: DevInstant::from_ticks(0),
            p_tx_frames,
        }
    }

    pub fn wake_up_at(&mut self) -> DevInstant {
        self.next_wake_up += DevDuration::millis((1000 / FRAME_RATE) as u64);
        self.next_wake_up
    }

    pub fn setup_timer(&mut self, now: DevInstant) {
        self.next_wake_up = now;
    }

    pub fn tick(&mut self, core_model: &mut CoreModel) -> Result<(), CoreError> {
        let _ = self.p_tx_frames.capacity();
        self.core_view.draw(core_model)
    }

    /*fn send_frame(&mut self) {
        let frame = Frame::new_data(StandardId::new(self.frame_count).unwrap(), []);
        let _ = self.p_tx_frames.enqueue(frame);
        self.frame_count += 1;
        trace!("Can paket enqueued")
    }*/

    pub fn can_activate(&self) -> bool {
        self.p_tx_frames.len() > 0
    }
}
