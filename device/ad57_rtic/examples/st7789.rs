// ***********************************************************
//
//                  Not yet tested
//
// ***********************************************************

#![no_main]
#![no_std]

mod driver;

use core::iter::{Cloned, Cycle};
use core::slice::Iter;
use cortex_m_rt::entry;
use defmt_rtt as _;
use stm32f4xx_hal::{
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    fsmc_lcd::{AccessMode, Timing, FsmcLcd, LcdPins, DataPins16},
};
use embedded_graphics::{
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
};
use st7789::ST7789;

#[entry]
fn main() -> ! {
    // Setup clocks
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    // Setup LED
    let gpiob = dp.GPIOB.split();
    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();


    // Pins connected to the LCD on the 32F412GDISCOVERY board
    use stm32f4xx_hal::gpio::alt::fsmc as alt;
    let lcd_pins = LcdPins::new(
        DataPins16::new(
            gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
            gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
            gpiod.pd9, gpiod.pd10,
        ),
        alt::Address::from(gpiod.pd11),
        gpiod.pd4,
        gpiod.pd5,
        alt::ChipSelect1::from(gpiod.pd7),
    );
    let lcd_reset = gpiod.pd3.into_push_pull_output();
    let backlight_control = gpiob.pb4.into_push_pull_output();

    let timing = Timing::default()
    .data(3)
    .address_setup(6)
    .bus_turnaround(0)
    .address_hold(1)
    .access_mode(AccessMode::ModeB);

    let (_fsmc, interface) = FsmcLcd::new(dp.FSMC, lcd_pins, &timing, &timing);

    // The 32F412GDISCOVERY board has an FRD154BP2902-CTP LCD. There is no easily available
    // datasheet, so the behavior of this code is based on the working demonstration C code:
    // https://github.com/STMicroelectronics/STM32CubeF4/blob/e084518f363e04344dc37822210a75e87377b200/Drivers/BSP/STM32412G-Discovery/stm32412g_discovery_lcd.c
    // https://github.com/STMicroelectronics/STM32CubeF4/blob/e084518f363e04344dc37822210a75e87377b200/Drivers/BSP/Components/st7789h2/st7789h2.c

    // Add LCD controller driver
    let mut lcd = ST7789::new(
        interface,
        Some(lcd_reset),
        Some(backlight_control),
        320,
        240,
    );

    let mut delay = cp.SYST.delay(&clocks);

    // Initialise the display and clear the screen
    lcd.init(&mut delay).unwrap();
    lcd.set_orientation(st7789::Orientation::PortraitSwapped)
        .unwrap();

    lcd.clear(Rgb565::new(0, 80, 128)).unwrap();

    // Initialise the display and clear the screen
    lcd.init(&mut delay).unwrap();
    lcd.clear(Rgb565::BLACK).unwrap();

    // Draw some circles
    let test_colors = [
        Rgb565::new(0x4e >> 3, 0x79 >> 2, 0xa7 >> 3),
        Rgb565::new(0xf2 >> 3, 0x8e >> 2, 0x2b >> 3),
        Rgb565::new(0xe1 >> 3, 0x57 >> 2, 0x59 >> 3),
        Rgb565::new(0x76 >> 3, 0xb7 >> 2, 0xb2 >> 3),
        Rgb565::new(0x59 >> 3, 0xa1 >> 2, 0x4f >> 3),
        Rgb565::new(0xed >> 3, 0xc9 >> 2, 0x48 >> 3),
        Rgb565::new(0xb0 >> 3, 0x7a >> 2, 0xa1 >> 3),
        Rgb565::new(0xff >> 3, 0x9d >> 2, 0xa7 >> 3),
        Rgb565::new(0x9c >> 3, 0x75 >> 2, 0x5f >> 3),
        Rgb565::new(0xba >> 3, 0xb0 >> 2, 0xac >> 3),
    ];
    let center_points = [
        Point::new(70, 70),
        Point::new(170, 70),
        Point::new(170, 170),
        Point::new(70, 170),
    ];
    let mut drawer = ColoredCircleDrawer::new(&center_points, &test_colors);
    loop {
        drawer.draw(&mut lcd).unwrap();
        delay.delay_ms(100 as u8);
    }
}

/// Draws colored circles of various locations and colors
struct ColoredCircleDrawer<'a> {
    /// Infinite iterator over circle center points
    centers: Cloned<Cycle<Iter<'a, Point>>>,
    /// Infinite iterator over Rgb565 colors
    colors: Cloned<Cycle<Iter<'a, Rgb565>>>,
}

impl<'a> ColoredCircleDrawer<'a> {
    pub fn new(centers: &'a [Point], colors: &'a [Rgb565]) -> Self {
        ColoredCircleDrawer {
            centers: centers.iter().cycle().cloned(),
            colors: colors.iter().cycle().cloned(),
        }
    }

    /// Draws one circle onto a target
    pub fn draw<T>(&mut self, target: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = Rgb565>,
    {
        let center = self.centers.next().unwrap();
        let color = self.colors.next().unwrap();

        Circle::new(center, 50)
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(target)
    }
}