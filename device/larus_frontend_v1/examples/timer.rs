#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use pac::interrupt;
use rtic_monotonic::Monotonic;
use stm32h7xx_hal::{pac, prelude::*};

use driver::*;

static TIMER: Mutex<RefCell<Option<MonoTimer>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    info!("init");

    let ccdr = set_clocksys!(dp);
    let mut timer = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    timer.set_time(4_290_000_000);
    timer.listen();

    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    unsafe {
        cp.NVIC.set_priority(interrupt::TIM2, 1);
        pac::NVIC::unmask(interrupt::TIM2);
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
