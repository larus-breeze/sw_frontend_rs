#![no_main]
#![no_std]

use cortex_m_rt::entry;
use {defmt_rtt as _, panic_probe as _};
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    // Setup clocks
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    // Setup LED
    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb4.into_push_pull_output();

    let mut state = false;

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        if state {
            led.set_high();
            state = false;
        } else {
            led.set_low();
            state = true;
        }
        delay.delay_ms(1000_u16);
    }
}

