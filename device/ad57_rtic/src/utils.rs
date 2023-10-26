mod statistics;

pub use statistics::{Statistics, Task};

///
/// An error holding its source (pins or SPI)
///
#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    DisplayError,
    PinError,
    EepromOrI2c1,
    NoItemAvailable,
}
