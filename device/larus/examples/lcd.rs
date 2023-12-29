#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;

use core::iter::{Cloned, Cycle};
use core::slice::Iter;

use cortex_m_rt::entry;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use stm32h7xx_hal::{
    pac::{CorePeripherals, Peripherals as DevicePeripherals},
    prelude::*,
    rcc::rec::FmcClkSel,
};

use driver::*;
use st7789::ST7789;

#[entry]
fn main() -> ! {
    // Setup clocks
    let mut cp = CorePeripherals::take().unwrap();
    let dp = DevicePeripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp);

    // Initialize system...
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();

    // Modify the kernel clock for FMC. See RM0433 Rev 7 Section 8.5.8.
    let pfmc = ccdr.peripheral.FMC;
    let pfmc = pfmc.kernel_clk_mux(FmcClkSel::Pll2R);

    let mut delay = cp.SYST.delay(ccdr.clocks);

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpiof = dp.GPIOF.split(ccdr.peripheral.GPIOF);

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
    let interface = LcdInterface::new(pfmc, dp.FMC, lcd_pins);

    let lcd_reset = gpioc.pc0.into_push_pull_output();
    let backlight_control = gpiof.pf5.into_push_pull_output();

    // Add LCD controller driver
    let mut lcd = ST7789::new(
        interface,
        Some(lcd_reset),
        Some(backlight_control),
        320,
        240,
    );
    // Initialise the display and clear the screen
    lcd.init(&mut delay).unwrap();
    lcd.set_orientation(st7789::Orientation::PortraitSwapped)
        .unwrap();
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
        delay.delay_ms(100u16);
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
