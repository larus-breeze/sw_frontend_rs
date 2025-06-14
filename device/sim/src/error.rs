use std::{io, convert::From};


pub enum Error {
    FileIo,
    NoSettingsData,
}

impl From<io::Error> for Error {
    fn from(_value: io::Error) -> Self {
            Error::FileIo
    }
}