#![macro_use]
#![allow(unused_macros)]

#[allow(unused)]
use corelib::Event;
#[allow(unused)]
use heapless::mpmc::MpMcQueue;

mod amplifier;
mod canbus;
mod eeprom;
mod file_sys;
mod fmc_lcd;
mod frame_buffer;
mod i2c_mgr;
mod init;
mod io_kbd;
mod nmea;
mod panic;
mod rng_uuid;
mod sound;
mod sys_timer;

pub use amplifier::*;
pub use canbus::*;
pub use eeprom::*;
pub use file_sys::*;
pub use fmc_lcd::*;
pub use frame_buffer::*;
pub use i2c_mgr::*;
pub use init::*;
pub use io_kbd::*;
pub use nmea::*;
pub use panic::*;
pub use rng_uuid::*;
pub use sound::*;
pub use sys_timer::*;

pub type QEvents = MpMcQueue<Event, 8>;
