//use defmt::trace;

use crate::driver::delay_ms;
use corelib::{CoreError, Eeprom, EepromTrait};
use eeprom24x::{addr_size::TwoBytes, page_size::B32, Eeprom24x, SlaveAddr};
use stm32h7xx_hal::{i2c::I2c, pac::I2C1};

pub type Eeprom24xType = Eeprom24x<I2c<I2C1>, B32, TwoBytes>;
pub struct Storage {
    eeprom: Eeprom24xType,
}

#[allow(dead_code)]
impl Storage {
    /// Create a Persistence Instance
    pub fn new(i2c: I2c<I2C1>) -> Result<Eeprom<Storage>, CoreError> {
        let addr = SlaveAddr::Alternative(true, true, true);
        let eeprom24 = Eeprom24x::new_24x64(i2c, addr);
        let storage = Storage { eeprom: eeprom24 };
        Eeprom::new(storage)
    }
}

impl EepromTrait for Storage {
    fn write_byte(&mut self, address: u32, data: u8) -> Result<(), CoreError> {
        self.eeprom
            .write_byte(address, data)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        delay_ms(10);
        Ok(())
    }

    fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), CoreError> {
        self.eeprom
            .write_page(address, data)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        delay_ms(10);
        Ok(())
    }

    fn read_byte(&mut self, address: u32) -> Result<u8, CoreError> {
        self.eeprom
            .read_byte(address)
            .map_err(|_| CoreError::EepromOrI2c1)
    }

    fn read_data(&mut self, address: u32, data: &mut [u8]) -> Result<(), CoreError> {
        self.eeprom
            .read_data(address, data)
            .map_err(|_| CoreError::EepromOrI2c1)?;
        Ok(())
    }
}
