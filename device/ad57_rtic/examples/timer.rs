#![no_main]
#![no_std]

mod driver;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use rtic_monotonic::Monotonic;
use stm32f4xx_hal::{
    pac::{interrupt, CorePeripherals, Peripherals, NVIC},
    prelude::*,
};

use driver::*;

static TIMER: Mutex<RefCell<Option<MonoTimer>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Setup clocks
    let mut cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    info!("init");

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    let mut timer = MonoTimer::new(dp.TIM2, &clocks);
    timer.set_time(4_290_000_000);
    timer.listen();

    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    unsafe {
        cp.NVIC.set_priority(interrupt::TIM2, 1);
        NVIC::unmask(interrupt::TIM2);
    }

    loop {
        let now = cortex_m::interrupt::free(|cs| {
            let mut rc = TIMER.borrow(cs).borrow_mut();
            let timer = rc.as_mut().unwrap();
            timer.now()
        });
        info!("timestamp64: {}", now.ticks());
        delay_us(999_978);
    }
}

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        let mut rc = TIMER.borrow(cs).borrow_mut();
        let timer = rc.as_mut().unwrap();
        timer.on_interrupt();
        timer.clear_compare_flag();
    })
}
