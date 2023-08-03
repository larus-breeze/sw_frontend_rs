use crate::{
    driver::{DevDuration, DevInstant, PTxFrames},
    DevDisplay,
};
use bxcan::{Frame, StandardId};
use vario_display::*;
use defmt::*;

pub struct DevView {
    core_view: CoreView<DevDisplay>,
    next_wake_up: DevInstant,
    p_tx_frames: PTxFrames, // can bus tx queue
    frame_count: u16,
    cs: f32,
}

impl DevView {
    pub fn new(display: DevDisplay, now: DevInstant, p_tx_frames: PTxFrames) -> Self {
        let next_wake_up = now + DevDuration::secs(1);
        let core_view = CoreView::new(display);
        DevView {
            core_view,
            next_wake_up,
            p_tx_frames,
            frame_count: 0,
            cs: 0.0,
        }
    }

    pub fn wake_up_at(&mut self) -> DevInstant {
        self.next_wake_up += DevDuration::millis((1000 / FRAME_RATE) as u64);
        self.next_wake_up
    }

    pub fn tick(&mut self, core_model: &mut CoreModel) -> Result<(), CoreError> {
        let _ = self.p_tx_frames.capacity();
        if self.cs != core_model.measured.climb_rate.0 {
            self.cs = core_model.measured.climb_rate.0;
            self.send_frame();
        }
        self.core_view.draw(core_model)
    }

    fn send_frame(&mut self) {
        let frame = Frame::new_data(StandardId::new(self.frame_count).unwrap(), []);
        let _ = self.p_tx_frames.enqueue(frame);
        self.frame_count += 1;
        trace!("Can paket enqueued")
    }

    pub fn can_activate(&self) -> bool {
        self.p_tx_frames.len() > 0
    }
}
