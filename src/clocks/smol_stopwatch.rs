use std::time::Instant;
use super::units::ElapsedTimeSecs;
use super::super::hooke_spring::HookeSpringClock;

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
    fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.evaluate_elapsed();
        self.last_effective_time += by * self.time_scale;
    }
}
impl Default for SmolStopwatch {
    fn default() -> Self {
        Self::new()
    }
}
impl SmolStopwatch {
    pub fn new() -> SmolStopwatch {
        SmolStopwatch {
            instant: Instant::now(),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
        }
    }

    pub fn get_time_scale(&self) -> &f64 {
        &self.time_scale
    }
    pub fn get_real_elapsed(&self) -> ElapsedTimeSecs {
        self.instant.elapsed().as_secs_f64()
    }

    pub fn time_dilate(&mut self, multiplier: f64) {
        self.evaluate_elapsed();
        self.time_scale = multiplier;
    }
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        self.evaluate_elapsed();
        self.last_effective_time += by;
    }
}