#![macro_use]
#![allow(unused_macros)]

#[allow(unused)]
use corelib::Event;
#[allow(unused)]
use heapless::mpmc::MpMcQueue;

mod amplifier;
mod canbus;
mod clut_colors;
mod display;
mod eeprom;
mod file_sys;
mod frame_buffer;
mod i2c_mgr;
mod init;
mod keyboard;
mod ltdc;
mod nmea;
mod panic;
mod rng_uuid;
mod sound;
mod st7701s;
mod sys_timer;

pub use amplifier::*;
pub use canbus::*;
pub use clut_colors::*;
pub use display::*;
pub use eeprom::*;
pub use file_sys::*;
pub use frame_buffer::*;
pub use i2c_mgr::*;
pub use init::*;
pub use keyboard::*;
pub use ltdc::*;
pub use nmea::*;
pub use panic::*;
pub use rng_uuid::*;
pub use sound::*;
pub use st7701s::*;
pub use sys_timer::*;

pub type QEvents = MpMcQueue<Event, 8>;
