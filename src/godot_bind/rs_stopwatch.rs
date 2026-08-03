//! Contains the `clock` implementation for use in Godot.
use godot::classes::Time;
use godot::prelude::*;

use super::super::hooke_spring::HookeSpringClock;
use super::super::clocks::units::*;

const fn usec_to_ets(usec: u64) -> ElapsedTimeSecs {
    usec as f64 / 1000.0f64 / 1000.0f64
}

#[derive(Debug, Clone)]
/// The inside of `RSStopWatch`.
/// It is used for `RSHookeSpring`'s `clock`.
/// 
/// Note: This implicitly gets dilated by Godot Engine's time scale.
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
    /// Forcibly advances the elapsed time, with respect to time dilation by `time_scale`.
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
    /// Creates a running `InnerRSStopwatch` instance.
    pub fn new() -> Self {
        Self {
            created: usec_to_ets(Time::singleton().get_ticks_usec()),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
            paused: false,
        }
    }
    /// Creates a paused `InnerRSStopwatch` instance.
    pub fn new_stopped() -> Self {
        Self {
            created: usec_to_ets(Time::singleton().get_ticks_usec()),
            last_evaluated_real: 0.0,
            last_effective_time: 0.0,
            time_scale: 1.0,
            paused: true,
        }
    }

    /// Query when this was created, according to Godot Engine's `Time`.
    pub fn created_time(&self) -> ElapsedTimeSecs {
        self.created
    }
    /// Query the `time_scale`.
    pub fn time_scale(&self) -> &f64 {
        &self.time_scale
    }
    /// Query how much engine time passed since creation.
    pub fn real_elapsed(&self) -> ElapsedTimeSecs {
        usec_to_ets(Time::singleton().get_ticks_usec()) - self.created
    }

    /// Query if the instance is paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }
    /// Pause the instance.
    /// 
    /// This causes the instance to re-evaluate itself if it was not paused.
    pub fn pause(&mut self) {
        if !self.paused {
            self.evaluate_elapsed();
            self.paused = true;
        }
    }
    /// Resume the instance.
    pub fn resume(&mut self) {
        if self.paused {
            // There should be no reason to `evaluate_elapsed()` on a stopwatch that was paused earlier
            // However we need to set last_evaluated_real to the current engine time so downstream calculations are accurate
            self.last_evaluated_real = self.real_elapsed();
            self.paused = false;
        }
    }

    /// Modifies the `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself if it's not paused.
    pub fn time_dilate(&mut self, multiplier: f64) {
        if !self.paused { self.evaluate_elapsed(); }
        self.time_scale = multiplier;
    }
    /// Forcibly advances the elapsed time, ignoring time dilation by `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself if it's not paused.
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        if !self.paused { self.evaluate_elapsed(); }
        self.last_effective_time += by;
    }
    /// Sets `last_effective_time` to `0.0f64`,
    /// so that `Self::evaluate_elapsed` would return something very close to `0.0f64`
    /// if called right after calling this method.
    pub fn reset(&mut self) {
        self.last_evaluated_real = self.real_elapsed();
        self.last_effective_time = 0.0f64;
    }
}

#[derive(GodotClass)]
#[class(base=Resource, no_init)]
/// A stopwatch, with support for skipping and dilating time.
/// 
/// Primarily used for `RSHookeSpring`, but can be used on its own.
/// 
/// Note: This implicitly gets dilated by Godot Engine's time scale.
pub struct RSStopwatch {
    pub inner: InnerRSStopwatch,
    pub base: Base<Resource>,
}

#[godot_api]
impl RSStopwatch {
    #[func]
    /// Creates a running `RSStopwatch` instance.
    pub fn new_running() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: InnerRSStopwatch::new(),
                base,
            }
        })
    }
    #[func]
    /// Creates a paused `RSStopwatch` instance.
    pub fn new_manual() -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: InnerRSStopwatch::new_stopped(),
                base,
            }
        })
    }

    /// Creates a `RSStopwatch` instance from the data of an existing `InnerRSStopwatch` instance.
    pub fn from_inner(inner: &InnerRSStopwatch) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                inner: inner.clone(),
                base,
            }
        })
    }

    #[func]
    /// Evaluates how much time has passed since the instance's creation.
    pub fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs {
        self.inner.evaluate_elapsed()
    }
    #[func]
    /// Forcibly advances the elapsed time, with respect to time dilation by `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.inner.time_skip(by)
    }

    #[func]
    /// Query when this was created, according to Godot Engine's `Time`.
    pub fn get_created_time(&self) -> f64 {
        self.inner.created_time()
    }
    #[func]
    /// Query the `time_scale`.
    pub fn get_time_scale(&self) -> f64 {
        *self.inner.time_scale()
    }
    #[func]
    /// Query how much engine time passed since creation.
    pub fn get_real_elapsed(&self) -> ElapsedTimeSecs {
        self.inner.real_elapsed()
    }

    #[func]
    /// Query if the instance is paused.
    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }
    #[func]
    /// Pause the instance.
    /// 
    /// This causes the instance to re-evaluate itself if it was not paused.
    pub fn pause(&mut self) {
        self.inner.pause();
    }
    #[func]
    /// Resume the instance.
    pub fn resume(&mut self) {
        self.inner.resume();
    }

    #[func]
    /// Modifies the `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself if it's not paused.
    pub fn time_dilate(&mut self, multiplier: f64) {
        self.inner.time_dilate(multiplier)
    }
    #[func]
    /// Forcibly advances the elapsed time, ignoring time dilation by `time_scale`.
    /// 
    /// This causes the instance to re-evaluate itself if it's not paused.
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        self.inner.time_skip_raw(by)
    }

    #[func]
    /// Sets `last_effective_time` to `0.0f64`,
    /// so that `Self::evaluate_elapsed` would return something very close to `0.0f64`
    /// if called right after calling this method.
    pub fn reset(&mut self) {
        self.inner.reset()
    }

    /// Returns a copy of this instance's `inner`.
    pub fn inner_clone(&self) -> InnerRSStopwatch {
        self.inner.clone()
    }
}

#[godot_api]
impl IResource for RSStopwatch {}