use crate::{
    driver::{DevDuration, DevInstant},
    Display,
};
use corelib::*;

pub struct DevView {
    core_view: CoreView<Display>,
    next_wake_up: DevInstant,
}

impl DevView {
    pub fn new(display: Display, core_model: &CoreModel) -> Self {
        let core_view = CoreView::new(display, core_model);
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

    pub fn core(&mut self) -> &mut CoreView<Display> {
        &mut self.core_view
    }
}
