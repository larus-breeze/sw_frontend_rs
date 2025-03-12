use corelib::Lock;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use stm32h7xx_hal::{
    device::I2C1,
    i2c::{Error as I2cError, I2c},
};

static I2C_REF: Lock<I2c<I2C1>> = Lock::new();

pub struct I2cManager {}

impl I2cManager {
    pub fn new(i2c: I2c<I2C1>) -> Self {
        I2C_REF.set(i2c);
        Self {}
    }

    pub fn clone() -> Self {
        Self {}
    }
}

impl Write for I2cManager {
    type Error = I2cError;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        I2C_REF.lock_during_use(|i2c_ref| {
            let bus = i2c_ref.unwrap();
            bus.write(address, bytes)
        })
    }
}

impl Read for I2cManager {
    type Error = I2cError;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        I2C_REF.lock_during_use(|i2c_ref| {
            let bus = i2c_ref.unwrap();
            bus.read(address, buffer)
        })
    }
}

impl WriteRead for I2cManager {
    type Error = I2cError;
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        I2C_REF.lock_during_use(|i2c_ref| {
            let bus = i2c_ref.unwrap();
            bus.write_read(address, bytes, buffer)
        })
    }
}
