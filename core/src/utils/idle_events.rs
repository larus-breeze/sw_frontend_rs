use heapless::spsc::{Consumer, Producer, Queue};
use crate::CoreError;

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum IdleEvent {
    EepromItem(PersistenceItem),
    SdCardItem(SdCardCmd),
    FeedTheDog,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SdCardCmd {
    SwUpdateAccepted,
    SwUpdateCanceled,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PersistenceId {
    DoNotStore = 65535,
    Volume = 0,
    McCready = 1,
    WaterBallast = 2,
    PilotWeight = 3,
    Glider = 4,
}

#[cfg(feature = "eeprom_size_8192")]
pub mod eeprom {
    pub const SIZE: u32 = 8192;
    pub const IDENTIFICATION_BLOCK: u32 = 0;
    pub const DAT: u32 = 32; // Data allocation table
    pub const DAT_LEN: u32 = SIZE / 8 / 4;
    pub const DATA_STORAGE: u32 = DAT + DAT_LEN;
    pub const MAX_ITEM_COUNT: u32 = (SIZE - DATA_STORAGE) / 4;
    pub const MAGIC: [u8; 8] = [0x1e, 0xf9, 0xb4, 0xaf, 0x22, 0xe1, 0xe5, 0xeb];
}

pub enum EepromTopic {
    ConfigValues,
}

pub const CONFIG_VALUES_START: u16 = 0;
pub const CONFIG_VALUES_END: u16 = 256;

impl From<u16> for PersistenceId {
    fn from(src: u16) -> Self {
        if src < eeprom::MAX_ITEM_COUNT as u16 {
            // Safety: Only valid or possible values are transmuted
            unsafe { core::mem::transmute::<u16, PersistenceId>(src) }
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PersistenceItem {
    pub id: PersistenceId,
    pub dat_bit: bool, // Data allocation table
    pub data: [u8; 4],
}

impl PersistenceItem {
    pub fn do_not_store() -> Self {
        PersistenceItem {
            id: PersistenceId::DoNotStore,
            dat_bit: false,
            data: [0, 0, 0, 0],
        }
    }

    pub fn from_i8(id: PersistenceId, value: i8) -> Self {
        PersistenceItem {
            id,
            dat_bit: true,
            data: (value as i32).to_le_bytes(),
        }
    }

    pub fn from_i32(id: PersistenceId, value: i32) -> Self {
        PersistenceItem {
            id,
            dat_bit: true,
            data: value.to_le_bytes(),
        }
    }

    pub fn from_u8(id: PersistenceId, value: u8) -> Self {
        PersistenceItem {
            id,
            dat_bit: true,
            data: (value as u32).to_le_bytes(),
        }
    }

    pub fn from_u16(id: PersistenceId, value: u16) -> Self {
        PersistenceItem {
            id,
            dat_bit: true,
            data: (value as u32).to_le_bytes(),
        }
    }

    pub fn from_u32(id: PersistenceId, value: u32) -> Self {
        PersistenceItem {
            id,
            dat_bit: true,
            data: value.to_le_bytes(),
        }
    }

    pub fn from_f32(id: PersistenceId, value: f32) -> Self {
        PersistenceItem {
            id,
            dat_bit: true,
            data: value.to_le_bytes(),
        }
    }

    pub fn to_i8(&self) -> i8 {
        i32::from_le_bytes(self.data) as i8
    }

    pub fn to_i32(&self) -> i32 {
        i32::from_le_bytes(self.data)
    }

    pub fn to_u8(&self) -> u8 {
        i32::from_le_bytes(self.data) as u8
    }

    pub fn to_u16(&self) -> u16 {
        i32::from_le_bytes(self.data) as u16
    }

    pub fn to_u32(&self) -> u32 {
        u32::from_le_bytes(self.data)
    }

    pub fn to_f32(&self) -> f32 {
        f32::from_le_bytes(self.data)
    }
}

// This queue transports the configuration PersItems from controller to the idle loop.
const MAX_IDLE_EVENTS: usize = 20;
pub type QIdleEvents = Queue<IdleEvent, MAX_IDLE_EVENTS>;
pub type PIdleEvents = Producer<'static, IdleEvent, MAX_IDLE_EVENTS>;
pub type CIdleEvents = Consumer<'static, IdleEvent, MAX_IDLE_EVENTS>;

pub trait EepromTrait {
    /// Write a single byte in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    fn write_byte(&mut self, address: u32, data: u8) -> Result<(), CoreError>;

    /// Write up to a page starting in an address.
    ///
    /// The maximum amount of data that can be written depends on the page
    /// size of the device and its overall capacity. If too much data is passed,
    /// the error `Error::TooMuchData` will be returned.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), CoreError>;

    /// Read a single byte from an address.
    fn read_byte(&mut self, address: u32) -> Result<u8, CoreError>;

    /// Read starting in an address as many bytes as necessary to fill the data array provided.
    fn read_data(&mut self, address: u32, data: &mut [u8]) -> Result<(), CoreError>;

}
