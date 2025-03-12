#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;

use corelib::Lock;
use cortex_m_rt::entry;

use pac::interrupt;
use rtic_monotonic::Monotonic;
use stm32h7xx_hal::{pac, prelude::*};

use driver::*;

static TIMER: Lock<MonoTimer> = Lock::new();

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    info!("init");

    let ccdr = set_clocksys!(dp, cp);

    let mut timer = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    timer.set_time(4_290_000_000);
    timer.listen();

    TIMER.set(timer);

    unsafe {
        cp.NVIC.set_priority(interrupt::TIM2, 1);
        pac::NVIC::unmask(interrupt::TIM2);
    }

    loop {
        let now = TIMER.lock_during_use(|timer| timer.unwrap().now());

        info!("timestamp64: {}", now.ticks());
        delay_us(999_997);
    }
}

#[interrupt]
fn TIM2() {
    TIMER.lock_during_use(|opt_tim| {
        if let Some(timer) = opt_tim {
            timer.on_interrupt();
            timer.clear_compare_flag();
        }
    });
}
