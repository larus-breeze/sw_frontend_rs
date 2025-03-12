use core::cell::RefCell;
use corelib::Lock;
use embedded_sdmmc::{
    Block, BlockCount, BlockDevice, BlockIdx, Error as SdmmcError, VolumeManager,
};
use stm32h7xx_hal::{
    device::SDMMC1,
    gpio::{Alternate, Input, Pin, Speed},
    prelude::*,
    rcc::rec::Sdmmc1,
    rcc::CoreClocks,
    sdmmc::{Error as DeviceError, SdCard, Sdmmc, SdmmcExt},
};
type FileSysError = SdmmcError<DeviceError>;

pub struct SdcardPins {
    clk: Pin<'C', 12, Alternate<12>>,
    cmd: Pin<'D', 2, Alternate<12>>,
    d0: Pin<'C', 8, Alternate<12>>,
    d1: Pin<'C', 9, Alternate<12>>,
    d2: Pin<'C', 10, Alternate<12>>,
    d3: Pin<'C', 11, Alternate<12>>,
    detect: Pin<'E', 3, Input>,
}

impl SdcardPins {
    pub fn new(
        clk: Pin<'C', 12>,
        cmd: Pin<'D', 2>,
        d0: Pin<'C', 8>,
        d1: Pin<'C', 9>,
        d2: Pin<'C', 10>,
        d3: Pin<'C', 11>,
        detect: Pin<'E', 3>,
    ) -> Self {
        let clk = clk
            .into_alternate::<12>()
            .internal_pull_up(false)
            .speed(Speed::VeryHigh);
        let cmd = cmd
            .into_alternate::<12>()
            .internal_pull_up(true)
            .speed(Speed::VeryHigh);
        let d0 = d0
            .into_alternate::<12>()
            .internal_pull_up(true)
            .speed(Speed::VeryHigh);
        let d1 = d1
            .into_alternate::<12>()
            .internal_pull_up(true)
            .speed(Speed::VeryHigh);
        let d2 = d2
            .into_alternate::<12>()
            .internal_pull_up(true)
            .speed(Speed::VeryHigh);
        let d3 = d3
            .into_alternate::<12>()
            .internal_pull_up(true)
            .speed(Speed::VeryHigh);
        let detect = detect.into_pull_down_input();
        SdcardPins {
            clk,
            cmd,
            d0,
            d1,
            d2,
            d3,
            detect,
        }
    }
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

pub struct FileIo {
    size: u64,
    sdmmc: RefCell<Sdmmc<SDMMC1, SdCard>>,
}

impl FileIo {
    pub fn new(
        pins: SdcardPins,
        sdmmc1: SDMMC1,
        prec: Sdmmc1,
        clocks: &CoreClocks,
    ) -> Result<Self, FileSysError> {
        if pins.detect.is_low() {
            return Err(SdmmcError::DeviceError(DeviceError::NoCard));
        }
        let pins = (pins.clk, pins.cmd, pins.d0, pins.d1, pins.d2, pins.d3);
        let mut sdmmc: Sdmmc<_, SdCard> = sdmmc1.sdmmc(pins, prec, clocks);
        sdmmc
            .init(10.MHz())
            .map_err(|e| SdmmcError::DeviceError(e))?;
        let size = sdmmc.card().map_err(|e| SdmmcError::DeviceError(e))?.size();
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
        // unsafe is ok, becaus we are the only one knowing and using sdmmc
        let mut sdmmc = unsafe { self.sdmmc.borrow_mut() };
        let start = start_block_idx.0;
        for block_idx in start..(start + blocks.len() as u32) {
            sdmmc
                .read_block(
                    block_idx,
                    &mut blocks[(block_idx - start) as usize].contents,
                )
                .map_err(|e| SdmmcError::DeviceError(e))?;
        }
        Ok(())
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        // unsafe is ok, becaus we are the only one knowing and using sdmmc
        let mut sdmmc = unsafe { self.sdmmc.borrow_mut() };
        let start = start_block_idx.0;
        for block_idx in start..(start + blocks.len() as u32) {
            let block = &blocks[(block_idx - start) as usize].contents;
            sdmmc
                .write_block(block_idx, block)
                .map_err(|e| SdmmcError::DeviceError(e))?;
        }
        Ok(())
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(BlockCount((self.size / 512u64) as u32))
    }
}

type VolMgr = VolumeManager<FileIo, TimeSource>;

pub static FILE_SYS: Lock<FileSys> = Lock::new();

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
    pub fn new(
        pins: SdcardPins,
        //        time_source: TimeSource,
        sdmmc1: SDMMC1,
        prec: Sdmmc1,
        clocks: &CoreClocks,
    ) -> Result<(), FileSysError> {
        let file_io = FileIo::new(pins, sdmmc1, prec, clocks)?;
        let mut vol_mgr = VolumeManager::new(file_io, TimeSource);
        FILE_SYS.set(FileSys { vol_mgr });
        Ok(())
    }

    pub fn vol_mgr(&mut self) -> &mut VolMgr {
        &mut self.vol_mgr
    }
}
