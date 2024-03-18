#![no_main]
#![no_std]

mod driver;
mod utils;

use defmt::trace;
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    watchdog::IndependentWatchdog,
};

use driver::*;
use utils::*;

pub fn delay_ms(millis: u32) {
    let cycles = millis * 168_000;
    cortex_m::asm::delay(cycles)
}

#[entry]
fn main() -> ! {
    // Setup clocks
    let _cp = CorePeripherals::take().unwrap();
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

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
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

    let _ = FileSys::new(dp.SDIO, &clocks, sdio_pins);
    let _ = ResetWatch::init();

    let keyboard_pins = KeyboardPins::new(gpioa.pa7, gpioc.pc5, gpiob.pb0, gpiob.pb1, gpioa.pa6);
    let mut keyboard = Keyboard::new(keyboard_pins);

    let mut watchdog = IndependentWatchdog::new(dp.IWDG);
    watchdog.start(1000.millis());

    trace!("reset");

    loop {
        delay_ms(20);
        watchdog.feed();
        let key = keyboard.tick();
        if key != Key::None {
            trace!("{}", key as u32);
        }
        match key {
            Key::Button4 => loop {},  // Watchdog test
            Key::Button3 => panic!(), // Panic test
            _ => (),
        }
    }
}
