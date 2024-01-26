#![macro_use]
#![allow(unused_macros)]

#[allow(unused)]
use corelib::Event;
#[allow(unused)]
use heapless::mpmc::MpMcQueue;

#[allow(unused)]
mod canbus;
#[allow(unused)]
mod eeprom;
#[allow(unused)]
mod fmc_lcd;
#[allow(unused)]
mod frame_buffer;
#[allow(unused)]
mod i2c_mgr;
#[allow(unused)]
mod keyboard;
#[allow(unused)]
mod panic;
#[allow(unused)]
mod sys_timer;

pub use canbus::*;
pub use eeprom::*;
pub use fmc_lcd::*;
pub use frame_buffer::*;
pub use i2c_mgr::*;
pub use keyboard::*;
pub use panic::*;
pub use sys_timer::*;

pub type QEvents = MpMcQueue<Event, 8>;

// The macro ensures, that all examples use the same clock settings
macro_rules! set_clocksys {
    ($dp: expr) => {{
        // Constrain and freeze power, save a little bit power, optimum is at vos3 / 200 MHz
        let pwrcfg = $dp.PWR.constrain().vos3().freeze();

        // Set all needed clock domains
        let ccdr = $dp
            .RCC
            .constrain()
            .use_hse(16.MHz())
            .sys_ck(200.MHz())
            .hclk(100.MHz())
            .pll1_q_ck(50.MHz()) // CAN
            .pll2_p_ck(100.MHz()) // ?
            .pll2_r_ck(50.MHz()) // LCD
            .freeze(pwrcfg, &$dp.SYSCFG);
        ccdr
    }};
}
