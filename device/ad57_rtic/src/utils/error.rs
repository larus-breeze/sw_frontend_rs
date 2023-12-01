use crate::driver::FioError;

///
/// Errors from ad57 parts
///
#[derive(Debug)]
#[allow(unused)]
pub enum Error<FSE, FE> {
    Unknown,
    FileIo(FE),
    FileSystem(FSE),
    CrcError,
    DisplayError,
    PinError,
    EepromOrI2c1,
    NoItemAvailable,
}

pub type FSE = fatfs::Error<FioError>;
pub type FE = FioError;
pub type DevError = Error<FSE, FE>;

impl From<fatfs::Error<FioError>> for DevError {
    fn from(error: fatfs::Error<FioError>) -> DevError {
        match error {
            fatfs::Error::Io(fio_error) => Error::FileIo(fio_error),
            fatfs::Error::UnexpectedEof => Error::FileSystem(fatfs::Error::UnexpectedEof),
            fatfs::Error::WriteZero => Error::FileSystem(fatfs::Error::WriteZero),
            fatfs::Error::InvalidInput => Error::FileSystem(fatfs::Error::InvalidInput),
            fatfs::Error::NotFound => Error::FileSystem(fatfs::Error::NotFound),
            fatfs::Error::AlreadyExists => Error::FileSystem(fatfs::Error::AlreadyExists),
            fatfs::Error::DirectoryIsNotEmpty => {
                Error::FileSystem(fatfs::Error::DirectoryIsNotEmpty)
            }
            fatfs::Error::CorruptedFileSystem => {
                Error::FileSystem(fatfs::Error::CorruptedFileSystem)
            }
            fatfs::Error::NotEnoughSpace => Error::FileSystem(fatfs::Error::NotEnoughSpace),
            fatfs::Error::InvalidFileNameLength => {
                Error::FileSystem(fatfs::Error::InvalidFileNameLength)
            }
            fatfs::Error::UnsupportedFileNameCharacter => {
                Error::FileSystem(fatfs::Error::UnsupportedFileNameCharacter)
            }
            _ => Error::Unknown,
        }
    }
}

impl From<FioError> for DevError {
    fn from(error: FioError) -> DevError {
        Error::FileIo(error)
    }
}
