use byteorder::{ByteOrder, LittleEndian as LE};
use heapless::spsc::{Consumer, Producer, Queue};

pub fn can_frame_sound(frequency: u16, volume: u8, duty_cycle: u16, continuous: bool) -> CanFrame {
    CanFrame::empty_from_id(0x00)
        .push_u16(frequency)
        .push_u16(duty_cycle)
        .push_u8(volume)
        .push_u8(if continuous { 1 } else { 0 })
}

#[derive(Copy, Clone)]
pub struct CanFrame {
    id: u16,
    rtr: bool,
    len: u8,
    data: [u8; 8],
}

#[allow(unused)]
impl CanFrame {
    pub fn empty_from_id(id: u16) -> Self {
        CanFrame {
            id,
            rtr: false,
            len: 0,
            data: [0u8; 8],
        }
    }

    pub fn remote_trans_rq(id: u16, len: u8) -> Self {
        CanFrame {
            id,
            rtr: true,
            len,
            data: [0u8; 8],
        }
    }

    pub fn from_slice(id: u16, src: &[u8]) -> Self {
        let mut data = [0u8; 8];
        let len = src.len();
        data[..len].copy_from_slice(&src[..len]);
        CanFrame {
            id,
            rtr: false,
            len: len as u8,
            data,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn is_rtr(&self) -> bool {
        self.rtr
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }

    pub fn dlc(&self) -> u8 {
        self.len
    }

    fn push_u32(mut self, val: u32) -> Self {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_u32(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    fn push_u16(mut self, val: u16) -> Self {
        let idx = self.len as usize;
        self.len += 2;
        LE::write_u16(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    fn push_u8(mut self, val: u8) -> Self {
        self.data[self.len as usize] = val;
        self.len += 1;
        self
    }

    fn push_i32(mut self, val: i32) -> Self {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_i32(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    fn push_i16(mut self, val: i16) -> Self {
        let idx = self.len as usize;
        self.len += 2;
        LE::write_i16(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    fn push_i8(mut self, val: i8) -> Self {
        self.data[self.len as usize] = val as u8;
        self.len += 1;
        self
    }

    fn push_f32(mut self, val: f32) -> Self {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_f32(&mut self.data[idx..(self.len as usize)], val);
        self
    }
}

// This queue transports the can bus frames from the view component to the can tx driver.
const MAX_TX_FRAMES: usize = 10;
pub type QTxFrames = Queue<CanFrame, MAX_TX_FRAMES>;
pub type PTxFrames = Producer<'static, CanFrame, MAX_TX_FRAMES>;
pub type CTxFrames = Consumer<'static, CanFrame, MAX_TX_FRAMES>;
