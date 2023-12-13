#![no_main]
#![no_std]

use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Configure PE1 as output.
    let mut led = gpioe.pe1.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        loop {
            led.set_high();
            delay.delay_ms(500_u16);

            led.set_low();
            delay.delay_ms(500_u16);
        }
    }
}