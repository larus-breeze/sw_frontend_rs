
use std::io::{Read, Write};
use std::convert::TryInto;

use corelib::{
    eeprom, PersistenceId, PersistenceItem, 
    EepromTopic, CONFIG_VALUES_START, CONFIG_VALUES_END
};

const FILE_NAME: &str = "eeprom.bin";

#[derive(Debug)]
pub enum Error {
    NoItemAvailable,
}

pub struct Eeprom {
    data: [u8; 8192],
}

#[allow(dead_code)]
impl Eeprom {
    /// Create a Persistence Instance
    pub fn new() -> Result<Self, Error> {
        let mut data = [0_u8; 8192];
        match std::fs::File::open(FILE_NAME) {
            Ok(mut f) => f.read_exact(&mut data).unwrap(),
            Err(_) => (),
        };

        let magic: [u8; 8] = data[0..8].try_into().unwrap();
        if magic != eeprom::MAGIC {
            // Write magic number
            data[..8].copy_from_slice(&eeprom::MAGIC);

            for idx in eeprom::DAT .. eeprom::DATA_STORAGE {
                data[idx as usize] = 0;
            }
            println!("Initialize DAT");
        }
        Ok(Eeprom { data })
    }

    /// Write a PersistentItem into the data store
    /// 
    /// The data is stored at the desired location defined by the ID. An entry is made in the data 
    /// allocation table (DAT), if desired (dat_bit in PersitentItem). 
    pub fn write_item(&mut self, item: PersistenceItem) -> Result <(), Error> {
        if item.id == PersistenceId::DoNotStore {
            return Ok(())
        }
        let address = (eeprom::DATA_STORAGE + item.id as u32 * 4) as usize;
        if item.dat_bit {
            self.set_id(item.id)?;
        }
        self.data[address..address+4].copy_from_slice(&item.data);
        let mut f = std::fs::File::create(FILE_NAME).unwrap();
        f.write(&self.data).unwrap();
        Ok(())
    }

    /// Read data from storage - do not check the DAT
    pub fn read_item_unchecked(&mut self, id: PersistenceId) -> Result<PersistenceItem, Error> {
        let address = (eeprom::DATA_STORAGE + id as u32 * 4) as usize;
        let mut data = [0_u8; 4];
        data.copy_from_slice(&self.data[address..address+4]);
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

    /// Tests a id, if coresponing dat_bit is set
    fn test_id(&mut self, id: PersistenceId) -> bool {
        let byte_adr = (eeprom::DAT + (id as u32) / 8) as usize;
        let bit_pattern: u8 = 1 << (id as u32) % 8;
        self.data[byte_adr] & bit_pattern != 0
    }

    /// Set dat_bit in table of contentspub fn iter_over(&mut self, p_type: PersistType) -> PersistenceIterator
    fn set_id(&mut self, id: PersistenceId) -> Result<(), Error> {
        let byte_adr = (eeprom::DAT + (id as u32) / 8) as usize;
        let bit_pattern: u8 = 1 << (id as u32) % 8;
        println!("set_id id {}, bit_pattern {:#010b}", id as u32, bit_pattern);
        self.data[byte_adr] |= bit_pattern;
        Ok(())
    }
}

/// Helper struct for Iteration
pub struct PersistenceIterator<'a> {
    cur_id: u16,
    end_id: u16, 
    persistence: &'a mut Eeprom,
}

impl <'a>PersistenceIterator<'a> {
    /// Creates a iteration helper struct
    pub fn new(start_id: u16, end_id: u16, persistence: &'a mut Eeprom) -> Self {
        PersistenceIterator { 
            cur_id: start_id, 
            end_id,
            persistence }
    }
}

impl Iterator for PersistenceIterator<'_> {
    type Item = PersistenceItem;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_id < self.end_id {
            if self.persistence.test_id(self.cur_id.into()) {
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
