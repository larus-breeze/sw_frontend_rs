#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;

use rtic::app;
use fugit::Instant;
use stm32h7xx_hal::{
    pac, prelude::*,
};

use driver::*;

#[app(device = pac, peripherals = true, dispatchers = [SPI1, SPI2])]
mod app {
    use super::*;

    #[monotonic(binds = TIM2, default = true)]
    type MyMono = MonoTimer;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        tp_foo: Instant<u64, 1, 1_000_000>,
        tp_bar: Instant<u64, 1, 1_000_000>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp = cx.device;
        let ccdr = set_clocksys!(dp);
        let mono = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    
        info!("init");

        // Schedule `foo` to run 1 second in the future
        let tp_foo = Instant::<u64, 1, 1_000_000>::from_ticks(0x1_0000_0000);
        bar::spawn_at(tp_foo).unwrap();
        foo::spawn_at(tp_foo).unwrap();

        (
            Shared {},
            Local {tp_foo, tp_bar: tp_foo},
            init::Monotonics(mono), // Give the monotonic to RTIC
        )
    }

    #[task(local=[tp_foo], priority=5)]
    fn foo(cx: foo::Context) {
        let ts = timestamp();
        info!("T = T + 1_000_000 µs {}", ts);

        let ticks = cx.local.tp_foo.ticks();
        *cx.local.tp_foo = Instant::<u64, 1, 1_000_000>::from_ticks(ticks + 1_000_000);

        foo::spawn_at(*cx.local.tp_foo).unwrap();
    }

    #[task(local = [tp_bar], priority=4)]
    fn bar(cx: bar::Context) {
        let ts = timestamp();
        info!("T = T + 0_999_999 µs {}", ts);

        let ticks = cx.local.tp_bar.ticks();
        *cx.local.tp_bar = Instant::<u64, 1, 1_000_000>::from_ticks(ticks + 0_999_999);

        bar::spawn_at(*cx.local.tp_bar).unwrap();
    }

}