#![no_main]
#![no_std]

mod driver;

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;
use stm32h7xx_hal::{
    pac::{CorePeripherals, Peripherals as DevicePeripherals},
    prelude::*,
};
use driver::*;

#[entry]
fn main() -> ! {
    // Setup clocks
    let mut cp = CorePeripherals::take().unwrap();
    let dp = DevicePeripherals::take().unwrap();

    info!("init");

    //    let pwr = dp.PWR.constrain().vos3(); // vos1: 400 MHz, vos2: 300 MHz, vos3: 200 MHz max
    //    let pwrcfg: stm32h7xx_hal::pwr::PowerConfiguration = pwr.freeze();

    //let rcc = dp.RCC.constrain();
    let ccdr = set_clocksys!(dp);

    // Initialize system...
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();

    trace!("hse_ck        {}", ccdr.clocks.hse_ck().unwrap().raw());
    trace!("sys_ck        {}", ccdr.clocks.sys_ck().raw());
    trace!("hclk          {}", ccdr.clocks.hclk().raw());
    trace!("q_ck (CAN)    {}", ccdr.clocks.pll1_q_ck().unwrap().raw());
    trace!("p_ck          {}", ccdr.clocks.pll2_p_ck().unwrap().raw());
    trace!("r_ck (LCD)    {}", ccdr.clocks.pll2_r_ck().unwrap().raw());

    let _mono = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    loop {
        trace!("Delay 3s");
        delay_ms(3_000_u32);
    }
}
