use super::units::ElapsedTimeSecs;
use super::super::hooke_spring::HookeSpringClock;

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
    fn default() -> Self {
        Self { elapsed: 0.0 }
    }
}