use crate::{
    controller::{Callback, Timer},
    CoreError,
};
use heapless::Deque;

/// Auxiliary structure for the scheduler
pub struct Tim {
    cnt: u16,
    reload: u16,
    cb: Callback,
}

#[allow(unused)]
impl Tim {
    /// Creates a new timer and assigns a fixed callback function to it
    pub const fn new(callback: Callback) -> Tim {
        Tim {
            cnt: 0,
            reload: 0,
            cb: callback,
        }
    }

    fn every(&mut self, d: Duration) {
        self.cnt = d.0;
        self.reload = d.0;
    }

    fn after(&mut self, d: Duration) {
        self.cnt = d.0;
        self.reload = 0;
    }

    fn stop(&mut self) {
        self.cnt = 0;
        self.reload = 0;
    }

    fn tick_100ms(&mut self) -> bool {
        if self.cnt == 0 {
            false
        } else {
            self.cnt -= 1;
            if self.cnt == 0 {
                self.cnt = self.reload;
                true
            } else {
                false
            }
        }
    }

    fn is_after(&self) -> bool {
        self.reload == 0
    }

    fn callback(&self) -> Callback {
        self.cb
    }
}

/// A simple scheduler for tasks in the controller context
///
/// Many activities have to be coordinated in the controller, some of which trigger time-controlled
/// actions. This scheduler abstracts timer functions and ensures that the scheduled activities can
/// be processed in a distributed manner.
pub struct Scheduler<const TCAP: usize> {
    timers: [Tim; TCAP],
    active_cbs: Deque<Callback, TCAP>,
}

impl<const TCAP: usize> Scheduler<TCAP> {
    ///Creates a new instance of the scheduler
    ///
    /// Note: The number of available timers must be known in advance, as well as the callback
    /// functions to be called.
    pub fn new(timers: [Tim; TCAP]) -> Self {
        Scheduler {
            timers,
            active_cbs: Deque::new(),
        }
    }

    /// Called every 100 ms
    ///
    /// Note, the call must be made without gaps - no ticks may be omitted
    pub fn tick_100ms(&mut self) -> Result<(), CoreError> {
        for tim in &mut self.timers {
            if tim.tick_100ms() {
                self.active_cbs
                    .push_back(tim.callback())
                    .map_err(|_| CoreError::SchedulerQueueOverflow)?;
            }
        }
        Ok(())
    }

    /// Pop a callback functions from the queue
    ///
    /// Checks the queue to see whether callback routines are to be called. These can then be
    /// processed. Note: The frequency of the call should be much higher than that of the tick
    /// calls so that all callbacks can be processed in good time.
    pub fn next_callback(&mut self) -> Option<Callback> {
        self.active_cbs.pop_front()
    }

    /// Starting a single shot timer
    ///
    /// This call can also be used to restart a timer that has already been started.
    pub fn after(&mut self, timer: Timer, d: Duration) {
        self.timers[timer as usize].after(d);
    }

    /// Start of a repeating timer with fixed frequency
    pub fn every(&mut self, timer: Timer, d: Duration) {
        self.timers[timer as usize].every(d);
    }

    /// Stops the timer
    ///
    /// With exec_cb true/false you can control whether the assigned callback function is to be
    /// called or not.
    pub fn stop(&mut self, timer: Timer, exec_cb: bool) -> Result<(), CoreError> {
        let tim = &mut self.timers[timer as usize];
        tim.stop();
        if exec_cb {
            self.active_cbs
                .push_back(tim.callback())
                .map_err(|_| CoreError::SchedulerQueueOverflow)?;
        }
        Ok(())
    }

    /// Inserts a callback function into the queue of callback functions
    ///
    /// Note: This function should only be used in conjunction with timer calls, as otherwise queue
    /// overflows may occur. A processed timer callback function always leaves a free space in the
    /// queue. A subsequent chain() call is therefore not critical.
    pub fn chain(&mut self, callback: Callback) -> Result<(), CoreError> {
        self.active_cbs
            .push_back(callback)
            .map_err(|_| CoreError::SchedulerQueueOverflow)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Duration(pub u16);

pub trait IntToDuration {
    fn millis(self) -> Duration;
    fn secs(self) -> Duration;
}

impl IntToDuration for u16 {
    fn millis(self) -> Duration {
        Duration(self / 100)
    }
    fn secs(self) -> Duration {
        assert!(self < u16::MAX / 10);
        Duration(self * 10)
    }
}
