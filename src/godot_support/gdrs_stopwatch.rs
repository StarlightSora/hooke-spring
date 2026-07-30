use godot::classes::Time;
use godot::prelude::*;

use super::super::hooke_spring::HookeSpringClock;
use super::super::clocks::units::*;

const fn usec_to_ets(usec: u64) -> ElapsedTimeSecs {
    usec as f64 / 1000.0f64 / 1000.0f64
}

#[derive(Debug, Clone)]
pub struct InnerGDRSStopwatch {
    created: ElapsedTimeSecs,
    last_evaluated_real: ElapsedTimeSecs,
    last_effective_time: ElapsedTimeSecs,
    time_scale: f64,
}
impl HookeSpringClock for InnerGDRSStopwatch {
    fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs {
        let raw_elapsed_time = self.real_elapsed();
        let raw_elapsed_diff = raw_elapsed_time - self.last_evaluated_real;
        
        let dilated_time_diff = raw_elapsed_diff * self.time_scale;
        self.last_effective_time += dilated_time_diff;
        self.last_evaluated_real = raw_elapsed_time;

        self.last_effective_time
    }
    fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.evaluate_elapsed();
        self.last_effective_time += by * self.time_scale;
    }
}
impl Default for InnerGDRSStopwatch {
    fn default() -> Self {
        Self::new()
    }
}
impl InnerGDRSStopwatch {
    pub fn new() -> Self {
        Self {
            created: usec_to_ets(Time::singleton().get_ticks_usec()),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
        }
    }
    pub fn new_stopped() -> Self {
        Self {
            created: usec_to_ets(Time::singleton().get_ticks_usec()),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 0.0,
        }
    }
    pub fn wrapped() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn time_scale(&self) -> &f64 {
        &self.time_scale
    }
    pub fn real_elapsed(&self) -> ElapsedTimeSecs {
        usec_to_ets(Time::singleton().get_ticks_usec()) - self.created
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

#[derive(GodotClass)]
#[class(base=Resource, no_init)]
pub struct GDRSStopwatch {
    pub inner: InnerGDRSStopwatch,
    pub base: Base<Resource>,
}

#[godot_api]
impl GDRSStopwatch {
    #[func]
    pub fn new_running() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: InnerGDRSStopwatch::new(),
                base,
            }
        })
    }
    #[func]
    pub fn new_manual() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: InnerGDRSStopwatch::new_stopped(),
                base,
            }
        })
    }

    pub fn from_inner(inner: &InnerGDRSStopwatch) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: inner.clone(),
                base,
            }
        })
    }

    #[func]
    pub fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs {
        self.inner.evaluate_elapsed()
    }
    #[func]
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.inner.time_skip(by)
    }

    #[func]
    pub fn get_time_scale(&self) -> f64 {
        *self.inner.time_scale()
    }
    #[func]
    pub fn get_real_elapsed(&self) -> ElapsedTimeSecs {
        self.inner.real_elapsed()
    }
    #[func]
    pub fn time_dilate(&mut self, multiplier: f64) {
        self.inner.time_dilate(multiplier)
    }
    #[func]
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        self.inner.time_skip_raw(by)
    }

    pub fn inner_clone(&self) -> InnerGDRSStopwatch {
        self.inner.clone()
    }
}

#[godot_api]
impl IResource for GDRSStopwatch {}