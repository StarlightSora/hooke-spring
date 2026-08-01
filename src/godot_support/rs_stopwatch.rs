use godot::classes::Time;
use godot::prelude::*;

use super::super::hooke_spring::HookeSpringClock;
use super::super::clocks::units::*;

const fn usec_to_ets(usec: u64) -> ElapsedTimeSecs {
    usec as f64 / 1000.0f64 / 1000.0f64
}

#[derive(Debug, Clone)]
pub struct InnerRSStopwatch {
    created: ElapsedTimeSecs,
    last_evaluated_real: ElapsedTimeSecs,
    last_effective_time: ElapsedTimeSecs,
    time_scale: f64,
    paused: bool,
}
impl HookeSpringClock for InnerRSStopwatch {
    fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs {
        if self.paused {
            self.last_effective_time
        } else {
            let raw_elapsed_time = self.real_elapsed();
            let raw_elapsed_diff = raw_elapsed_time - self.last_evaluated_real;
            
            let dilated_time_diff = raw_elapsed_diff * self.time_scale;
            self.last_effective_time += dilated_time_diff;
            self.last_evaluated_real = raw_elapsed_time;

            self.last_effective_time
        }
    }
    fn time_skip(&mut self, by: ElapsedTimeSecs) {
        if !self.paused { self.evaluate_elapsed(); }
        self.last_effective_time += by * self.time_scale;
    }
}
impl Default for InnerRSStopwatch {
    fn default() -> Self {
        Self::new()
    }
}
impl InnerRSStopwatch {
    pub fn new() -> Self {
        Self {
            created: usec_to_ets(Time::singleton().get_ticks_usec()),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
            paused: false,
        }
    }
    pub fn new_stopped() -> Self {
        Self {
            created: usec_to_ets(Time::singleton().get_ticks_usec()),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
            paused: true,
        }
    }
    pub fn wrapped() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn created_time(&self) -> ElapsedTimeSecs {
        self.created
    }
    pub fn time_scale(&self) -> &f64 {
        &self.time_scale
    }
    pub fn real_elapsed(&self) -> ElapsedTimeSecs {
        usec_to_ets(Time::singleton().get_ticks_usec()) - self.created
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }
    pub fn pause(&mut self) {
        if !self.paused {
            self.evaluate_elapsed();
            self.paused = true;
        }
    }
    pub fn resume(&mut self) {
        if self.paused {
            // There should be no reason to `evaluate_elapsed()` on a stopwatch that was paused earlier
            // However we need to set last_evaluated_real to the current engine time so downstream calculations are accurate
            self.last_evaluated_real = self.real_elapsed();
            self.paused = false;
        }
    }

    pub fn time_dilate(&mut self, multiplier: f64) {
        if !self.paused { self.evaluate_elapsed(); }
        self.time_scale = multiplier;
    }
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        if !self.paused { self.evaluate_elapsed(); }
        self.last_effective_time += by;
    }

    pub fn reset(&mut self) {
        self.last_evaluated_real = self.real_elapsed();
        self.last_effective_time = 0.0f64;
    }
}

#[derive(GodotClass)]
#[class(base=Resource, no_init)]
pub struct RSStopwatch {
    pub inner: InnerRSStopwatch,
    pub base: Base<Resource>,
}

#[godot_api]
impl RSStopwatch {
    #[func]
    pub fn new_running() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: InnerRSStopwatch::new(),
                base,
            }
        })
    }
    #[func]
    pub fn new_manual() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: InnerRSStopwatch::new_stopped(),
                base,
            }
        })
    }

    pub fn from_inner(inner: &InnerRSStopwatch) -> Gd<Self> {
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
    pub fn get_created_time(&self) -> f64 {
        self.inner.created_time()
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
    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }
    #[func]
    pub fn pause(&mut self) {
        self.inner.pause();
    }
    #[func]
    pub fn resume(&mut self) {
        self.inner.resume();
    }

    #[func]
    pub fn time_dilate(&mut self, multiplier: f64) {
        self.inner.time_dilate(multiplier)
    }
    #[func]
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        self.inner.time_skip_raw(by)
    }

    #[func]
    pub fn reset(&mut self) {
        self.inner.reset()
    }

    pub fn inner_clone(&self) -> InnerRSStopwatch {
        self.inner.clone()
    }
}

#[godot_api]
impl IResource for RSStopwatch {}