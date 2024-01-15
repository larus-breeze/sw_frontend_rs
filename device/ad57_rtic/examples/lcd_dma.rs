#![no_main]
#![no_std]
mod driver;

use core::iter::{Cloned, Cycle};
use core::slice::Iter;
use corelib::Colors;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use cortex_m_rt::entry;
use embedded_graphics::prelude::*;
use stm32f4xx_hal::{
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    dma::StreamsTuple,
};

use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use fmc_lcd::{DataPins16, LcdPins};
use driver::*;

pub fn delay_ms(millis: u32) {
    let cycles = millis * 168_000;
    cortex_m::asm::delay(cycles)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    EepromOrI2c1,
    NoItemAvailable,
}

#[entry]
fn main() -> ! {
    // Setup clocks
    let _cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    trace!("init");

    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    let _dma2 = StreamsTuple::new(dp.DMA2);
    let mut _timer = MonoTimer::new(dp.TIM2, &clocks);

    let gpiob = dp.GPIOB.split();
    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();

    let mut backlight = gpiob.pb4.into_push_pull_output();
    backlight.set_high(); // Switch on the light

    let lcd_pins = LcdPins::new(
        DataPins16::new(
            gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
            gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
            gpiod.pd9, gpiod.pd10,
        ),
        gpiod.pd11,
        gpiod.pd4,
        gpiod.pd5,
        gpiod.pd7,
    );
    let lcd_reset = gpiod.pd3.into_push_pull_output();

    let (mut frame_buffer, mut display) = FrameBuffer::new(dp.FSMC, lcd_pins, lcd_reset);

    display.clear(Colors::Red).unwrap();

    // Draw some circles
    let test_colors = [
        Colors::Blue,
        Colors::Green,
        Colors::Yellow,
        Colors::Red,
        Colors::Gray,
        Colors::Pink,
        Colors::LightBlue,
    ];
    let center_points = [
        Point::new(70, 70),
        Point::new(170, 70),
        Point::new(170, 170),
        Point::new(70, 170),
    ];
    let mut drawer = ColoredCircleDrawer::new(&center_points, &test_colors);


    loop {
        drawer.draw(&mut display).unwrap();
        frame_buffer.flush();
        trace!("tick()");
        delay_ms(1000);
    }
}

/// Draws colored circles of various locations and colors
struct ColoredCircleDrawer<'a> {
    /// Infinite iterator over circle center points
    centers: Cloned<Cycle<Iter<'a, Point>>>,
    /// Infinite iterator over Rgb565 colors
    colors: Cloned<Cycle<Iter<'a, Colors>>>,
}

impl<'a> ColoredCircleDrawer<'a> {
    pub fn new(centers: &'a [Point], colors: &'a [Colors]) -> Self {
        ColoredCircleDrawer {
            centers: centers.iter().cycle().cloned(),
            colors: colors.iter().cycle().cloned(),
        }
    }

    /// Draws one circle onto a target
    pub fn draw<T>(&mut self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = Colors>,
    {
        let center = self.centers.next().unwrap();
        let color = self.colors.next().unwrap();

        Circle::new(center, 50)
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(target)
    }
}
