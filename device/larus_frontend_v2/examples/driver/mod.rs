#![macro_use]
#![allow(unused_macros)]

#[allow(unused)]
mod clut_colors;
#[allow(unused)]
mod ltdc;
mod panic;
#[allow(unused)]
mod st7701s;

#[allow(unused)]
pub use clut_colors::*;
#[allow(unused)]
pub use ltdc::*;
#[allow(unused)]
pub use panic::*;
#[allow(unused)]
pub use st7701s::*;

// The macro ensures, that all examples use the same clock settings
macro_rules! set_clocksys {
    ($dp: expr) => {{
        // Constrain and freeze power, save a little bit power, optimum is at vos3 / 200 MHz
        let pwrcfg = $dp.PWR.constrain().vos3().freeze();

        // Set all needed clock domains
        let ccdr = $dp
            .RCC
            .constrain()
            .use_hse(25.MHz())
            .sys_ck(200.MHz())
            .hclk(100.MHz())
            .pll1_q_ck(50.MHz()) // CAN
            .pll2_p_ck(100.MHz()) // ?
            .pll2_r_ck(50.MHz()) // LCD
            .freeze(pwrcfg, &$dp.SYSCFG);
        ccdr
    }};
}
