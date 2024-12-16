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
    let gpiog = dp.GPIOG.split(ccdr.peripheral.GPIOG);
    let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
    let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);
    let gpioj = dp.GPIOJ.split(ccdr.peripheral.GPIOJ);
    let gpiok = dp.GPIOK.split(ccdr.peripheral.GPIOK);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let lcd_pins = LcdPins(gpiob.pb5, gpiog.pg9, gpiog.pg11, gpioh.ph4, gpioh.ph3);

    let ltdc_pins = LtdcPins(
        gpioa.pa5, gpioa.pa8, gpioc.pc0, gpiod.pd3, gpiog.pg6, gpiog.pg7, gpiog.pg10, gpioi.pi11,
        gpioi.pi12, gpioi.pi13, gpioj.pj1, gpioj.pj2, gpioj.pj9, gpioj.pj11, gpioj.pj12,
        gpioj.pj15, gpiok.pk0, gpiok.pk3, gpiok.pk4, gpiok.pk5, gpiok.pk6, gpiok.pk7,
    );

    St7701s::init(
        dp.SPI1,
        ccdr.peripheral.SPI1,
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
