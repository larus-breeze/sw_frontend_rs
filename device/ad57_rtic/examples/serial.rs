#![no_main]
#![no_std]

mod driver;
use defmt::*;
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    block,
    {pac, prelude::*, serial::config::Config},
};

#[entry]
fn main() -> ! {
    let _cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    info!("init");

    let gpioa = dp.GPIOA.split();
    // let gpiob = dp.GPIOB.split();

    // Unten: USART1 PA9(TX), PA10(RX)
    let tx = gpioa.pa9;
    let rx = gpioa.pa10;

    // Oben: USART3 PB10(TX), PB11(RX)
    // let tx = gpiob.pb10;
    // let rx = gpiob.pb11;

    let serial = dp
        // .USART3
        .USART1
        .serial((tx, rx), Config::default().baudrate(38400.bps()), &clocks)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    loop {
        // Loop back test. Connect RX with TX line.  Received character shall be reported in the Terminal
        block!(tx.write(b'A')).ok();
        info!("Send character {}", b'A');

        let received = block!(rx.read()).unwrap();
        info!("Received character {}", received);
    }
}
