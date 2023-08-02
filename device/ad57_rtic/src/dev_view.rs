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
    pub fn new(display: DevDisplay, now: DevInstant, p_tx_frames: PTxFrames) -> Self {
        let next_wake_up = now + DevDuration::secs(1);
        let core_view = CoreView::new(display);
        DevView {
            core_view,
            next_wake_up,
            p_tx_frames,
        }
    }

    pub fn wake_up_at(&mut self) -> DevInstant {
        self.next_wake_up += DevDuration::millis((1000 / FRAME_RATE) as u64);
        self.next_wake_up
    }

    pub fn tick(&mut self, core_model: &mut CoreModel) -> Result<(), CoreError> {
        let _ = self.p_tx_frames.capacity();
        self.core_view.draw(core_model)
    }
}
