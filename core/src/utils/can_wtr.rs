use bxcan::Frame;
use byteorder::{ByteOrder, LittleEndian as LE};
use embedded_hal::can::{Id, StandardId};
use heapless::spsc::{Consumer, Producer, Queue};

pub struct CanFrame {
    id: u16,
    len: u8,
    data: [u8; 8],
}

#[allow(unused)]
impl CanFrame {
    pub fn empty_from_id(id: u16) -> Self {
        CanFrame {
            id,
            len: 0,
            data: [0u8; 8],
        }
    }

    pub fn from_slice(id: u16, src: &[u8]) -> Self {
        let mut data = [0u8; 8];
        let len = src.len();
        data[..len].copy_from_slice(&src[..len]);
        CanFrame {
            id,
            len: len as u8,
            data,
        }
    }

    fn push_u32(&mut self, val: u32) {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_u32(&mut self.data[idx..(self.len as usize)], val);
    }

    fn push_u16(&mut self, val: u16) {
        let idx = self.len as usize;
        self.len += 2;
        LE::write_u16(&mut self.data[idx..(self.len as usize)], val);
    }

    fn push_u8(&mut self, val: u8) {
        self.data[self.len as usize] = val;
        self.len += 1;
    }

    fn push_i32(&mut self, val: i32) {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_i32(&mut self.data[idx..(self.len as usize)], val);
    }

    fn push_i16(&mut self, val: i16) {
        let idx = self.len as usize;
        self.len += 2;
        LE::write_i16(&mut self.data[idx..(self.len as usize)], val);
    }

    fn push_i8(&mut self, val: i8) {
        self.data[self.len as usize] = val as u8;
        self.len += 1;
    }

    fn push_f32(&mut self, val: f32) {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_f32(&mut self.data[idx..(self.len as usize)], val);
    }
}

impl embedded_hal::can::Frame for CanFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        let id = match id.into() {
            Id::Standard(id) => id,
            Id::Extended(_id) => return None,
        };
        if data.len() > 8 {
            return None;
        }

        let mut bytes = [0; 8];
        bytes[..data.len()].copy_from_slice(data);
        let len = data.len() as u8;

        Some(Self {
            id: id.as_raw(),
            len,
            data: bytes,
        })
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        None
    }

    fn is_extended(&self) -> bool {
        false
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Id::Standard(StandardId::new(self.id).unwrap())
    }

    fn dlc(&self) -> usize {
        self.len as usize
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}

// This queue transports the can bus frames from the view component to the can tx driver.
const MAX_TX_FRAMES: usize = 10;
pub type QTxFrames = Queue<Frame, MAX_TX_FRAMES>;
pub type PTxFrames = Producer<'static, Frame, MAX_TX_FRAMES>;
pub type CTxFrames = Consumer<'static, Frame, MAX_TX_FRAMES>;
