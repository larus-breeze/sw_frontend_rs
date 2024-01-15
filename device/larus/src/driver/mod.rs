#![macro_use]
#![allow(unused_macros)]

#[allow(unused)]
use corelib::Event;
#[allow(unused)]
use heapless::mpmc::MpMcQueue;

mod canbus;
mod eeprom;
mod fmc_lcd;
mod frame_buffer;
mod init;
mod keyboard;
mod panic;
mod rng_uuid;
mod sys_timer;

pub use canbus::*;
pub use eeprom::*;
pub use fmc_lcd::*;
pub use frame_buffer::*;
pub use init::*;
pub use keyboard::*;
pub use panic::*;
pub use rng_uuid::*;
pub use sys_timer::*;

pub type QEvents = MpMcQueue<Event, 8>;
