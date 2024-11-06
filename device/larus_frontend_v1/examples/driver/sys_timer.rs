/// This module provides time functions for various applications
///
/// 1. fn timestamp() for logs and other functions
/// 2. timestam!() for defmt
/// 3. delay functions as busy-wait in different variants
///     - fn delay_ms() works in milliseconds
///     - fn delay_us() waits microseconds
///     - delay instance in any number
/// 4. time base for RTIC with microsecond resolution
///
/// Note: uses TIM2 as time base
use defmt::*;
use defmt_rtt as _;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicU32, Ordering},
};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use fugit::{Duration, Instant};
use pac::interrupt;
use rtic_monotonic::Monotonic;
use stm32h7xx_hal::{
    pac,
    prelude::*,
    rcc::{rec::Tim2, CoreClocks, ResetEnable},
    time::Hertz,
    timer::GetClk,
};

// Address of timer 2 counter register
const TIM2_CNT: *const u32 = 0x4000_0024 as *const u32;

/// Get timestamp with µs resolution from tim2
pub fn timestamp() -> u32 {
    // Safety: There is nothing wrong with reading a synchronized and aligned u32 number from the timer
    unsafe { core::ptr::read_volatile(TIM2_CNT) }
}

defmt::timestamp!("{=u32:us}", timestamp());

/// Busy wait based on timer, so (interrupt-) delays in between will ignored
pub fn delay_ms(ms: u32) {
    let target = timestamp().wrapping_add(ms * 1_000);
    loop {
        let diff = target.wrapping_sub(timestamp()) as i32;
        if diff <= 0 {
            break;
        }
    }
}

/// Busy wait based on timer, so (interrupt-) delays in between will ignored
pub fn delay_us(us: u32) {
    let target = timestamp().wrapping_add(us);
    loop {
        let diff = target.wrapping_sub(timestamp()) as i32;
        if diff <= 0 {
            break;
        }
    }
}

/// Busy wait delays as definied in embedded hal traits
struct Delay {}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        delay_ms(ms)
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        delay_us(us)
    }
}

const OVF_VALUE: u64 = 0x1_0000_0000; // Overflow for u32
const TICKS_PER_SECOND: u32 = 1_000_000; // TIM2 runs with 1 MHz

pub struct MonoTimer {
    tim: pac::TIM2,
    overflow: u64,
}

impl MonoTimer {
    pub fn new(tim: pac::TIM2, prec: Tim2, clocks: &CoreClocks) -> Self {
        let _ = prec.enable().reset(); // drop, can be recreated by free method

        tim.cr1.modify(|_, w| w.cen().clear_bit()); // pause()
                                                    // UEV event occours on next overflow
        tim.cr1.modify(|_, w| w.urs().counter_only()); // urs_counter_only()
        tim.sr.modify(|_, w| {
            // clear_irq()
            // Clears timeout event
            w.uif().clear_bit()
        });
        let _ = tim.sr.read();
        let _ = tim.sr.read(); // Delay 2 peripheral clocks

        let clk = pac::TIM2::get_clk(clocks)
            .expect("TIM2 no input clock")
            .raw();
        let div = clk / TICKS_PER_SECOND; // set_tick_freq(frequency)

        let psc = (div - 1) as u16;
        tim.psc.write(|w| w.psc().bits(psc));

        let counter_max = u32::MAX;
        tim.arr.write(|w| w.bits(counter_max));

        // Generate an update event to force an update of the ARR
        // register. This ensures the first timer cycle is of the
        // specified duration.
        tim.egr.write(|w| w.ug().set_bit()); // apply_freq()

        // Test overflow
        tim.cnt.write(|w| w.bits(0));

        // Start counter
        tim.cr1.modify(|_, w| w.cen().set_bit()); // resume()

        MonoTimer { tim, overflow: 0 }
    }

    /// Enable overflow and ccr1 interrupt
    pub fn listen(&mut self) {
        self.tim.dier.write(|w| w.uie().set_bit()); // listen(timer::Event::TimeOut)
        self.tim.dier.write(|w| w.cc1ie().set_bit()); // listen(timer::Event::CaptureCompare1)
    }

    /// Set timer counter for test purposes
    pub fn set_time(&mut self, ticks: u64) {
        self.overflow = ticks & 0xffff_ffff_0000_0000;
        self.tim.cnt.write(|w| w.bits(ticks as u32));
    }
}

/// Use Compare channel 1 for Monotonic
impl rtic_monotonic::Monotonic for MonoTimer {
    // Since we are counting overflows we can't let RTIC disable the interrupt.
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    type Instant = fugit::TimerInstantU64<1_000_000>;
    type Duration = fugit::TimerDurationU64<1_000_000>;

    fn now(&mut self) -> Self::Instant {
        let cnt = self.tim.cnt.read().bits();

        // If the overflow bit is set, we add this to the timer value. It means the `on_interrupt`
        // has not yet happened, and we need to compensate here.
        let ovf = if self.tim.sr.read().uif().bit_is_set() {
            OVF_VALUE
        } else {
            0
        };
        Self::Instant::from_ticks(u64::from(cnt) + ovf + self.overflow)
    }

    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }

    unsafe fn reset(&mut self) {
        self.listen()
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        let now = self.now();

        // Since the timer may or may not overflow based on the requested compare val, we check
        // how many ticks are left.
        let val: u32 = match instant.checked_duration_since(now) {
            None => 1, // In the past, RTIC will handle this
            Some(x) if x.ticks() < OVF_VALUE => instant.duration_since_epoch().ticks() as u32, // Will not overflow
            Some(_x) => self.tim.cnt.read().bits().wrapping_add(0xffff_fffe), // Will overflow
        };
        self.tim.ccr[0].write(|w| unsafe { w.bits(val) });
    }

    fn clear_compare_flag(&mut self) {
        self.tim.sr.modify(|_, w| w.cc1if().clear_bit());
        let _ = self.tim.sr.read();
        let _ = self.tim.sr.read(); // Delay 2 peripheral clocks
    }

    fn on_interrupt(&mut self) {
        // If there was an overflow, increment the overflow counter.
        if self.tim.sr.read().uif().bit_is_set() {
            self.tim.sr.modify(|_, w| w.uif().clear_bit());
            let _ = self.tim.sr.read();
            let _ = self.tim.sr.read(); // Delay 2 peripheral clocks
            self.overflow += OVF_VALUE;
        }
    }
}
