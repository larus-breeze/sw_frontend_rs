#![no_main]
#![no_std]

mod driver;
use defmt::*;
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};
use stm32h7xx_hal::block;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    //USART1: PB14(TX), PB15(RX)
    //USART2: PA2(TX), PD6(RX) Connected by populating R62 & R63 

    let tx = gpiob.pb14.into_alternate();
    let rx = gpiob.pb15.into_alternate();

    let serial = dp
    .USART1
    .serial((tx, rx), 19_200.bps(), ccdr.peripheral.USART1, &ccdr.clocks)
    .unwrap();

    let (mut tx, mut rx) = serial.split();

   
    loop {
        // Loop back test. Connect RX with TX line.  Received character shall be reported in the Terminal
        block!(tx.write(b'A')).ok();
        info!("Send character {}", b'A');

        let received =  block!(rx.read()).unwrap();
        info!("Received character {}", received);
        }
}
