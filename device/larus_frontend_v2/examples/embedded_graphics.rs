#![no_main]
#![no_std]

use corelib::{Colors, DrawImage};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point};
use stm32h7xx_hal::{pac, prelude::*};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use u8g2_fonts::{fonts, FontRenderer};

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

    let frame_buffer = FrameBuffer::new(ltdc);
    let mut display = Display::new(frame_buffer);

    let font = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();

    loop {
        display.clear(Colors::Black).unwrap();
        display
            .draw_img(
                &*include_bytes!("../assets/wp_vario.lif"),
                Point::new(0, 0),
                Some(Colors::White),
            )
            .unwrap();
        display.show();
        delay.delay_ms(2000_u16);

        display.clear(Colors::Blue).unwrap();
        font.render_aligned(
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
