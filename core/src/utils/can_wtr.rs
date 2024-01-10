use can_dispatch::CanFrame;
use heapless::spsc::{Consumer, Producer, Queue};

pub fn can_frame_sound(frequency: u16, volume: u8, duty_cycle: u16, continuous: bool) -> CanFrame {
    CanFrame::empty_from_id(0x00)
        .push_u16(frequency)
        .push_u16(duty_cycle)
        .push_u8(volume)
        .push_u8(if continuous { 1 } else { 0 })
}

// This queue transports the can bus frames from the view component to the can tx driver.
const MAX_TX_FRAMES: usize = 10;
pub type QTxFrames = Queue<CanFrame, MAX_TX_FRAMES>;
pub type PTxFrames = Producer<'static, CanFrame, MAX_TX_FRAMES>;
pub type CTxFrames = Consumer<'static, CanFrame, MAX_TX_FRAMES>;
