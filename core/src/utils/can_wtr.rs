use byteorder::{ByteOrder, LittleEndian as LE};
use can_dispatch::*;
use crate::{GenericId, SysConfigId, SysValueId};


const OBJECT_ID: u16 = 4;
const OBJECT_ID_GEN: u16 = 0;

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

pub fn can_frame_heartbeat(uuid: u32) -> Frame {
    Frame::generic(
        CanFrame::empty_from_id(0x00)
            .push_u16(OBJECT_ID)
            .push_u16(OBJECT_ID_GEN)
            .push_u32(uuid),
        GenericId::Heartbeat as u16,
    )
}

pub fn can_frame_sys_config(config_id: SysConfigId, sys_value: SysValueId) -> Frame {
    let mut data = [0u8; 6];
    match sys_value {
        SysValueId::U8(b) => data[0] = b,
        SysValueId::F32(f) => LE::write_f32(&mut data[2..6], f),
    }

    Frame::generic(
        CanFrame::empty_from_id(0)
            .push_u16(config_id as u16)
            .push_slice(&data),
        GenericId::SetSysSetting as u16,
    )
}
