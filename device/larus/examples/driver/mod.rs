#[allow(unused)]
use corelib::Event;
#[allow(unused)]
use heapless::mpmc::MpMcQueue;


#[allow(unused)]
mod canbus;
#[allow(unused)]
mod fmc_lcd;
#[allow(unused)]
mod keyboard;

pub use canbus::*;
pub use fmc_lcd::*;
pub use keyboard::*;

pub type QEvents = MpMcQueue<Event, 8>;
