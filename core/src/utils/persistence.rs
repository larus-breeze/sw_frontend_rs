use heapless::spsc::{Queue, Producer, Consumer};


#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum PersistenceId {
    MacCready = 0,
    Qnh = 1,
    Ballast = 2,
    Bugs = 3,
}

#[cfg(feature = "eeprom_size_8192")]
pub mod eeprom {
    pub const SIZE: u32 = 8192;
    pub const IDENTIFICATION_BLOCK: u32 = 0;
    pub const DAT: u32 = 32; // Data allocation table
    pub const DAT_LEN: u32 = SIZE/8/4;
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
            unsafe{core::mem::transmute::<u16, PersistenceId>(src)}
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PersistenceItem {
    pub id: PersistenceId,
    pub dat_bit: bool,  // Data allocation table
    pub data: [u8; 4],
}

impl PersistenceItem {
    pub fn from_f32(id: PersistenceId, value: f32) -> Self {
        PersistenceItem { id, dat_bit: true, data: value.to_le_bytes() }
    }

    pub fn to_f32(&self) -> f32 {
        f32::from_le_bytes(self.data)
    }
}

// This queue transports the configuration PersItems from controller to the idle loop.
const MAX_PERS_ITEMS: usize = 20;
pub type QPersistenceItems = Queue<PersistenceItem, MAX_PERS_ITEMS>;
pub type PPersistenceItems = Producer<'static, PersistenceItem, MAX_PERS_ITEMS>;
pub type CPersistenceItems = Consumer<'static, PersistenceItem, MAX_PERS_ITEMS>;
