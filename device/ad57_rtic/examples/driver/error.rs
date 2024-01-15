#[derive(Debug)]
#[allow(unused)]
pub enum DevError {
    Unknown,
    CrcError,
    DisplayError,
    PinError,
    EepromOrI2c1,
    NoItemAvailable,
}
