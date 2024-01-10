use can_dispatch::*;

const OBJECT_ID: u16 = 4;

pub fn can_frame_sound(frequency: u16, volume: u8, duty_cycle: u16, continuous: bool) -> Frame {
    Frame::specific(
        CanFrame::empty_from_id(0x00)
            .push_u16(frequency)
            .push_u16(duty_cycle)
            .push_u8(volume)
            .push_u8(if continuous { 1 } else { 0 }),
        0,
        OBJECT_ID,
    )
}
