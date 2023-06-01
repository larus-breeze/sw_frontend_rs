use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use core::{
    convert::Infallible,
};
use stm32f4xx_hal::{
        pac,
        prelude::*,
        gpio::{Output, Pin, PushPull, Speed},
        fsmc_lcd::{ChipSelect1, FsmcLcd, LcdPins, Timing, AccessMode},
    };
use crate::driver::{
    r61580::{R61580, Orientation},
    Display,
    MonoTimer, display::FrameBuffer,
};

pub fn delay_ms(millis: u32) {
    let cycles = millis*168_000;
    cortex_m::asm::delay(cycles)
}

pub fn hw_init(
    device: pac::Peripherals, 
    _core: cortex_m::peripheral::Peripherals,
) -> (
    Pin<Output<PushPull>, 'B', 4>,  // backlight
    Display<Pin<Output<PushPull>, 'D', 3>, Infallible>,
    MonoTimer<pac::TIM5, 1_000_000>,
    FrameBuffer,
) {

        // Setup clocks
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr
            .use_hse(16.mhz())
            .sysclk(168.mhz())
            .hclk(168.mhz())
            .freeze();

        // Setup timer
        let mono = super::mono::MonoTimer::new(device.TIM5, &clocks);

        // Take ownership of GPIO ports
        let gpiob = device.GPIOB.split();
        let gpiod = device.GPIOD.split();
        let gpioe = device.GPIOE.split();

        // Setup LCD driver peripheral of STM32F407
        let mut backlight = gpiob.pb4.into_push_pull_output();

        let lcd_pins = LcdPins {
            data: (
                gpiod.pd14.into_alternate().set_speed(Speed::VeryHigh),    // D0
                gpiod.pd15.into_alternate().set_speed(Speed::VeryHigh),    // D1
                gpiod.pd0.into_alternate().set_speed(Speed::VeryHigh),     // D2
                gpiod.pd1.into_alternate().set_speed(Speed::VeryHigh),     // D3
                gpioe.pe7.into_alternate().set_speed(Speed::VeryHigh),     // D4
                gpioe.pe8.into_alternate().set_speed(Speed::VeryHigh),     // D5
                gpioe.pe9.into_alternate().set_speed(Speed::VeryHigh),     // D6
                gpioe.pe10.into_alternate().set_speed(Speed::VeryHigh),    // D7
                gpioe.pe11.into_alternate().set_speed(Speed::VeryHigh),    // D8
                gpioe.pe12.into_alternate().set_speed(Speed::VeryHigh),    // D9
                gpioe.pe13.into_alternate().set_speed(Speed::VeryHigh),    // D10
                gpioe.pe14.into_alternate().set_speed(Speed::VeryHigh),    // D11
                gpioe.pe15.into_alternate().set_speed(Speed::VeryHigh),    // D12
                gpiod.pd8.into_alternate().set_speed(Speed::VeryHigh),     // D13
                gpiod.pd9.into_alternate().set_speed(Speed::VeryHigh),     // D14
                gpiod.pd10.into_alternate().set_speed(Speed::VeryHigh),    // D15
            ),
            address: gpiod.pd11.into_alternate().set_speed(Speed::VeryHigh),
            read_enable: gpiod.pd4.into_alternate().set_speed(Speed::VeryHigh),
            write_enable: gpiod.pd5.into_alternate().set_speed(Speed::VeryHigh),
            chip_select: ChipSelect1(gpiod.pd7.into_alternate().set_speed(Speed::VeryHigh)),
        };
        let lcd_reset = gpiod.pd3.into_push_pull_output().set_speed(Speed::High);

        let timing = 
            Timing::default().data(3).address_setup(6).bus_turnaround(0).address_hold(1).access_mode(AccessMode::ModeB);
        let (_fsmc, _interface) = 
            FsmcLcd::new(device.FSMC, lcd_pins, &timing, &timing);

        // Initialize RG61580 LCD driver
        let mut lcd = R61580::new(lcd_reset);
        lcd.init();
        let _ = lcd.set_orientation(Orientation::Portrait);

        // Initialize the display, clear the screen and turn off the backlight
        let frame_buffer = FrameBuffer::new();
        let display: Display<Pin<Output<PushPull>, 'D', 3>, Infallible> = Display::new(lcd, frame_buffer.split());
        backlight.set_low();

        trace!("AD57 initialized");
        
        (backlight, display, mono, frame_buffer)
}
