#![no_main]
#![no_std]

mod driver;

use defmt::trace;
use cortex_m_rt::entry;
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;
use {defmt_rtt as _, panic_probe as _};
use embedded_sdmmc::{VolumeIdx, Mode};

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
    let mut file_sys = FileSys::new(dp.SDIO, &clocks, sdio_pins).unwrap();
    let volume = file_sys.fat().open_volume(VolumeIdx(0)).unwrap();

    let root_dir = file_sys.fat().open_root_dir(volume).unwrap();
    trace!("List all the directories and their info");
    file_sys.fat()
        .iterate_dir(root_dir, |entry| {
            trace!("{}", defmt::Display2Format(&entry.name));
        })
        .unwrap();

    let file = file_sys.fat().open_file_in_dir(root_dir, "TEST.TXT", Mode::ReadWriteAppend).unwrap();
    for _idx in [0..1000] {
        file_sys.fat().write(file, "Dies ist ein Test, der zeigen soll... ".as_bytes()).unwrap();
    };
    file_sys.fat().close_file(file).unwrap();

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        delay.delay_ms(1000_u16);
        trace!("tick");
    }
}
