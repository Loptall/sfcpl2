pub mod timer;
pub use timer::Timer;

type OnExit = bool;

/// Repeat calling function with timer
pub struct TaskRunner {
    timer: Timer,
    time_limit: u128,
    margin_time: u128,
    check_interval: usize,
}

impl TaskRunner {
    /// Initialize with new timer instance
    pub fn new(time_limit: u128, margin_time: u128, check_interval: usize) -> Self {
        Self {
            timer: Timer::new(),
            time_limit,
            margin_time,
            check_interval,
        }
    }

    /// Initialize using other timer
    pub fn with_timer(
        timer: Timer,
        time_limit: u128,
        margin_time: u128,
        check_interval: usize,
    ) -> Self {
        Self {
            timer,
            time_limit,
            margin_time,
            check_interval,
        }
    }

    /// Get duration ratio for the time limit
    pub fn duration_as_ratio(&self) -> f64 {
        self.timer.duration_as_millis() as f64 / self.time_limit as f64
    }

    /// Returns whether current time is in margin time
    pub fn should_exit(&self) -> bool {
        self.timer.is_passed(self.time_limit - self.margin_time)
    }

    /// Call the task repeatedly
    ///
    /// task is a clojure takes one argument, `OnExit`.
    /// This type is alias for bool.
    /// While loop is running, calls f(false),
    /// and calls f(true) at last just before the exit.
    ///
    /// ```ignore
    /// use runner::TaskRunner;
    ///
    /// let runner = TaskRunner::new(100, 20, 1);
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
    /// runner.run_loop(f)
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

    /// Call the task repeatedly in given time
    ///
    /// While timer value is less than exit_time.
    ///
    /// margin_time is not apllied
    ///
    /// ```
    /// use runner::TaskRunner;
    ///
    /// let runner = TaskRunner::new(100, 20, 1);
    ///
    /// let mut i = 0usize;
    /// let f = || i += 1;
    ///
    /// let looped = runner.run_while(f, 100);
    /// assert_eq!(looped, i);
    /// ```
    pub fn run_while<F: FnMut()>(self, mut task: F, exit_time: u128) -> usize {
        let mut loop_count = 0usize;
        loop {
            if loop_count % self.check_interval == 0 {
                if self.timer.is_passed(exit_time) {
                    break;
                }
            }
            task();
            loop_count += 1;
        }

        loop_count
    }

    /// Call the task repeatedly for given times
    ///
    /// While timer value is less than exit_time.
    ///
    /// margin_time is not apllied
    ///
    /// ```
    /// use runner::TaskRunner;
    ///
    /// let runner = TaskRunner::new(100, 20, 1);
    ///
    /// let mut i = 0usize;
    /// let f = || i += 1;
    ///
    /// let looped = runner.run_for(f, 100);
    /// assert_eq!(looped, 100);
    /// assert_eq!(looped, i);
    /// ```
    pub fn run_for<F: FnMut()>(self, mut task: F, n: u128) -> usize {
        let mut loop_count = 0usize;
        for _ in 0..n {
            task();
            loop_count += 1;
        }

        loop_count
    }
}

#[cfg(test)]
mod tests {
    use super::TaskRunner;

    #[test]
    #[ignore = "
        Timer::run_loop uses `std::process::exit` inside.
        This function exit the process test is working.
    "]
    fn task_loop() {
        let runner = TaskRunner::new(100, 20, 1);

        let mut i = 0usize;
        let f = |x| {
            if x {
                println!("{}", i);
            } else {
                i += 1
            }
        };

        // call f repeatedly while it pass 80ms, and exit the program
        runner.run_loop(f)
    }

    #[test]
    fn task_loop_while() -> Result<(), ()> {
        let runner = TaskRunner::new(100, 20, 1);

        let mut i = 0usize;
        let f = || i += 1;

        let looped = runner.run_while(f, 100);
        assert_eq!(looped, i);
        Ok(())
    }
}
