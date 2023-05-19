///
/// An error holding its source (pins or SPI)
///
#[derive(Debug)]
pub enum Error<PinE> {
    DisplayError,
    Pin(PinE),
}
