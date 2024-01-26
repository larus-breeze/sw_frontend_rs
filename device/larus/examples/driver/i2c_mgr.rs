/// `RefCell`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with a `RefCell`. This means it has low overhead, but RefCell based instances are not `Send`,
/// so it only allows sharing within a single thread (interrupt priority level). If you need to share a bus across several
/// threads, use [`CriticalSectionDevice`](super::CriticalSectionDevice) instead.
///
/// Assuming there is a pressure sensor with address `0x42` on the same bus as a temperature sensor with address `0x20`; 
/// I2cManager can be used to give access to both of these sensors from a single `i2c` instance.
///
/// ```
/// use embedded_hal_bus::i2c;
/// use core::cell::RefCell;
///
/// let i2c = hal.i2c();
/// let i2c_ref_cell = RefCell::new(i2c);
/// 
/// let mut temperature_sensor = TemperatureSensor::new(
///   i2c::RefCellDevice::new(&i2c_ref_cell),
///   0x20,
/// );
/// 
/// let mut pressure_sensor = PressureSensor::new(
///   i2c::RefCellDevice::new(&i2c_ref_cell),
///   0x42,
/// );
/// ```
/// 
/// In the front end, i2c communication is intended to take place exclusively in the idle loop. In this respect, this 
/// simple solution is sufficient.
use core::{borrow::BorrowMut, cell::RefCell};
use embedded_hal::blocking::i2c::{self, SevenBitAddress, TenBitAddress, Write, Read, WriteRead};
use stm32h7xx_hal::{
    device::I2C1,
    i2c::{I2c, Error as I2cError},
};

pub struct I2cManager<'a> {
    i2c_ref: &'a RefCell<I2c<I2C1>>,
}

impl<'a> I2cManager<'a> {
    /// Create a new `RefCellDevice`.
    #[inline]
    pub fn new(i2c_ref: &'a RefCell<I2c<I2C1>>) -> Self {
        Self { i2c_ref }
    }
}

impl<'a> Write for I2cManager<'a> {
    type Error = I2cError;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.i2c_ref.borrow_mut();
        bus.write(address, bytes)
    }
}

impl<'a> Read for I2cManager<'a> {
    type Error = I2cError;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.i2c_ref.borrow_mut();
        bus.read(address, buffer)
    }
}

impl<'a> WriteRead for I2cManager<'a> {
    type Error = I2cError;
    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.i2c_ref.borrow_mut();
        bus.write_read(address, bytes, buffer)
    }
}
