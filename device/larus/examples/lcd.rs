#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;

use core::cell::RefCell;
use core::iter::{Cloned, Cycle};
use core::slice::Iter;
use corelib::Colors;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use stm32h7xx_hal::{
    pac::{interrupt, CorePeripherals, Peripherals as DevicePeripherals, NVIC},
    prelude::*,
    rcc::rec::FmcClkSel,
};

use driver::*;
use st7789::ST7789;

static FRAME_BUFFER: Mutex<RefCell<Option<FrameBuffer>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Setup clocks
    let mut cp = CorePeripherals::take().unwrap();
    let dp = DevicePeripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp);
    let _mono = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);

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

    let stream0 = stm32h7xx_hal::dma::mdma::StreamsTuple::new(dp.MDMA, ccdr.peripheral.MDMA).0;

    let (frame_buffer, mut display) = FrameBuffer::new(lcd, stream0);

    cortex_m::interrupt::free(|cs| {
        FRAME_BUFFER.borrow(cs).replace(Some(frame_buffer));
    });

    display.clear(Colors::Blue).unwrap();

    unsafe {
        cp.NVIC.set_priority(interrupt::MDMA, 1);
        NVIC::unmask(interrupt::MDMA);
    }

    // Draw some circles
    let test_colors = [
        Colors::White,
        Colors::Blue,
        Colors::Green,
        Colors::Red,
        Colors::Yellow,
        Colors::Cyan,
        Colors::Gray,
        Colors::Magenta,
        Colors::Red,
        Colors::Green,
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

        let ts = timestamp();
        cortex_m::interrupt::free(|cs| {
            let mut rc = FRAME_BUFFER.borrow(cs).borrow_mut();
            let frame_buffer = rc.as_mut().unwrap();
            frame_buffer.flush()
        });
        trace!("frame_buffer.flush() {} µs", timestamp() - ts);

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

#[interrupt]
fn MDMA() {
    cortex_m::interrupt::free(|cs| {
        let mut rc = FRAME_BUFFER.borrow(cs).borrow_mut();
        let frame_buffer = rc.as_mut().unwrap();
        frame_buffer.on_interrupt()
    });
}
