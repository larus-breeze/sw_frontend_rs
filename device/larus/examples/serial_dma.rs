#![no_main]
#![no_std]

mod driver;
use driver::{NmeaRx, NmeaTx, NmeaTxRx};

use defmt::*;
use defmt_rtt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32h7xx_hal::{dma::dma::StreamsTuple, interrupt, pac, prelude::*};

const TEST_STREAM: &[u8] = b"Dies ist ein Test, der zeigen soll";

static NMEA_RX: Mutex<RefCell<Option<NmeaRx>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn USART1() {
    info!("char match interrupt");
    cortex_m::interrupt::free(|cs| {
        let mut rc = NMEA_RX.borrow(cs).borrow_mut();
        let rx = rc.as_mut().unwrap();
        rx.on_interrupt();
        while let Some(chunk) = rx.read() {
            info!("chunk '{:?}'", chunk);
        }
    })
}

static NMEA_TX: Mutex<RefCell<Option<NmeaTx>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn DMA1_STR1() {
    info!("NmeaTx complete interrupt");
    cortex_m::interrupt::free(|cs| {
        let mut rc = NMEA_TX.borrow(cs).borrow_mut();
        let tx = rc.as_mut().unwrap();
        let _ = tx.ready(); // Ack
    })
}

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let streams = StreamsTuple::new(dp.DMA1, ccdr.peripheral.DMA1);

    let (mut nmea_tx, nmea_rx) = NmeaTxRx::new(
        streams.1,
        streams.2,
        gpiob.pb14,
        gpiob.pb15,
        dp.USART1,
        ccdr.peripheral.USART1,
        &ccdr.clocks,
    );
    unsafe {
        cp.NVIC.set_priority(interrupt::DMA1_STR1, 1);
        pac::NVIC::unmask(interrupt::DMA1_STR1);

        cp.NVIC.set_priority(interrupt::USART1, 1);
        pac::NVIC::unmask(interrupt::USART1);
    }

    nmea_tx.send(TEST_STREAM);

    cortex_m::interrupt::free(|cs| {
        NMEA_RX.borrow(cs).replace(Some(nmea_rx));
        NMEA_TX.borrow(cs).replace(Some(nmea_tx));
    });

    loop {}
}
