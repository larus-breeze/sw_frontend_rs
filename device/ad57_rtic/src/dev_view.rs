use crate::{
    driver::{DevDuration, DevInstant},
    DevDisplay,
};
use corelib::*;

pub struct DevView {
    core_view: CoreView<DevDisplay>,
    next_wake_up: DevInstant,
}

impl DevView {
    pub fn new(display: DevDisplay) -> Self {
        let core_view = CoreView::new(display);
        DevView {
            core_view,
            next_wake_up: DevInstant::from_ticks(0),
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
        self.core_view.draw(core_model)
    }

}
