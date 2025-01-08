use crate::{driver::timestamp_us, DevDuration, DevInstant};
use defmt::*;

use crate::app;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Task {
    CanRx,
    CanTx,
    CanTimer,
    Keyboard,
    //
    Controller,
    View,
    LcdCopy,
    NmeaRx,
    //
    NmeaTx,
    None,
}

// Pattern defines, which taks must be active to feed the watchdog
const ALL_ALIVE_PATTERN: u32 = 0b_0001_1111_1110;

impl Task {
    pub fn from_usize(u: usize) -> Self {
        if u < Task::None as usize {
            // We checked the range, so transmute is ok
            unsafe { core::mem::transmute::<u8, Task>(u as u8) }
        } else {
            core::panic!()
        }
    }
}

// define the task names
const TASK_NAMES: [&str; TASK_CNT] = [
    "CanRx",
    "CanTx",
    "CanTimer",
    "Keyboard",
    "Controller",
    "View",
    "LcdCopy",
    "NmeaRx",
    "NmeaTx",
];
const TASK_CNT: usize = Task::None as usize;

// storage for the task times
#[derive(Clone, Copy)]
struct StatTimes {
    min_time: u32,
    max_time: u32,
    sum_time: u32,
    act_task_time: u32,
    last_start: u32,
    count: u32,
}

pub struct Statistics {
    stats: [StatTimes; TASK_CNT],
    next_show: DevInstant,
    stack: [u8; TASK_CNT],
    stack_cnt: usize,
    alive: u32,
}

const INTERVAL: usize = 3;

impl Statistics {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // initialize storage
        let stats = StatTimes {
            min_time: u32::MAX,
            max_time: 0,
            sum_time: 0,
            act_task_time: 0,
            last_start: 0,
            count: 0,
        };
        Statistics {
            stats: [stats; TASK_CNT],
            next_show: app::monotonics::now() + DevDuration::secs(1),
            stack: [0u8; TASK_CNT],
            stack_cnt: 0,
            alive: 0,
        }
    }

    /// The [start_task] function starts the time counting for a task. If a task was already
    /// started before, the counting is interrupted for this and the intermediate result is
    /// stored.
    pub fn start_task(&mut self, task: Task) {
        self.alive |= 1 << (task as u8);
        let now = timestamp_us();

        if self.stack_cnt > 0 {
            let low_prio_task = self.stack[self.stack_cnt - 1];
            let low_prio_stats = &mut self.stats[low_prio_task as usize];
            low_prio_stats.act_task_time += now.saturating_sub(low_prio_stats.last_start);
        }

        let task_idx = task as usize;
        let stats = &mut self.stats[task_idx];
        stats.count += 1;
        stats.last_start = now;
        stats.act_task_time = 0;
        self.stack[self.stack_cnt] = task_idx as u8;
        self.stack_cnt += 1;
    }

    /// Ends the time counting for a task. If a task with lower priority was started before, the
    /// time counting will be continued for this task. If no task is active any more, it is still
    /// checked whether the minimum time for an output has been reached and this is executed if
    /// necessary.
    pub fn end_task(&mut self, task: Task) {
        let now = timestamp_us();

        let task_idx = task as usize;
        let stats = &mut self.stats[task_idx];
        let time_used = now
            .saturating_sub(stats.last_start)
            .saturating_add(stats.act_task_time);
        if time_used < stats.min_time {
            stats.min_time = time_used;
        }
        if time_used > stats.max_time {
            stats.max_time = time_used;
        }
        stats.sum_time = stats.sum_time.saturating_add(time_used);
        self.stack_cnt -= 1;

        if self.stack_cnt > 0 {
            let low_prio_task_idx = self.stack[self.stack_cnt - 1] as usize;
            let low_prio_stats = &mut self.stats[low_prio_task_idx];
            low_prio_stats.last_start = now;
        } else if app::monotonics::now() > self.next_show {
            self.next_show += DevDuration::secs(INTERVAL as u64);
            let mut workload: u32 = 0;
            info!("TaskCalls[/sec] Sum[ms/sec] Max[µs/loop]");
            for (idx, task_name) in TASK_NAMES.iter().enumerate().take(TASK_CNT) {
                let stats = &mut self.stats[idx];
                workload = workload.saturating_add(stats.sum_time);
                let sum = if stats.count > 0 {
                    stats.sum_time / INTERVAL as u32 / 1000
                } else {
                    0
                };
                info!(
                    "{} {} {} {}",
                    task_name,
                    stats.count / INTERVAL as u32,
                    sum,
                    stats.max_time
                );
                stats.min_time = u32::MAX;
                stats.max_time = 0;
                stats.sum_time = 0;
                stats.count = 0;
                stats.act_task_time = 0;
            }
            let workload = workload / (1_000_000 * INTERVAL as u32 / 100);
            info!("Workload {}%", workload);
        }
    }

    pub fn all_alive(&mut self) -> bool {
        let r = (self.alive & ALL_ALIVE_PATTERN) == ALL_ALIVE_PATTERN;
        if r {
            self.alive = 0;
        }
        r
    }
}
