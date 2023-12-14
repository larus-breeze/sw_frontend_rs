#[allow(unused)]
use corelib::Event;
#[allow(unused)]
use heapless::mpmc::MpMcQueue;

#[allow(unused)]
mod fmc_lcd;
#[allow(unused)]
mod keyboard;

pub use fmc_lcd::*;
pub use keyboard::*;

pub type QEvents = MpMcQueue<Event, 8>;
