use crate::{CoreError, PersistenceId};

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

pub struct Eeprom<S>
where
    S: EepromTrait,
{
    eeprom: S,
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
impl<S> Eeprom<S>
where
    S: EepromTrait,
{
    /// Create a Persistence Instance
    pub fn new(mut eeprom: S) -> Result<Self, CoreError> {
        let mut magic = [0_u8; 8];
        eeprom
            .read_data(eeprom::IDENTIFICATION_BLOCK, &mut magic)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        if magic != eeprom::MAGIC {
            // Write magic number
            eeprom
                .write_page(eeprom::IDENTIFICATION_BLOCK, &eeprom::MAGIC)
                .map_err(|_| CoreError::EepromOrI2c1)?;

            // Clear DAT
            let mut address = eeprom::DAT;
            let data = [0_u8; 8];
            while address < (eeprom::DAT + eeprom::DAT_LEN) {
                eeprom
                    .write_page(address, &data)
                    .map_err(|_| CoreError::EepromOrI2c1)?;
                address += 8;
            }
        }
        Ok(Eeprom { eeprom })
    }

    /// Write a PersistentItem into the data store
    ///
    /// The data is stored at the desired location defined by the ID. An entry is made in the data
    /// allocation table (DAT), if desired (dat_bit in PersitentItem).
    pub fn write_item(&mut self, item: PersistenceItem) -> Result<(), CoreError> {
        if item.id == PersistenceId::DoNotStore {
            return Ok(());
        }
        let address = eeprom::DATA_STORAGE + item.id as u32 * 4;
        if item.dat_bit {
            self.set_id(item.id)?;
        }
        self.eeprom
            .write_page(address, &item.data)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        Ok(())
    }

    /// Read data from storage - do not check the DAT
    pub fn read_item_unchecked(&mut self, id: PersistenceId) -> Result<PersistenceItem, CoreError> {
        let address = eeprom::DATA_STORAGE + id as u32 * 4;
        let mut data = [0_u8; 4];
        self.eeprom
            .read_data(address, &mut data)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        Ok(PersistenceItem {
            id,
            dat_bit: false,
            data,
        })
    }

    /// Read data from storage - return error if DAT bit is not set
    pub fn read_item(&mut self, id: PersistenceId) -> Result<PersistenceItem, CoreError> {
        if self.test_id(id) {
            self.read_item_unchecked(id)
        } else {
            Err(CoreError::NoItemAvailable)
        }
    }

    /// Returns an iterator to the desired topic area
    pub fn iter_over(&mut self, p_type: EepromTopic) -> PersistenceIterator<S> {
        let (start_id, end_id) = match p_type {
            EepromTopic::ConfigValues => (CONFIG_VALUES_START, CONFIG_VALUES_END),
        };
        PersistenceIterator::new(start_id, end_id, self)
    }

    /// Returns a byte of the DAT
    fn read_bitfield_byte(&mut self, adr: u32) -> Result<u8, CoreError> {
        let byte_adr = eeprom::DAT + adr;
        self.eeprom
            .read_byte(byte_adr)
            .map_err(|_| CoreError::EepromOrI2c1)
    }

    /// Tests a id, if coresponing dat_bit is set
    fn test_id(&mut self, id: PersistenceId) -> bool {
        let byte_adr = eeprom::DAT + (id as u32) / 8;
        let bit_pattern: u8 = 1 << ((id as u32) % 8);
        match self.eeprom.read_byte(byte_adr) {
            Ok(found) => (found & bit_pattern) != 0,
            Err(_) => false,
        }
    }

    /// Set dat_bit in table of contentspub fn iter_over(&mut self, p_type: PersistType) -> PersistenceIterator
    fn set_id(&mut self, id: PersistenceId) -> Result<(), CoreError> {
        let byte_adr = eeprom::DAT + (id as u32) / 8;
        let bit_pattern: u8 = 1 << ((id as u32) % 8);
        let found = self
            .eeprom
            .read_byte(byte_adr)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        let target = found | bit_pattern;
        if found != target {
            self.eeprom
                .write_byte(byte_adr, target)
                .map_err(|_| CoreError::EepromOrI2c1)?;
        }
        Ok(())
    }
}

/// Helper struct for Iteration
pub struct PersistenceIterator<'a, S>
where
    S: EepromTrait,
{
    cur_id: u16,
    end_id: u16,
    cur_byte: u8,
    persistence: &'a mut Eeprom<S>,
}

impl<'a, S> PersistenceIterator<'a, S>
where
    S: EepromTrait,
{
    /// Creates a iteration helper struct
    pub fn new(start_id: u16, end_id: u16, persistence: &'a mut Eeprom<S>) -> Self {
        PersistenceIterator {
            cur_id: start_id,
            end_id,
            cur_byte: 0,
            persistence,
        }
    }
}

impl<S> Iterator for PersistenceIterator<'_, S>
where
    S: EepromTrait,
{
    type Item = PersistenceItem;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_id < self.end_id {
            if self.cur_id % 8 == 0 {
                self.cur_byte = self
                    .persistence
                    .read_bitfield_byte((self.cur_id / 8) as u32)
                    .unwrap();
            }
            let cur_bit = 0x01 << (self.cur_id % 8);
            let id_exists = (cur_bit & self.cur_byte) != 0;

            if id_exists {
                let r = self
                    .persistence
                    .read_item_unchecked(self.cur_id.into())
                    .unwrap();
                self.cur_id += 1;
                return Some(r);
            } else {
                self.cur_id += 1;
            }
        }
        None
    }
}
