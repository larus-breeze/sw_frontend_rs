use eeprom::ADR_USER_PROFILE;

use crate::{CoreError, PersistenceId, Variant};

#[cfg(feature = "eeprom_size_8192")]
pub mod eeprom {
    // size of eeprom
    pub const SIZE: u32 = 8192;
    // address, where magic number is stored
    pub const ADR_IDENTIFICATION_BLOCK: u32 = 0;
    // address, where active user profile is stored
    pub const ADR_USER_PROFILE: u32 = 8;
    // address, where reset reason is stored
    pub const ADR_RESET_REASON: u32 = 9;
    //...

    // start adress of data allocation table
    pub const ADR_DAT: u32 = 32;
    // len of DAT in bytes
    pub const DAT_LEN: u32 = SIZE / 8 / 4;
    // start address of stored data
    pub const ADR_DATA_STORAGE: u32 = ADR_DAT + DAT_LEN;
    // maximum of possible items to store in eeprom
    pub const MAX_ITEM_COUNT: u32 = (SIZE - ADR_DATA_STORAGE) / 4;
    // magic number to identify, if eeprom is initialized
    pub const MAGIC: [u8; 8] = [0x1e, 0xf9, 0xb4, 0xaf, 0x22, 0xe1, 0xe5, 0xeb];
}

pub enum EepromTopic {
    ConfigValues,
}

pub const MAX_USER_VALUES: u32 = 256;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PersistenceItem {
    pub id: PersistenceId,
    pub data: [u8; 4],
}

impl PersistenceItem {
    pub fn do_not_store() -> Self {
        PersistenceItem {
            id: PersistenceId::DoNotStore,
            data: [0, 0, 0, 0],
        }
    }

    pub fn from_bool(id: PersistenceId, value: bool) -> Self {
        let data: u32 = if value { 1 } else { 0 };
        PersistenceItem {
            id,
            data: data.to_le_bytes(),
        }
    }

    pub fn from_i8(id: PersistenceId, value: i8) -> Self {
        PersistenceItem {
            id,
            data: (value as i32).to_le_bytes(),
        }
    }

    pub fn from_i32(id: PersistenceId, value: i32) -> Self {
        PersistenceItem {
            id,
            data: value.to_le_bytes(),
        }
    }

    pub fn from_u8(id: PersistenceId, value: u8) -> Self {
        PersistenceItem {
            id,
            data: (value as u32).to_le_bytes(),
        }
    }

    pub fn from_u16(id: PersistenceId, value: u16) -> Self {
        PersistenceItem {
            id,
            data: (value as u32).to_le_bytes(),
        }
    }

    pub fn from_u32(id: PersistenceId, value: u32) -> Self {
        PersistenceItem {
            id,
            data: value.to_le_bytes(),
        }
    }

    pub fn from_f32(id: PersistenceId, value: f32) -> Self {
        PersistenceItem {
            id,
            data: value.to_le_bytes(),
        }
    }

    pub fn from_variant(id: PersistenceId, variant: Variant) -> Self {
        match variant {
            Variant::Bool(bool) => Self::from_bool(id, bool),
            Variant::I8(i8) => Self::from_i8(id, i8),
            Variant::I32(i32) => Self::from_i32(id, i32),
            Variant::F32(f32) => Self::from_f32(id, f32),
            Variant::U8(u8) => Self::from_u8(id, u8),
            Variant::U32(u32) => Self::from_u32(id, u32),

            Variant::Mass(mass) => Self::from_f32(id, mass.to_kg()),
            Variant::Pressure(pressure) => Self::from_f32(id, pressure.to_hpa()),
            Variant::Speed(speed) => Self::from_f32(id, speed.to_m_s()),

            Variant::DisplayActive(display_active) => Self::from_u32(id, display_active as u32),
            Variant::DisplayTheme(display_theme) => Self::from_u32(id, display_theme as u32),
            Variant::VarioModeControl(vario_mode_control) => {
                Self::from_u32(id, vario_mode_control as u32)
            }
            Variant::Rotation(rotation) => Self::from_u32(id, rotation as u32),
            Variant::DataSource(data_source) => Self::from_u32(id, data_source as u32),
        }
    }

    pub fn to_bool(&self) -> bool {
        i32::from_le_bytes(self.data) == 1
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

    /// Clears the dat (data allocation table) and other data
    fn clear_all_data(&mut self) -> Result<(), CoreError> {
        let mut address = eeprom::ADR_DAT;
        let data = [0_u8; 8];
        while address < (eeprom::ADR_DAT + eeprom::DAT_LEN) {
            self.write_page(address, &data)?;
            address += 8;
        }
        self.write_byte(ADR_USER_PROFILE, 0)
    }

    /// check magic number, if not ok, eeprom will be deleted
    fn check_magic(&mut self) -> Result<(), CoreError> {
        let mut magic = [0_u8; 8];
        self.read_data(eeprom::ADR_IDENTIFICATION_BLOCK, &mut magic)?;
        if magic != eeprom::MAGIC {
            // Write magic number
            self.write_page(eeprom::ADR_IDENTIFICATION_BLOCK, &eeprom::MAGIC)?;
            self.clear_all_data()?;
        }
        Ok(())
    }
}

pub struct Eeprom<S>
where
    S: EepromTrait,
{
    user_profile: u8,
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
        eeprom.check_magic()?;

        let user_profile = eeprom.read_byte(ADR_USER_PROFILE)?;
        let user_profile = match user_profile {
            0..=3 => user_profile,
            _ => 0,
        };

        Ok(Eeprom {
            eeprom,
            user_profile,
        })
    }

