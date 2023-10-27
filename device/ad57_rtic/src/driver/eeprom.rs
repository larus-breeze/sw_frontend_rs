use defmt::trace;

use stm32f4xx_hal::{
    {i2c::I2c, pac::I2C1},
};
use eeprom24x::{
    SlaveAddr, Eeprom24x,
    page_size::B32, addr_size::TwoBytes,
};
use vario_display::{
    eeprom, PersistenceId, PersistenceItem, 
    EepromTopic, CONFIG_VALUES_START, CONFIG_VALUES_END
};
use crate::{
    driver::delay_ms,
    Error,
};

pub struct Eeprom {
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
impl Eeprom {
    /// Create a Persistence Instance
    pub fn new(i2c: I2c<I2C1>) -> Result<Self, Error> {
        let addr = SlaveAddr::default();
        let mut eeprom = Eeprom24x::new_24x64(i2c, addr);
        let mut magic = [0_u8; 8];
        eeprom.read_data(eeprom::IDENTIFICATION_BLOCK, &mut magic).map_err(|_| Error::EepromOrI2c1)?;
        delay_ms(10);
        if magic != eeprom::MAGIC {
            // Write magic number
            eeprom.write_page(eeprom::IDENTIFICATION_BLOCK, &eeprom::MAGIC).map_err(|_| Error::EepromOrI2c1)?;
            delay_ms(10);

            // Clear DAT
            let mut address = eeprom::DAT;
            let data = [0_u8; 8];
            while address < (eeprom::DAT + eeprom::DAT_LEN) {
                eeprom.write_page(address, &data).map_err(|_| Error::EepromOrI2c1)?;
                delay_ms(10);
                address += 8;
            }
            trace!("Initialize DAT");
        }
        Ok(Eeprom { eeprom })
    }

    /// Write a PersistentItem into the data store
    /// 
    /// The data is stored at the desired location defined by the ID. An entry is made in the data 
    /// allocation table (DAT), if desired (dat_bit in PersitentItem). 
    pub fn write_item(&mut self, item: PersistenceItem) -> Result <(), Error> {
        if item.id == PersistenceId::DoNotStore {
            return Ok(())
        }
        let address = eeprom::DATA_STORAGE + item.id as u32 * 4;
        if item.dat_bit {
            self.set_id(item.id)?;
        }
        self.eeprom.write_page(address, &item.data).map_err(|_| Error::EepromOrI2c1)?;
        delay_ms(10);
        Ok(())
    }

    /// Read data from storage - do not check the DAT
    pub fn read_item_unchecked(&mut self, id: PersistenceId) -> Result<PersistenceItem, Error> {
        let address = eeprom::DATA_STORAGE + id as u32 * 4;
        let mut data = [0_u8; 4];
        self.eeprom.read_data(address, &mut data).map_err(|_| Error::EepromOrI2c1)?;
        Ok(PersistenceItem { id, dat_bit: false, data })
    }

    /// Read data from storage - return error if DAT bit is not set
    pub fn read_item(&mut self, id: PersistenceId) -> Result<PersistenceItem, Error> {
        if self.test_id(id) {
            self.read_item_unchecked(id)
        } else {
            Err(Error::NoItemAvailable)
        }
    }

    /// Returns an iterator to the desired topic area
    pub fn iter_over(&mut self, p_type: EepromTopic) -> PersistenceIterator {
        let (start_id, end_id) = match p_type {
            EepromTopic::ConfigValues => (CONFIG_VALUES_START, CONFIG_VALUES_END),
        };
        PersistenceIterator::new(start_id, end_id, self)
    }

    /// Returns a byte of the DAT
    fn read_bitfield_byte(&mut self, adr: u32) -> Result<u8, Error> {
        let byte_adr = eeprom::DAT + adr;
        self.eeprom.read_byte(byte_adr).map_err(|_| Error::EepromOrI2c1)
    }

    /// Tests a id, if coresponing dat_bit is set
    fn test_id(&mut self, id: PersistenceId) -> bool {
        let byte_adr = eeprom::DAT + (id as u32) / 8;
        let bit_pattern: u8 = 1 << (id as u32) % 8;
        match self.eeprom.read_byte(byte_adr) {
            Ok(found) => (found & bit_pattern) != 0,
            Err(_) => false,
        }
    }

    /// Set dat_bit in table of contentspub fn iter_over(&mut self, p_type: PersistType) -> PersistenceIterator
    fn set_id(&mut self, id: PersistenceId) -> Result<(), Error> {
        let byte_adr = eeprom::DAT + (id as u32) / 8;
        let bit_pattern: u8 = 1 << (id as u32) % 8;
        let found = self.eeprom.read_byte(byte_adr).map_err(|_| Error::EepromOrI2c1)?;
        let target = found | bit_pattern;
        if found != target {
            self.eeprom.write_byte(byte_adr, target).map_err(|_| Error::EepromOrI2c1)?;
            delay_ms(10);
        }
        Ok(())
    }
}

/// Helper struct for Iteration
pub struct PersistenceIterator<'a> {
    cur_id: u16,
    end_id: u16, 
    cur_byte: u8,
    persistence: &'a mut Eeprom,
}

impl <'a>PersistenceIterator<'a> {
    /// Creates a iteration helper struct
    pub fn new(start_id: u16, end_id: u16, persistence: &'a mut Eeprom) -> Self {
        PersistenceIterator { 
            cur_id: start_id, 
            end_id,
            cur_byte: 0, 
            persistence }
    }
}

impl Iterator for PersistenceIterator<'_> {
    type Item = PersistenceItem;
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
