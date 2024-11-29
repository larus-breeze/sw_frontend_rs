#![no_main]
#![no_std]

use stm32h7xx_hal::{pac, prelude::*};

mod driver;

use defmt::*;
use defmt_rtt as _;
use driver::{LcdPins, St7701s};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp);
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let lcd_pins = LcdPins(gpioc.pc1, gpioc.pc2, gpioa.pa12, gpiod.pd14, gpioe.pe8);

    St7701s::init(
        dp.SPI2,
        ccdr.peripheral.SPI2,
        lcd_pins,
        &ccdr.clocks,
        &mut delay,
    );

    loop {
        loop {
            info!("LED on");
            delay.delay_ms(500_u16);

            info!("LED off");
            delay.delay_ms(500_u16);
        }
    }
}