    /// Write a PersistentItem into the data store
    ///
    /// The data is stored at the desired location defined by the ID. An entry is made in the data
    /// allocation table (DAT), if desired (dat_bit in PersitentItem).
    pub fn write_item(&mut self, item: PersistenceItem) -> Result<(), CoreError> {
        match item.id {
            PersistenceId::DoNotStore => Ok(()),
            PersistenceId::DeleteAll => self.eeprom.clear_all_data(),
            PersistenceId::UserProfile => self
                .eeprom
                .write_byte(eeprom::ADR_USER_PROFILE, item.data[0]),
            _ => {
                let address = self.item_address(item.id);
                self.set_id(item.id)?;
                self.eeprom.write_page(address, &item.data)
            }
        }
    }

    /// Read data from storage - do not check the DAT
    pub fn read_item_unchecked(&mut self, id: PersistenceId) -> Result<PersistenceItem, CoreError> {
        let address = self.item_address(id);
        let mut data = [0_u8; 4];
        self.eeprom
            .read_data(address, &mut data)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        Ok(PersistenceItem { id, data })
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
            EepromTopic::ConfigValues => (0, PersistenceId::LastItem as u16),
        };
        PersistenceIterator::new(start_id, end_id, self)
    }

    /// Delete all items of the given list
    pub fn delete_items_list(&mut self, items_list: &[PersistenceId]) -> Result<(), CoreError> {
        for item_id in items_list {
            self.clear_id(*item_id)?;
        }
        Ok(())
    }

    /// returns the address of an item
    fn item_address(&mut self, id: PersistenceId) -> u32 {
        eeprom::ADR_DATA_STORAGE + self.user_profile as u32 * MAX_USER_VALUES + id as u32 * 4
    }

    /// returns address of the byte of id in DAT
    fn data_byte_adr_from_id(&mut self, id: PersistenceId) -> u32 {
        eeprom::ADR_DAT + (id as u32 + self.user_profile as u32 * MAX_USER_VALUES) / 8
    }

    /// Returns a byte of the DAT
    fn read_bitfield_byte(&mut self, id: PersistenceId) -> Result<u8, CoreError> {
        if id as u32 >= PersistenceId::LastItem as u32 {
            return Err(CoreError::PersistenceIdNotInDat);
        }
        let byte_adr = self.data_byte_adr_from_id(id);
        self.eeprom.read_byte(byte_adr)
    }

    /// Tests a id, if coresponing dat_bit is set
    fn test_id(&mut self, id: PersistenceId) -> bool {
        let byte_adr = self.data_byte_adr_from_id(id);
        let bit_pattern: u8 = 1 << ((id as u32) % 8);
        match self.eeprom.read_byte(byte_adr) {
            Ok(found) => (found & bit_pattern) != 0,
            Err(_) => false,
        }
    }

    /// Set dat_bit in table of contentspub fn iter_over(&mut self, p_type: PersistType) -> PersistenceIterator
    fn set_id(&mut self, id: PersistenceId) -> Result<(), CoreError> {
        let byte_adr = self.data_byte_adr_from_id(id);
        let bit_pattern: u8 = 1 << ((id as u32) % 8);
        let found = self.eeprom.read_byte(byte_adr)?;
        let target = found | bit_pattern;
        if found != target {
            self.eeprom.write_byte(byte_adr, target)?;
        }
        Ok(())
    }

    /// Deletes the bit in DAT => deletes the item in store
    fn clear_id(&mut self, id: PersistenceId) -> Result<(), CoreError> {
        let byte_adr = self.data_byte_adr_from_id(id);
        let bit_pattern: u8 = !(1 << ((id as u32) % 8));
        let found = self.eeprom.read_byte(byte_adr)?;
        let target = found & bit_pattern;
        if found != target {
            self.eeprom.write_byte(byte_adr, target)?;
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
    cur_byte: Result<u8, CoreError>,
    user_profile_sent: bool,
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
            cur_byte: Ok(0),
            user_profile_sent: false,
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
        if self.user_profile_sent {
            while self.cur_id < self.end_id {
                if self.cur_id % 8 == 0 {
                    self.cur_byte = self.persistence.read_bitfield_byte(self.cur_id.into());
                }
                if let Ok(cur_byte) = self.cur_byte {
                    let cur_bit = 0x01 << (self.cur_id % 8);
                    let id_exists = (cur_bit & cur_byte) != 0;

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
                } else {
                    self.cur_id += 1;
                }
            }
            None
        } else {
            let value = self.persistence.eeprom.read_byte(ADR_USER_PROFILE).unwrap();
            let value = num::clamp(value, 0, 3);
            let item = PersistenceItem::from_u8(PersistenceId::UserProfile, value);
            self.user_profile_sent = true;
            Some(item)
        }
    }
}
