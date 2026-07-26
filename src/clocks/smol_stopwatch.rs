//! Contains `SmolStopwatch`, a wrapper around `std::time::Instant` that can offset and dilate time.
use std::time::Instant;
use super::units::ElapsedTimeSecs;
use super::super::hooke_spring::HookeSpringClock;

/// A wrapper around `std::time::Instant`, with added support for
/// offsetting elapsed time and dilating the speed of time, through `time_skip` and `time_dilate`.
pub struct SmolStopwatch {
    instant: Instant,
    last_evaluated_real: ElapsedTimeSecs,
    last_effective_time: ElapsedTimeSecs,
    time_scale: f64,
}
impl HookeSpringClock for SmolStopwatch {
    fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs {
        let raw_elapsed_time = self.get_real_elapsed();
        let raw_elapsed_diff = raw_elapsed_time - self.last_evaluated_real;
        
        let dilated_time_diff = raw_elapsed_diff * self.time_scale;
        self.last_effective_time += dilated_time_diff;
        self.last_evaluated_real = raw_elapsed_time;

        self.last_effective_time // copy
    }
    /// Forcibly advances the elapsed time, with respect to time dilation by `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself.
    fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.evaluate_elapsed();
        self.last_effective_time += by * self.time_scale;
    }
}
impl Default for SmolStopwatch {
    /// Alias for `SmolStopwatch::new()`.
    fn default() -> Self {
        Self::new()
    }
}
impl SmolStopwatch {
    /// Creates a `SmolStopwatch` instance.
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
        }
    }

    /// Queries the `time_scale`.
    pub fn get_time_scale(&self) -> &f64 {
        &self.time_scale
    }
    /// Queries how much real-life time elapsed according to `std::Time::Instant`.
    pub fn get_real_elapsed(&self) -> ElapsedTimeSecs {
        self.instant.elapsed().as_secs_f64()
    }

    /// Modifies the `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn time_dilate(&mut self, multiplier: f64) {
        self.evaluate_elapsed();
        self.time_scale = multiplier;
    }
    /// Forcibly advances the elapsed time, ignoring time dilation by `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        self.evaluate_elapsed();
        self.last_effective_time += by;
    }
}