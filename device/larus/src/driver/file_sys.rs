use embedded_sdmmc::{
    Controller, Error as SdmmcError,
};
use stm32h7xx_hal::{
    device::SDMMC1,
    gpio::{Alternate, Input, Pin, Speed},
    pac,
    prelude::*,
    rcc::rec::Sdmmc1,
    rcc::CoreClocks,
    sdmmc::{Error as DeviceError, SdCard, Sdmmc, SdmmcBlockDevice, SdmmcExt},
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

type FatFs = Controller<SdmmcBlockDevice<Sdmmc<pac::SDMMC1, SdCard>>, TimeSource>;

pub struct FileSys {
    _detect: Pin<'E', 3, Input>,
    fat_fs: FatFs,
}

impl FileSys {
    pub fn new(
        pins: SdcardPins,
        //        time_source: TimeSource,
        sdmmc1: SDMMC1,
        prec: Sdmmc1,
        clocks: &CoreClocks,
    ) -> Result<FileSys, FileSysError> {
        if pins.detect.is_low() {
            return Err(SdmmcError::DeviceError(DeviceError::NoCard));
        }
        let mut sdmmc: Sdmmc<_, SdCard> = sdmmc1.sdmmc(
            (pins.clk, pins.cmd, pins.d0, pins.d1, pins.d2, pins.d3),
            prec,
            clocks,
        );
        sdmmc.init(10.MHz())?;
        let fat_fs = Controller::new(sdmmc.sdmmc_block_device(), TimeSource);
        Ok(Self {
            fat_fs,
            _detect: pins.detect,
        })
    }

    pub fn fat(&mut self) -> &mut FatFs {
        &mut self.fat_fs
    }
}
