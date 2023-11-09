use stm32f4xx_hal::sdio::{SdCard, Sdio};
use fatfs::{self, SeekFrom, IoError, IoBase, Read, Write, Seek};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    SdUnexpectedEof,
    SdWriteZeroError,
    SdReadError,
    SdWriteError,
    SdSeekNegative,
    SdNoSignature,
    SdInitError,
}

impl IoError for Error {
    fn is_interrupted(&self) -> bool {
        false
    }

    fn new_unexpected_eof_error() -> Self {
        Error::SdUnexpectedEof
    }

    fn new_write_zero_error() -> Self {
        Error::SdWriteZeroError
    }
}

pub struct FileIo {
    size: u64,
    sdio: Sdio<SdCard>,
    current: u64,
    blockaddr: u32,
    block: [u8; 512],
    is_on_sdcard: bool,
}

impl FileIo {
    pub fn new(mut sdio: Sdio<SdCard>) -> Result<Self, Error> {
        let card = sdio.card().map_err(|_| Error::SdInitError)?;
        let size = card.csd.card_size();
        let mut block = [0_u8; 512];
        let blockaddr = 0;
        sdio.read_block(blockaddr, &mut block).map_err(|_| Error::SdReadError)?;
        if block[0x01FE..0x0200] != [0x55, 0xAA] {
            return Err(Error::SdNoSignature);
        }
        Ok(FileIo { 
            size, 
            sdio, 
            current: 0, 
            blockaddr,
            block, 
            is_on_sdcard: true 
        })
    }

    fn read_block(&mut self, blockaddr: u32) -> Result<(), Error> {
        if blockaddr != self.blockaddr {
            if !self.is_on_sdcard {
                self.sdio.write_block(blockaddr, &self.block).map_err(|_| Error::SdWriteError)?;
            }
            self.sdio.read_block(blockaddr, &mut self.block).map_err(|_| Error::SdReadError)?;
            self.blockaddr = blockaddr;
            self.is_on_sdcard = true;
        }
        Ok(())
    }
}

impl IoBase for FileIo {
    type Error = Error;
}

impl Seek for FileIo {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let idx = match pos {
            SeekFrom::Current(dif) => self.current as i64 + dif,
            SeekFrom::End(dif) => self.size as i64 + dif,
            SeekFrom::Start(idx) => idx as i64,
        };
        //rprintln!("SeekFrom {:?}, idx {}", pos, idx);
        if idx < 0 {
            return Err(Error::SdSeekNegative)
        }
        self.current = idx as u64;
        Ok(self.current)
    }
}

impl Read for FileIo {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
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

impl Write for FileIo {
    fn flush(&mut self) -> Result<(), Error> {
        if !self.is_on_sdcard {
            self.sdio.write_block(self.blockaddr, &self.block).map_err(|_| Error::SdWriteError)?;
            self.is_on_sdcard = true;
        }
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if (self.current >= self.size) | (buf.len() == 0) {
            Ok(0)
        } else {
            let blockaddr = (self.current / 6512) as u32;
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

impl core::ops::Drop for FileIo {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
