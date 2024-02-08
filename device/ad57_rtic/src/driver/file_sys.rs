use core::cell::RefCell;
use embedded_sdmmc::{Block, BlockCount, BlockDevice as SdmmcBlockDevice, BlockIdx, VolumeManager};
use stm32f4xx_hal::{
    gpio::Pin,
    pac::SDIO,
    rcc::Clocks,
    sdio::ClockFreq,
    sdio::{SdCard, Sdio},
};

#[derive(Debug)]
pub enum Error {
    SdCard,
}

pub struct TimeSource;

impl embedded_sdmmc::TimeSource for TimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

/// Pin definition for SDIO Peripheral
pub type SdioPins = (
    Pin<'C', 12>,
    Pin<'D', 2>,
    Pin<'C', 8>,
    Pin<'C', 9>,
    Pin<'C', 10>,
    Pin<'C', 11>,
);

static mut SDIO: RefCell<Option<Sdio<SdCard>>> = RefCell::new(None);

pub struct FileIo {
    size: u64,
}

impl FileIo {
    /// Init Card if available
    pub fn new(dp_sdio: SDIO, clocks: &Clocks, pins: SdioPins) -> Option<Self> {
        let mut sdio: Sdio<SdCard> = Sdio::new(dp_sdio, pins, clocks);
        sdio.init(ClockFreq::F24Mhz).ok()?;
        let card = sdio.card().ok()?;
        let size = card.csd.card_size();
        // unsafe is ok, becaus we are the only one knowing and using SDIO
        unsafe { SDIO.replace(Some(sdio)) };
        Some(FileIo { size })
    }
}

impl SdmmcBlockDevice for FileIo {
    type Error = Error;

    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        let start = start_block_idx.0;
        // unsafe is ok, becaus we are the only one knowing and using SDIO
        let mut rc = unsafe { SDIO.borrow_mut() };
        let sdio = rc.as_mut().ok_or(Error::SdCard)?;
        for block_idx in start..(start + blocks.len() as u32) {
            sdio.read_block(
                block_idx,
                &mut blocks[(block_idx - start) as usize].contents,
            )
            .map_err(|_| Error::SdCard)?;
        }
        Ok(())
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        // unsafe is ok, becaus we are the only one knowing and using SDIO
        let mut rc = unsafe { SDIO.borrow_mut() };
        let sdio = rc.as_mut().ok_or(Error::SdCard)?;
        let start = start_block_idx.0;
        for block_idx in start..(start + blocks.len() as u32) {
            sdio.write_block(block_idx, &blocks[(block_idx - start) as usize].contents)
                .map_err(|_| Error::SdCard)?;
        }
        Ok(())
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(BlockCount((self.size / 512u64) as u32))
    }
}

type FatFs = VolumeManager<FileIo, TimeSource>;

pub struct FileSys {
    fat_fs: FatFs,
}

impl FileSys {
    pub fn new(dp_sdio: SDIO, clocks: &Clocks, pins: SdioPins) -> Result<Self, Error> {
        let file_io = FileIo::new(dp_sdio, clocks, pins).ok_or(Error::SdCard)?;
        let fat_fs = VolumeManager::new(file_io, TimeSource);
        Ok(FileSys { fat_fs })
    }

    pub fn fat(&mut self) -> &mut FatFs {
        &mut self.fat_fs
    }
}
