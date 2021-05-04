//! Timer to get duration from when the program is launched.
//! Using in competitive programming, espetially Marathons

use std::time::{Duration, Instant};

pub struct Timer {
    initialized_at: Instant,
    // time_limit: u128,
    // margin_time: u128,
}

impl Timer {
    /// Create Timer instance
    ///
    /// This function should use in head of main.
    pub fn new() -> Self {
        Self {
            initialized_at: Instant::now(),
        }
    }

    /// Get duration from timer had initialized
    pub fn duration(&self) -> Duration {
        let cur = Instant::now();
        cur - self.initialized_at
    }

    /// Get duration as `ms`
    pub fn duration_as_millis(&self) -> u128 {
        self.duration().as_millis()
    }

    /// Returns whether current time is over given time
    pub fn is_passed(&self, time_ms: u128) -> bool {
        time_ms <= self.duration_as_millis()
    }
}
