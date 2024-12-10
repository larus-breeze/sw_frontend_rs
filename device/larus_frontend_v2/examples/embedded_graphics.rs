#![no_main]
#![no_std]

use corelib::{Colors, DrawImage, images, themes};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point};
use stm32h7xx_hal::{pac, prelude::*};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

mod driver;

use defmt::*;
use defmt_rtt as _;
use driver::{Display, FrameBuffer, LcdPins, Ltdc, LtdcPins, St7701s};

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

    let frame_buffer = FrameBuffer::new(ltdc);
    let mut display = Display::new(frame_buffer);

    loop {
        display.clear(Colors::Black).unwrap();
        display
            .draw_img(
                images::WP_VARIO_IMG,
                Point::new(0, 0),
                Some(Colors::White),
            )
            .unwrap();
        display.show();
        delay.delay_ms(2000_u16);

        display.clear(Colors::Blue).unwrap();
        themes::FONT_BIG
            .render_aligned(
                "Hello",
                Point::new(240, 240),
                VerticalPosition::Center,
                HorizontalAlignment::Center,
                FontColor::Transparent(Colors::White),
                &mut display,
            )
            .unwrap();
        display.show();
        delay.delay_ms(2000_u16);
    }
}
