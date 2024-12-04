#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp, cp);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        loop {
            info!("LED on");
            delay.delay_ms(500_u16);

            info!("LED off");
            delay.delay_ms(500_u16);
        }
    }
}
