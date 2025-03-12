#![no_main]
#![no_std]
mod driver;

use defmt::*;
use defmt_rtt as _;

use embedded_sdmmc::VolumeIdx;
use stm32h7xx_hal::{pac, prelude::*, rcc};

use driver::*;

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    // Get peripherals
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let ccdr = dp
        .RCC
        .constrain()
        .sys_ck(200.MHz())
        .pll1_strategy(rcc::PllConfigStrategy::Iterative)
        .pll1_q_ck(100.MHz())
        .pll2_strategy(rcc::PllConfigStrategy::Iterative)
        .pll3_strategy(rcc::PllConfigStrategy::Iterative)
        .freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    //let mut delay = cp.SYST.delay(ccdr.clocks);

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    let pins = SdcardPins::new(
        gpioc.pc12, gpiod.pd2, gpioc.pc8, gpioc.pc9, gpioc.pc10, gpioc.pc11, gpioe.pe3,
    );

    FileSys::new(pins, dp.SDMMC1, ccdr.peripheral.SDMMC1, &ccdr.clocks).unwrap();

    FILE_SYS.lock_during_use(|opt_fs| {
        if let Some(fs) = opt_fs {
            let mut volume = fs.vol_mgr().open_volume(VolumeIdx(0)).unwrap();
            let mut root_dir = volume.open_root_dir().unwrap();
            root_dir
                .iterate_dir(|entry| {
                    trace!("{}", defmt::Display2Format(&entry.name));
                })
                .unwrap();
        }
    });

    loop {
        cortex_m::asm::nop()
    }
}
