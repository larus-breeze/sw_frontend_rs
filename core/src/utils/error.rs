#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum CoreError {
    NoError,
    DrawError,
    U8g2BackgroundColorNotSupported,
    U8g2GlyphNotFound,
    EepromOrI2c1,
    NoItemAvailable,
    OutOfRange,
    SdCard,
    ConversionError,
    ParseError,
    SchedulerQueueOverflow,
    PersistenceIdNotInDat,
}

impl From<u8g2_fonts::Error<CoreError>> for CoreError {
    fn from(error: u8g2_fonts::Error<CoreError>) -> Self {
        match error {
            u8g2_fonts::Error::BackgroundColorNotSupported => {
                CoreError::U8g2BackgroundColorNotSupported
            }
            u8g2_fonts::Error::GlyphNotFound(_) => CoreError::U8g2GlyphNotFound,
            u8g2_fonts::Error::DisplayError(core_error) => core_error,
        }
    }
}
