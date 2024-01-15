#![no_main]
#![no_std]

mod driver;

use corelib::{EepromTopic, PersistenceId, PersistenceItem};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use driver::*;
use stm32h7xx_hal::{
    pac::{CorePeripherals, Peripherals as DevicePeripherals},
    prelude::*,
    rcc::PllConfigStrategy,
};

#[entry]
fn main() -> ! {
    // Setup clocks
    let mut cp = CorePeripherals::take().unwrap();
    let dp = DevicePeripherals::take().unwrap();

    info!("init");

    // Constrain and freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Initialize clock system
    let rcc = dp.RCC.constrain();

    let ccdr = rcc
        .use_hse(25.MHz())
        .sys_ck(200.MHz())
        .hclk(200.MHz())
        .pclk1(100.MHz())
        .pll1_strategy(PllConfigStrategy::Iterative)
        .pll1_q_ck(50.MHz())
        .pll2_p_ck(100.MHz())
        .pll2_r_ck(100.MHz())
        .freeze(pwrcfg, &dp.SYSCFG);

    // Initialize system...
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Setup ----------> Eeprom driver for idle loop
    let scl = gpiob.pb6.into_alternate_open_drain();
    let sda = gpiob.pb7.into_alternate_open_drain();
    let i2c = dp
        .I2C1
        .i2c((scl, sda), 400.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);
    let r_eeprom = Storage::new(i2c);
    let mut eeprom = r_eeprom.unwrap();

    for item in eeprom.iter_over(EepromTopic::ConfigValues) {
        info!(
            "After REset - item {}, value {:?}",
            item.id as u8, item.data
        );
    }

    let mc_cready: f32 = 0.5;
    let p_item = PersistenceItem::from_f32(PersistenceId::McCready, mc_cready);
    eeprom.write_item(p_item).unwrap();

    let mut volume: i8 = 2;
    let p_item = PersistenceItem::from_i8(PersistenceId::Volume, volume);
    eeprom.write_item(p_item).unwrap();

    loop {
        for item in eeprom.iter_over(EepromTopic::ConfigValues) {
            info!("item {}, value {:?}", item.id as u8, item.data);
        }
        delay_ms(1_000);

        volume += 1;
        let p_item = PersistenceItem::from_i8(PersistenceId::Volume, volume);
        eeprom.write_item(p_item).unwrap();
    }
}
