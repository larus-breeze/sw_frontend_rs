#![no_main]
#![no_std]
mod driver;

use defmt::*;
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{adc, pac, prelude::*, rcc::rec::AdcClkSel};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let mut ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::Per);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let mut adc1 = adc::Adc::adc1(
        dp.ADC1,
        4.MHz(),
        &mut delay,
        ccdr.peripheral.ADC12,
        &ccdr.clocks,
    )
    .enable();
    adc1.set_resolution(adc::Resolution::SixteenBit);

    // We can't use ADC2 here because ccdr.peripheral.ADC12 has been
    // consumed. See examples/adc12.rs

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    let mut supply = gpioa.pa6.into_analog();
    let mut illumination = gpioc.pc4.into_analog();
    let mut temp= gpiob.pb1.into_analog();

    loop {
            delay.delay_ms(20_u16);

            let voltage_adc: u32 = adc1.read(&mut supply).unwrap();
            info!(
               "Supply Voltage {}",
               voltage_adc as f32 * 0.000503540039
            );

            let illumination_adc: u32 = adc1.read(&mut illumination).unwrap();
            info!(
                "Illumination Voltage {}",
                illumination_adc as f32 * 0.000045776
             );

            let temperature_adc: u32 = adc1.read(&mut temp).unwrap();
            info!(
                "PCB Temperature {}",
                100.0 * ((temperature_adc as f32) * 0.000045776367 - 0.5)
             );

        }
    
}
