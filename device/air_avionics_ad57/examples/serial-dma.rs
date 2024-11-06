#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use driver::*;

use stm32f4xx_hal::{dma::StreamsTuple, interrupt, pac, prelude::*};

const TEST_STREAM: &[u8] = b"Dies ist ein Test, der zeigen soll";

static NMEA_TX: Mutex<RefCell<Option<NmeaTx>>> = Mutex::new(RefCell::new(None));

#[interrupt]
#[allow(non_snake_case)]
fn DMA2_STREAM7() {
    cortex_m::interrupt::free(|cs| {
        let mut rc = NMEA_TX.borrow(cs).borrow_mut();
        let nmea_tx = rc.as_mut().unwrap();
        info!("tx ready {}", nmea_tx.ready());
    })
}

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
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

    let rcc = unsafe { &*pac::RCC::ptr() };
    rcc.ahb1enr.modify(|_, w| w.dma2en().set_bit()); // enable ahb1 clock for dma2

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA2_STREAM7);
    }

    info!("init");

    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();
    let streams = StreamsTuple::new(dp.DMA2);

    let (mut nmea_tx, mut nmea_rx) = NmeaTxRx::new(
        dp.USART1, streams.5, streams.7, gpioa.pa9, gpioa.pa10, &clocks,
    );

    nmea_tx.send(TEST_STREAM);

    cortex_m::interrupt::free(|cs| {
        NMEA_TX.borrow(cs).replace(Some(nmea_tx));
    });

    loop {
        if let Some(chunk) = nmea_rx.read() {
            cortex_m::interrupt::free(|cs| {
                let mut rc = NMEA_TX.borrow(cs).borrow_mut();
                let nmea_tx = rc.as_mut().unwrap();
                nmea_tx.send(chunk);
            })
        }
        delay.delay_ms(20_u32);
    }
}
