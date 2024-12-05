#![macro_use]
#![allow(unused_macros)]

/// Starts the time counting for a task, see [Statistics].
macro_rules! task_start {
    ($cx:expr, $x:expr) => {
        $cx.shared.statistics.lock(|stats| stats.start_task($x));
    };
}

/// Stops the time counting for a task, see [Statistics].
macro_rules! task_end {
    ($cx:expr, $x:expr) => {
        $cx.shared.statistics.lock(|stats| stats.end_task($x));
    };
}
