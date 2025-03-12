#![no_main]
#![no_std]

mod driver;

use cortex_m_rt::entry;
use defmt::trace;
use defmt_rtt as _;
use embedded_sdmmc::{Mode, VolumeIdx};
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;

use driver::*;

#[entry]
fn main() -> ! {
    // Setup clocks
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    // Setup LED
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    let sdio_pins: SdioPins = (
        gpioc.pc12,
        gpiod.pd2.internal_pull_up(true),
        gpioc.pc8.internal_pull_up(true),
        gpioc.pc9.internal_pull_up(true),
        gpioc.pc10.internal_pull_up(true),
        gpioc.pc11.internal_pull_up(true),
    );
    //let sd_detect = gpioc.pc0.internal_pull_up(true).into_input();
    FileSys::new(dp.SDIO, &clocks, sdio_pins).unwrap();
    FILE_SYS.lock(|opt_fs| {
        if let Some(file_sys) = opt_fs {
            let volume = file_sys.vol_mgr().open_volume(VolumeIdx(0)).unwrap();

            let root_dir = file_sys.vol_mgr().open_root_dir(volume).unwrap();
            trace!("List all the directories and their info");
            file_sys
                .vol_mgr()
                .iterate_dir(root_dir, |entry| {
                    trace!("{}", defmt::Display2Format(&entry.name));
                })
                .unwrap();

            let file = file_sys
                .vol_mgr()
                .open_file_in_dir(root_dir, "TEST.TXT", Mode::ReadWriteCreateOrAppend)
                .unwrap();
            for _idx in [0..1000] {
                file_sys
                    .vol_mgr()
                    .write(file, "Dies ist ein Test, der zeigen soll... ".as_bytes())
                    .unwrap();
            }
            file_sys.vol_mgr().close_file(file).unwrap();
        }
    });

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);
    loop {
        delay.delay_ms(1000_u16);
        trace!("tick");
    }
}
