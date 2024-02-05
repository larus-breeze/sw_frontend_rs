
///
/// Errors from ad57 parts
///
#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    Unknown,
    SdCard,
    CrcError,
    DisplayError,
    PinError,
    EepromOrI2c1,
    NoItemAvailable,
}

pub type DevError = Error;