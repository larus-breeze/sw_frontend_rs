use defmt::*;

use rtic_monotonic::Monotonic;
use stm32f4xx_hal::{
    pac::{RCC, TIM3},
    rcc::Clocks,
};
use systick_monotonic::fugit::{Duration, Instant};

pub const TICKS_PER_SECOND: u32 = 1_000;

pub type DevDuration = Duration<u32, 1, TICKS_PER_SECOND>;
pub type DevInstant = Instant<u32, 1, TICKS_PER_SECOND>;
pub type DevTimer = MonoTimer<stm32f4xx_hal::pac::TIM3, TICKS_PER_SECOND>;

#[inline]
pub fn timestamp_us() -> u32 {
    unsafe { core::ptr::read_volatile(0x4000_0024 as *const u32) }
}

defmt::timestamp!("{=u32:us}", {
    // NOTE(interrupt-safe) single instruction volatile read operation
    timestamp_us()
});

// HW timer runs with 1 tick / us, mono timer with 1 tick / ms
const OVF_WIDTH: u32  = 0x1_0000;

pub struct MonoTimer<T, const FREQ: u32> {
    timer: T,
    ovf: u32,
}

impl<const FREQ: u32> MonoTimer<TIM3, FREQ> {
    pub fn new(timer: TIM3, clocks: &Clocks) -> Self {
        //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
        let rcc = unsafe { &(*RCC::ptr()) };
        rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());
        rcc.apb1rstr.modify(|_, w| w.tim3rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.tim3rst().clear_bit());
        let pclk_mul = if clocks.ppre1() == 1 { 1 } else { 2 };
        let prescaler = clocks.pclk1().0 * pclk_mul / FREQ - 1;

        timer.psc.write(|w| w.psc().bits(prescaler as u16));
        timer.arr.write(|w| unsafe { w.bits(u32::MAX) });

        timer.cr1.modify(|_, w| w.urs().set_bit());
        timer.egr.write(|w| w.ug().set_bit());
        timer.cr1.modify(|_, w| w.urs().clear_bit());

        //timer.sr.modify(|_, w| w.uif().clear_bit());
        timer.dier.modify(|_, w| w.cc1ie().set_bit().uie().set_bit());
        timer.cr1.modify(|_, w| w.cen().set_bit()); //.udis().clear_bit());
        Self {timer, ovf: 0}
    }
}

impl<const FREQ: u32> Monotonic for MonoTimer<TIM3, FREQ> {
    type Instant = fugit::TimerInstantU32<FREQ>;
    type Duration = fugit::TimerDurationU32<FREQ>;

    unsafe fn reset(&mut self) {
        self.timer.dier.modify(|_, w| w.cc1ie().set_bit().uie().set_bit());
    }

    #[inline(always)]
    fn now(&mut self) -> Self::Instant {
        let mut cnt = self.timer.cnt.read().cnt().bits() as u32;
        if self.timer.sr.read().uif().bits() {
            cnt += OVF_WIDTH;
        }
        Self::Instant::from_ticks(cnt + self.ovf)
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        let now = self.now();
        // Since the timer may or may not overflow based on the requested compare val, we check
        // how many ticks are left.
        let val = match instant.checked_duration_since(now) {
            None => 1, // In the past, RTIC will handle this
            Some(x) if x.ticks() < OVF_WIDTH => {
                trace!("No overflow");
                instant.duration_since_epoch().ticks()
            } // Will not overflow
            Some(_x) => {
                let count = self.timer.cnt.read().cnt().bits() as u32;
                trace!("overflow");
                count.wrapping_add(OVF_WIDTH  - 4) // Will overflow
            }
        };
        trace!("now {}, val {}", now.ticks(), val);
        self.timer
            .ccr1
            .write(|w| wlcd_view_wakeup_at.ccr().bits(val as u16));
    }

    fn clear_compare_flag(&mut self) {
        self.timer.sr.modify(|_, w| w.cc1if().clear_bit());
    }

    fn on_interrupt(&mut self) {
        trace!("Interrupt TIM3");
        // If there was an overflow, clear uif bit and increment the overflow counter.
        if self.timer.sr.read().uif().bits() {
            self.timer.sr.modify(|_, w| w.uif().clear_bit());
            self.ovf += OVF_WIDTH;
        }
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }
}

