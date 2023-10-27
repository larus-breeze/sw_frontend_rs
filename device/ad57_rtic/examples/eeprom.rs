#![no_main]
#![no_std]
use core::convert::From;

use {defmt_rtt as _, panic_probe as _};
use defmt::trace;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    {i2c::I2c, pac::I2C1},
};
use eeprom24x::{
    SlaveAddr, Eeprom24x,
    page_size::B32, addr_size::TwoBytes,
};

pub fn delay_ms(millis: u32) {
    let cycles = millis * 168_000;
    cortex_m::asm::delay(cycles)
}

#[derive(Debug)]
enum Error {
    EepromOrI2c1,
    NoItemAvailable,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum PersId {
    MacCready = 0,
    Qnh = 1,
    Ballast = 2,
    Bugs = 3,
}

impl From<u16> for PersId {
    fn from(id: u16) -> Self {
        unsafe{core::mem::transmute::<u16, PersId>(id)}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PersistentItem {
    id: PersId,
    dat_bit: bool,  // Data allocation table
    data: [u8; 4],
}

impl PersistentItem {
    pub fn from_f32(id: PersId, value: f32) -> Self {
        PersistentItem { id, dat_bit: true, data: value.to_le_bytes() }
    }

    pub fn to_f32(&self) -> f32 {
        f32::from_le_bytes(self.data)
    }
}

const EEPROM_SIZE: u32 = 8192;
const EEPROM_IDENTIFICATION_BLOCK: u32 = 0;
const EEPROM_DAT: u32 = 32; // Data allocation table
const EEPROM_DAT_LEN: u32 = EEPROM_SIZE/8/4;
const EEPROM_DATA_STORAGE: u32 = EEPROM_DAT + EEPROM_DAT_LEN;
const EEPROM_MAX_ITEM_COUNT: u32 = (EEPROM_SIZE - EEPROM_DATA_STORAGE) / 4;
const EEPROM_MAGIC: [u8; 8] = [0x1e, 0xf9, 0xb4, 0xaf, 0x22, 0xe1, 0xe5, 0xeb];

enum PersistType {
    ConfigValues,
}

struct Persistence {
    eeprom: Eeprom24x<I2c<I2C1>, B32, TwoBytes>,
}

/// Store configuration data in an EEPROM
/// 
/// The storage of configuration and other data, which should bridge a power failure, is done in an 
/// EEPROM, which is connected via an i2c interface.  Conceptually, the memory area is divided into 
/// three areas:
/// 
/// - Identification of the initialization with a signature (Identification Block).
/// - Table of contents (data allocation table)
/// - Data storage
/// 
/// The data are divided into four-byte blocks in the data store. They are addressed with Ids, 
/// where each Id addresses one such block. Each ID is assigned to a bit in the table of contents.
/// 
/// When a block is written, the corresponding bit in the table of contents is written first and 
/// then the block. When reading it is the other way round. First it is checked whether the 
/// corresponding ID is stored. If this is not the case, an error message is issued, otherwise 
/// the block is returned.
/// 
/// Another function is the possibility to define topic areas and to assign them to Id areas. 
/// An iterator makes it possible to search out all stored data of such a topic area. This can be 
/// useful to load all initialization data, log eventc, etc..
/// 
/// Summarizing features of the solution:
/// - Individual data is stored one by one
/// - When changes are made, only these are written to the EEPROM
/// - The solution is extensible and version independent - new data points can always be added 
///   without losing the old data
/// - The solution is independent of a specific EEPROM type.
/// - At the moment it is limited to data up to 4 bytes, but can be extended to data with more 
///   blocks, if needed.
/// - The blocks of this structure are well suited to be transported via queues.
/// - The data to be stored must be defined individually
/// 
#[allow(dead_code)]
impl Persistence {
    /// Create a Persistence Instance
    pub fn new(i2c: I2c<I2C1>) -> Result<Self, Error> {
        let addr = SlaveAddr::default();
        let mut eeprom = Eeprom24x::new_24x64(i2c, addr);
        let mut magic = [0_u8; 8];
        eeprom.read_data(EEPROM_IDENTIFICATION_BLOCK, &mut magic).map_err(|_| Error::EepromOrI2c1)?;
        delay_ms(10);
        if magic != EEPROM_MAGIC {
            // Write magic number
            eeprom.write_page(EEPROM_IDENTIFICATION_BLOCK, &EEPROM_MAGIC).map_err(|_| Error::EepromOrI2c1)?;
            delay_ms(10);

            // Clear DAT
            let mut address = EEPROM_DAT;
            let data = [0_u8; 8];
            while address < (EEPROM_DAT + EEPROM_DAT_LEN) {
                eeprom.write_page(address, &data).map_err(|_| Error::EepromOrI2c1)?;
                delay_ms(10);
                address += 8;
            }
            trace!("Initialize DAT");
        }
        Ok(Persistence { eeprom })
    }

    /// Write a PersistentItem into the data store
    /// 
    /// The data is stored at the desired location defined by the ID. An entry is made in the data 
    /// allocation table (DAT), if desired (dat_bit in PersitentItem). 
    pub fn write_item(&mut self, item: PersistentItem) -> Result <(), Error> {
        self.id_in_range(item.id)?;
        let address = EEPROM_DATA_STORAGE + item.id as u32 * 4;
        if item.dat_bit {
            self.set_id(item.id)?;
        }
        self.eeprom.write_page(address, &item.data).map_err(|_| Error::EepromOrI2c1)?;
        delay_ms(10);
        Ok(())
    }

    /// Read data from storage - do not check the DAT
    pub fn read_item_unchecked(&mut self, id: PersId) -> Result<PersistentItem, Error> {
        let address = EEPROM_DATA_STORAGE + id as u32 * 4;
        let mut data = [0_u8; 4];
        self.eeprom.read_data(address, &mut data).map_err(|_| Error::EepromOrI2c1)?;
        Ok(PersistentItem { id, dat_bit: false, data })
    }

    /// Read data from storage - return error if DAT bit is not set
    pub fn read_item(&mut self, id: PersId) -> Result<PersistentItem, Error> {
        self.id_in_range(id)?;
        if self.test_id(id) {
            self.read_item_unchecked(id)
        } else {
            Err(Error::NoItemAvailable)
        }
    }

    /// Returns an iterator to the desired topic area
    pub fn iter_over(&mut self, p_type: PersistType) -> PersistenceIterator {
        let (start_id, end_id) = match p_type {
            PersistType::ConfigValues => (0, 256),
        };
        PersistenceIterator::new(start_id, end_id, self)
    }

    /// Returns a byte of the DAT
    fn read_bitfield_byte(&mut self, adr: u32) -> Result<u8, Error> {
        let byte_adr = EEPROM_DAT + adr;
        self.eeprom.read_byte(byte_adr).map_err(|_| Error::EepromOrI2c1)
    }

    /// Tests a id, if coresponing dat_bit is set
    fn test_id(&mut self, id: PersId) -> bool {
        if self.id_in_range(id).is_err() {
            return false
        };
        let byte_adr = EEPROM_DAT + (id as u32) / 8;
        let bit_pattern: u8 = 1 << (id as u32) % 8;
        match self.eeprom.read_byte(byte_adr) {
            Ok(found) => (found & bit_pattern) != 0,
            Err(_) => false,
        }
    }

    /// Set dat_bit in table of contentspub fn iter_over(&mut self, p_type: PersistType) -> PersistenceIterator
    fn set_id(&mut self, id: PersId) -> Result<(), Error> {
        self.id_in_range(id)?;
        let byte_adr = EEPROM_DAT + (id as u32) / 8;
        let bit_pattern: u8 = 1 << (id as u32) % 8;
        let found = self.eeprom.read_byte(byte_adr).map_err(|_| Error::EepromOrI2c1)?;
        let target = found | bit_pattern;
        if found != target {
            self.eeprom.write_byte(byte_adr, target).map_err(|_| Error::EepromOrI2c1)?;
            delay_ms(10);
        }
        Ok(())
    }

    /// Tests, if id is in valid range
    fn id_in_range(&self, id: PersId) -> Result<(), Error> {
        if id as u32 >= EEPROM_MAX_ITEM_COUNT {
            Err(Error::NoItemAvailable)
        } else {
            Ok(())
        }
    }
}

/// Helper struct for Iteration
struct PersistenceIterator<'a> {
    cur_id: u16,
    end_id: u16, 
    cur_byte: u8,
    persistence: &'a mut Persistence,
}

impl <'a>PersistenceIterator<'a> {
    /// Creates a iteration helper struct
    pub fn new(start_id: u16, end_id: u16, persistence: &'a mut Persistence) -> Self {
        PersistenceIterator { 
            cur_id: start_id, 
            end_id,
            cur_byte: 0, 
            persistence }
    }
}

/// The iterator himself
impl Iterator for PersistenceIterator<'_> {
    type Item = PersistentItem;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_id < self.end_id {
            if self.cur_id % 8 == 0 {
                self.cur_byte = self.persistence.read_bitfield_byte((self.cur_id / 8) as u32).unwrap();
            }
            let cur_bit = 0x01 << (self.cur_id % 8);
            let id_exists = (cur_bit & self.cur_byte) != 0;
    
            if id_exists {
                let r = self.persistence.read_item_unchecked(self.cur_id.into()).unwrap();
                self.cur_id += 1;
                return Some(r)
            } else {
                self.cur_id += 1;
            }
        }
        None
    }
}

#[entry]
fn main() -> ! {
    // Setup clocks
    let _cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    trace!("init");

    let clocks = rcc.cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    // Setup LED
    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb4.into_push_pull_output();

    let scl: stm32f4xx_hal::gpio::Pin<'B', 6> = gpiob.pb6.internal_pull_up(true);
    let sda = gpiob.pb7.internal_pull_up(true);
    let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);

    let mut persist = Persistence::new(i2c).unwrap();

    for item in persist.iter_over(PersistType::ConfigValues) {
        trace!("Item found {} {}", item.id as u32, item.to_f32());
    }

    persist.write_item(PersistentItem::from_f32(PersId::Ballast, 1.1)).unwrap();
    persist.write_item(PersistentItem::from_f32(PersId::Bugs, 1.0)).unwrap();
    persist.write_item(PersistentItem::from_f32(PersId::MacCready, 1.3)).unwrap();
    persist.write_item(PersistentItem::from_f32(PersId::Qnh, 1013.2)).unwrap();

    let mut state = false;


    loop {
        if state {
            led.set_high();
            state = false;
        } else {
            led.set_low();
            state = true;
        }
        delay_ms(1000);
    }
}

