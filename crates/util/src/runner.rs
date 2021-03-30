//! Timer to get duration from when the program is launched.
//! Using in competitive programming, espetially Marathons
//!
//! mod name "runner" comes from marathon

use std::time::{Duration, Instant};

type OnExit = bool;

pub struct Timer {
    initialized_at: Instant,
    time_limit: u128,
    check_interval: usize,
    margin_time: u128,
}

impl Timer {
    /// Create Timer instance
    ///
    /// This function should use in head of main.
    pub fn new(time_limit: u128, check_interval: usize, margin_time: u128) -> Self {
        assert!(margin_time < time_limit);
        Self {
            initialized_at: Instant::now(),
            time_limit,
            check_interval,
            margin_time,
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

    /// Get duration ratio for the time limit
    pub fn duration_as_ratio(&self) -> f64 {
        self.duration_as_millis() as f64 / self.time_limit as f64
    }

    pub fn is_passed(&self, time_ms: u128) -> bool {
        time_ms <= self.duration_as_millis()
    }

    /// Returns whether current time is in margin time
    pub fn should_exit(&self) -> bool {
        self.is_passed(self.time_limit - self.margin_time)
    }

    /// Call the task repeatedly
    ///
    /// task is a clojure takes one argument, `OnExit`.
    /// This type is alias for bool.
    /// While loop is running, calls f(false),
    /// and calls f(true) at last just before the exit.
    ///
    /// ```ignore
    /// use util::runner::Timer;
    ///
    /// let timer = Timer::new(100, 1, 20);
    ///
    /// let mut i = 0usize;
    /// let f = |x| {
    ///     if x {
    ///         println!("{}", i);
    ///     } else {
    ///         i += 1
    ///     }
    /// };
    ///  // call f repeatedly while it pass 80ms, and exit the program
    /// timer.run_loop(f)
    /// ```
    pub fn run_loop<F: FnMut(OnExit)>(self, mut task: F) -> ! {
        let mut loop_count = 0usize;
        loop {
            if loop_count % self.check_interval == 0 {
                if self.should_exit() {
                    task(true);
                    std::process::exit(0);
                }
            }
            task(false);
            loop_count += 1;
        }
    }

    /// Call the task repeatedly
    ///
    ///
    /// ```
    /// use util::runner::Timer;
    ///
    /// let timer = Timer::new(100, 1, 20);
    ///
    /// let mut i = 0usize;
    /// let f = || i += 1;
    ///
    /// let looped = timer.run_while(f, 100).unwrap();
    /// assert_eq!(looped, i);
    /// ```
    pub fn run_while<F: FnMut()>(self, mut task: F, exit_time: u128) -> Result<usize, ()> {
        let mut loop_count = 0usize;
        loop {
            if loop_count % self.check_interval == 0 {
                let rest = self.time_limit - self.duration_as_millis();
                if rest <= exit_time {
                    break;
                }
            }
            task();
            loop_count += 1;
        }

        Ok(loop_count)
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    #[ignore = "
        Timer::run_loop uses `std::process::exit` inside.
        This function exit the process test is working.
    "]
    fn task_loop() {
        let timer = Timer::new(100, 1, 20);

        let mut i = 0usize;
        let f = |x| {
            if x {
                println!("{}", i);
            } else {
                i += 1
            }
        };

        // call f repeatedly while it pass 80ms, and exit the program
        timer.run_loop(f)
    }

    #[test]
    fn task_loop_while() -> Result<(), ()> {
        let timer = Timer::new(100, 1, 20);

        let mut i = 0usize;
        let f = || i += 1;

        let looped = timer.run_while(f, 100)?;
        assert_eq!(looped, i);
        Ok(())
    }
}
