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

pub struct FileIo {
    size: u64,
    sdio: RefCell<Sdio<SdCard>>,
}

impl FileIo {
    /// Init Card if available
    pub fn new(dp_sdio: SDIO, clocks: &Clocks, pins: SdioPins) -> Option<Self> {
        let mut sdio: Sdio<SdCard> = Sdio::new(dp_sdio, pins, clocks);
        sdio.init(ClockFreq::F24Mhz).ok()?;
        let card = sdio.card().ok()?;
        let size = card.csd.card_size();
        // unsafe is ok, becaus we are the only one knowing and using SDIO
        //unsafe { SDIO.replace(Some(sdio)) };
        Some(FileIo { size, sdio: RefCell::new(sdio) })
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
        let mut sdio = self.sdio.borrow_mut();
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
        let mut sdio = self.sdio.borrow_mut();
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

static mut FILE_SYS: Option<FileSys> = None;

/// Access to the file system of the SdCard
///
/// The file system may only be accessed when the application is started or when all interrupts 
/// are disabled. The background to this is that the SDIO driver of the HAL is not resistant to 
/// interrupts during access. For this reason, access from different thread contexts is not 
/// protected.
pub struct FileSys {
    vol_mgr: FatFs,
}

impl FileSys {
    pub fn new(dp_sdio: SDIO, clocks: &Clocks, pins: SdioPins) -> Result<(), Error> {
        let file_io = FileIo::new(dp_sdio, clocks, pins).ok_or(Error::SdCard)?;
        let vol_mgr = VolumeManager::new(file_io, TimeSource);
        // ok, since access only provided from one thread 
        unsafe {
            FILE_SYS = Some(FileSys { vol_mgr })
        }
        Ok(())
    }

    pub fn vol_mgr(&mut self) -> &mut FatFs {
        &mut self.vol_mgr
    }
}

pub fn get_filesys() -> Option<&'static mut FileSys> {
    // ok, since access only provided from one thread 
    unsafe { FILE_SYS.as_mut() }

}