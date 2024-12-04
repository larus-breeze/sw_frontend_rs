#![macro_use]
#![allow(unused_macros)]

#[allow(unused)]
mod clut_colors;
#[allow(unused)]
mod display;
#[allow(unused)]
mod frame_buffer;
#[allow(unused)]
mod ltdc;
mod panic;
#[allow(unused)]
mod st7701s;

#[allow(unused)]
pub use clut_colors::*;
#[allow(unused)]
pub use display::*;
#[allow(unused)]
pub use frame_buffer::*;
#[allow(unused)]
pub use ltdc::*;
#[allow(unused)]
pub use panic::*;
#[allow(unused)]
pub use st7701s::*;

// The macro ensures, that all examples use the same clock settings
macro_rules! set_clocksys {
    ($dp: expr, $cp: expr) => {{
        // Constrain and freeze power, save a little bit power, optimum is at vos3 / 200 MHz
        let pwrcfg = $dp.PWR.constrain().vos1().freeze();

        // Set all needed clock domains
        let ccdr = $dp
            .RCC
            .constrain()
            .use_hse(25.MHz())
            .sys_ck(400.MHz())
            .hclk(200.MHz()) // AHB1,2,3 bus, AXI bus
            .pll1_q_ck(48.MHz()) // spi
            .pll3_p_ck(150.MHz())
            .pll3_q_ck(150.MHz())
            .pll3_r_ck(9.MHz()) // LTDC pixel frequency
            .freeze(pwrcfg, &$dp.SYSCFG);

        $cp.SCB.invalidate_icache();
        $cp.SCB.enable_icache();
    
        ccdr
    }};
}
