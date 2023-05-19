//!
//! Demonstrates use of the Flexible Static Memory Controller to interface with an ST7789 LCD
//! controller
//!
//! Hardware required: an STM32F412G-DISCO board
//!
//! Procedure: Compile this example, load it onto the microcontroller, and run it.
//!
//! Example run command: `cargo run --release --features stm32f412,rt,fsmc_lcd --example st7789-lcd`
//!
//! Expected behavior: The display shows a black background with four colored circles. Periodically,
//! the color of each circle changes.
//!
//! Each circle takes a noticeable amount of time to draw, from top to bottom. Because
//! embedded-graphics by default does not buffer anything in memory, it sends one pixel at a time
//! to the LCD controller. The LCD interface can transfer rectangular blocks of pixels more quickly.
//!

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod driver;
mod utils;

use vario_display::*;
use embedded_graphics::prelude::*;

use embedded_hal::blocking::delay::DelayUs;

pub fn main(
    display: &mut driver::Display<stm32f4xx_hal::fsmc_lcd::Lcd<stm32f4xx_hal::fsmc_lcd::SubBank1>, stm32f4xx_hal::gpio::Pin<stm32f4xx_hal::gpio::Output<stm32f4xx_hal::gpio::PushPull>, 'D', 11>, core::convert::Infallible>,
    delay: &mut impl DelayUs<u32>) {


    let mut blackboard = Blackboard::new();
    blackboard.climb_rate = 2.3;
    blackboard.average_climb_rate = 1.6;
    blackboard.mc_cready = 1.0;
    blackboard.wind_angle = 30.0.deg();
    blackboard.wind_speed = 30.0;
    blackboard.average_wind_angle = 60.0.deg();
    blackboard.average_wind_speed = 25.0;
    blackboard.speed_to_fly_dif = 15.0;

    let mut vario = VarioDisplay::new();

    vario.draw(display, &blackboard).unwrap();

    loop {
        vario.draw(display, &blackboard).unwrap();
        display.flush().unwrap();
        delay.delay_us(1000_000_u32);
    } /**/
}
