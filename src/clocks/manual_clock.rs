//! Contains `ManualClock`. It is recommended to implement your own clock that implements `HookeSpringClock` for production use.
use super::units::ElapsedTimeSecs;
use super::super::hooke_spring::HookeSpringClock;

/// A clock that you are meant to call `time_skip` manually according to your use case
/// to update its elapsed time.
pub struct ManualClock {
    elapsed: ElapsedTimeSecs,
}
impl HookeSpringClock for ManualClock {
    fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs {
        self.elapsed
    }
    fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.elapsed += by
    }
}
impl Default for ManualClock {
    /// Alias for `ManualClock::new()`.
    fn default() -> Self {
        Self::new()
    }
}
impl ManualClock {
    /// Creates a `ManualClock` instance.
    fn new() -> Self {
        Self { elapsed: 0.0f64 }
    }
}