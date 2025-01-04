use core::cell::RefCell;
use embedded_sdmmc::{
    Block, BlockCount, BlockDevice, BlockIdx, Error as SdmmcError, VolumeManager,
};
use stm32h7xx_hal::{
    device::SDMMC1,
    gpio::{Pin, Speed},
    prelude::*,
    rcc::rec::Sdmmc1,
    rcc::CoreClocks,
    sdmmc::{Error as DeviceError, SdCard, Sdmmc, SdmmcExt},
};
type FileSysError = SdmmcError<DeviceError>;

pub struct SdcardPins(
    pub Pin<'C', 12>, // clk
    pub Pin<'D', 2>,  // cmd
    pub Pin<'C', 8>,  // d0
    pub Pin<'C', 9>,  // d1
    pub Pin<'C', 10>, // d2
    pub Pin<'C', 11>, // d3
    pub Pin<'A', 15>, // detect
);

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

pub struct FileIo {
    size: u64,
    sdmmc: RefCell<Sdmmc<SDMMC1, SdCard>>,
}

impl FileIo {
    pub fn new(
        sdcard_pins: SdcardPins,
        sdmmc1: SDMMC1,
        prec: Sdmmc1,
        clocks: &CoreClocks,
    ) -> Result<Self, FileSysError> {
        let (clk, cmd, d0, d1, d2, d3, detect) = (
            sdcard_pins
                .0
                .into_alternate::<12>()
                .internal_pull_up(false)
                .speed(Speed::VeryHigh),
            sdcard_pins
                .1
                .into_alternate::<12>()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh),
            sdcard_pins
                .2
                .into_alternate::<12>()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh),
            sdcard_pins
                .3
                .into_alternate::<12>()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh),
            sdcard_pins
                .4
                .into_alternate::<12>()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh),
            sdcard_pins
                .5
                .into_alternate::<12>()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh),
            sdcard_pins.6.into_input(),
        );

        if detect.is_high() {
            return Err(SdmmcError::DeviceError(DeviceError::NoCard));
        }
        let mut sdmmc: Sdmmc<_, SdCard> = sdmmc1.sdmmc((clk, cmd, d0, d1, d2, d3), prec, clocks);
        sdmmc.init(10.MHz()).map_err(SdmmcError::DeviceError)?;
        let size = sdmmc.card().map_err(SdmmcError::DeviceError)?.size();
        Ok(FileIo {
            size,
            sdmmc: RefCell::new(sdmmc),
        })
    }
}

impl BlockDevice for FileIo {
    type Error = FileSysError;

    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        let mut sdmmc = self.sdmmc.borrow_mut();
        let start = start_block_idx.0;
        for block_idx in start..(start + blocks.len() as u32) {
            sdmmc
                .read_block(
                    block_idx,
                    &mut blocks[(block_idx - start) as usize].contents,
                )
                .map_err(SdmmcError::DeviceError)?;
        }
        Ok(())
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        let mut sdmmc = self.sdmmc.borrow_mut();
        let start = start_block_idx.0;
        for block_idx in start..(start + blocks.len() as u32) {
            let block = &blocks[(block_idx - start) as usize].contents;
            sdmmc
                .write_block(block_idx, block)
                .map_err(SdmmcError::DeviceError)?;
        }
        Ok(())
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(BlockCount((self.size / 512u64) as u32))
    }
}

type VolMgr = VolumeManager<FileIo, TimeSource>;

static mut FILE_SYS: Option<FileSys> = None;

/// Access to the file system of the SdCard
///
/// The file system may only be accessed when the application is started or when all interrupts
/// are disabled. The background to this is that the SDIO driver of the HAL is not resistant to
/// interrupts during access. For this reason, access from different thread contexts is not
/// protected.
pub struct FileSys {
    vol_mgr: VolMgr,
}

impl FileSys {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        pins: SdcardPins,
        //        time_source: TimeSource,
        sdmmc1: SDMMC1,
        prec: Sdmmc1,
        clocks: &CoreClocks,
    ) -> Result<(), FileSysError> {
        let file_io = FileIo::new(pins, sdmmc1, prec, clocks)?;
        let vol_mgr = VolumeManager::new(file_io, TimeSource);
        // ok, since access only provided from one thread
        unsafe { FILE_SYS = Some(FileSys { vol_mgr }) }
        Ok(())
    }

    pub fn vol_mgr(&mut self) -> &mut VolMgr {
        &mut self.vol_mgr
    }
}

pub fn get_filesys() -> Option<&'static mut FileSys> {
    // ok, since access only provided from one thread
    unsafe { FILE_SYS.as_mut() }
}
