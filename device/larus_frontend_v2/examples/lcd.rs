#![no_main]
#![no_std]

use stm32h7xx_hal::{pac, prelude::*};

mod driver;

use defmt::*;
use defmt_rtt as _;
use driver::{FrameBuffer, LcdPins, Ltdc, LtdcPins, St7701s};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp, cp);
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let lcd_pins = LcdPins(gpioc.pc1, gpioc.pc2, gpioa.pa12, gpiod.pd14, gpioe.pe8);

    let ltdc_pins = LtdcPins(
        gpioc.pc0, gpioa.pa3, gpioa.pa4, gpioa.pa6, gpiob.pb0, gpioe.pe11, gpioe.pe12, gpioe.pe13,
        gpioe.pe14, gpioe.pe15, gpiob.pb10, gpiob.pb11, gpiod.pd10, gpioc.pc6, gpioc.pc7,
        gpioc.pc9, gpioa.pa8, gpioa.pa11, gpioc.pc10, gpiod.pd3, gpiob.pb8, gpiob.pb9,
    );

    St7701s::init(
        dp.SPI2,
        ccdr.peripheral.SPI2,
        lcd_pins,
        &ccdr.clocks,
        &mut delay,
    );

    let ltdc = Ltdc::init(
        dp.LTDC,
        ltdc_pins,
        ccdr.peripheral.LTDC,
        &ccdr.clocks,
        &mut delay,
    );

    let mut frame_buffer = FrameBuffer::new(ltdc);
    let mut buf = frame_buffer.swap_buffers();

    fn fill_fb(color: u8, buf: &mut [u8]) {
        for idx in 0..buf.len() {
            buf[idx] = color;
        }
    }

    loop {
        fill_fb(137, buf);
        buf = frame_buffer.swap_buffers();
        info!("yellow");
        delay.delay_ms(2000_u16);

        fill_fb(112, buf); // red
        buf = frame_buffer.swap_buffers();
        info!("red");
        delay.delay_ms(2000_u16);

        fill_fb(9, buf); // blue
        buf = frame_buffer.swap_buffers();
        info!("blue");
        delay.delay_ms(2000_u16);

        fill_fb(50, buf); // green
        buf = frame_buffer.swap_buffers();
        info!("green");
        delay.delay_ms(2000_u16);
    }
}
