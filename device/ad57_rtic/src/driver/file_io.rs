use stm32f4xx_hal::sdio::{SdCard, Sdio, ClockFreq};
use fatfs::{self, SeekFrom, IoError, IoBase, Read, Write, Seek};


///
/// Errros from FileIo
/// 
#[allow(dead_code)]
#[derive(Debug)]
pub enum FioError {
    UnexpectedEof,
    WriteZeroError,
    ReadError,
    WriteError,
    SeekNegative,
    NoSignature,
    InitError,
}

impl IoError for FioError {
    fn is_interrupted(&self) -> bool {
        false
    }

    fn new_unexpected_eof_error() -> Self {
        FioError::UnexpectedEof
    }

    fn new_write_zero_error() -> Self {
        FioError::WriteZeroError
    }
}

/// Adapter for fatfs, to access the SD Card
/// 
/// FileIo fulfills the traits defined by fatfs in order to access the SD card. The implementation 
/// is simple, but with little focus on performance. Only a 512 byte buffer is used to cache the 
/// read and write data. 
pub struct FileIo {
    size: u64,
    sdio: Sdio<SdCard>,
    current: u64,
    blockaddr: u32,
    block: [u8; 512],   
    is_on_sdcard: bool,
}

impl FileIo {
    /// Create a FileIo instance
    pub fn new(sdio: Sdio<SdCard>) -> Result<Self, FioError> {
        let block = [0_u8; 512];
        let mut fileio = FileIo { 
                size: 0, 
                sdio, 
                current: 0, 
                blockaddr: 0,
                block, 
                is_on_sdcard: true, 
        };
        fileio.init()?;
        Ok(fileio)
    }

    /// Initialize the instance
    /// 
    /// Note: It is currently not possible to access different SD cards with the stm32f4xx_hal
    ///  crate during the course. See also: https://github.com/stm32-rs/stm32f4xx-hal/issues/692.
    ///
    /// This separate init() routine is already prepared for the fact that it will be possible 
    /// to access several SD cards at some point.
    fn init(&mut self) -> Result<(), FioError> {
        self.sdio.init(ClockFreq::F24Mhz).map_err(|_| FioError::InitError)?;
        let card = self.sdio.card().map_err(|_| FioError::InitError)?;
        self.size = card.csd.card_size();
        self.blockaddr = 0;
        self.current = 0;
        self.is_on_sdcard = true;
        self.sdio.read_block(self.blockaddr, &mut self.block).map_err(|_| FioError::ReadError)?;
        if self.block[0x01FE..0x0200] != [0x55, 0xAA] {
            Err(FioError::NoSignature)
        } else {
            Ok(())
        }
    }


    /// This function reads in a block and manages a cache buffer of 512 bytes.
    fn read_block(&mut self, blockaddr: u32) -> Result<(), FioError> {
        if blockaddr != self.blockaddr {
            if !self.is_on_sdcard {
                self.sdio.write_block(self.blockaddr, &self.block).map_err(|_| FioError::WriteError)?;
            }
            self.sdio.read_block(blockaddr, &mut self.block).map_err(|_| FioError::ReadError)?;
            self.blockaddr = blockaddr;
            self.is_on_sdcard = true;
        }
        Ok(())
    }
}

/// see https://github.com/rafalh/rust-fatfs/blob/master/src/io.rs
impl IoBase for FileIo {
    type Error = FioError;
}

/// see https://github.com/rafalh/rust-fatfs/blob/master/src/io.rs
impl Seek for FileIo {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, FioError> {
        let idx = match pos {
            SeekFrom::Current(dif) => self.current as i64 + dif,
            SeekFrom::End(dif) => self.size as i64 + dif,
            SeekFrom::Start(idx) => idx as i64,
        };
        //rprintln!("SeekFrom {:?}, idx {}", pos, idx);
        if idx < 0 {
            return Err(FioError::SeekNegative)
        }
        self.current = idx as u64;
        Ok(self.current)
    }
}

/// see https://github.com/rafalh/rust-fatfs/blob/master/src/io.rs
impl Read for FileIo {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, FioError> {
        if (self.current >= self.size) | (buf.len() == 0) {
            Ok(0)
        } else {
            let blockaddr = (self.current / 512) as u32;
            let pos_in_block = (self.current % 512) as usize;
            //rprintln!("read block {}, pos {}, len {}", blockaddr, pos_in_block, buf.len());

            self.read_block(blockaddr)?;
            let mut dest_idx: usize = 0;
            for src_idx in pos_in_block..512 {
                if dest_idx < buf.len() {
                    buf[dest_idx] = self.block[src_idx];
                    dest_idx += 1;
                } else {
                    break;
                }
            }

            //rprintln!("fatfs::read() {} {:?}", dest_idx, buf);
            self.current += dest_idx as u64;
            Ok(dest_idx)
        }
    }
}

/// see https://github.com/rafalh/rust-fatfs/blob/master/src/io.rs
impl Write for FileIo {
    fn flush(&mut self) -> Result<(), FioError> {
        if !self.is_on_sdcard {
            self.sdio.write_block(self.blockaddr, &self.block).map_err(|_| FioError::WriteError)?;
            self.is_on_sdcard = true;
        }
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, FioError> {
        if (self.current >= self.size) | (buf.len() == 0) {
            Ok(0)
        } else {
            let blockaddr = (self.current / 512) as u32;
            let pos_in_block = (self.current % 512) as usize;
            self.read_block(blockaddr)?;

            let mut src_idx: usize = 0;
            for dest_idx in pos_in_block..512 {
                if src_idx < buf.len() {
                    self.block[dest_idx] = buf[src_idx];
                    src_idx += 1;
                } else {
                    break;
                }
            }
            self.is_on_sdcard = false;
            self.current += src_idx as u64;
            Ok(src_idx)
        }
    }
}

/// see https://github.com/rafalh/rust-fatfs/blob/master/src/io.rs
impl core::ops::Drop for FileIo {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
