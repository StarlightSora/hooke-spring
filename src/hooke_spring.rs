//! Contains `HookeSpring`, a spring you can simulate based on Hooke's Law.
extern crate alloc;
use alloc::boxed::Box;
use core::ops::{Add, AddAssign, Mul, MulAssign};
use crate::ManualClock;

#[cfg(feature = "libm")]
use libm::{sqrt, exp, cos, sin};

use super::clocks::units::ElapsedTimeSecs;

/// Traits that constitute a `HookeSpringClock`.
/// It is able to evaluate the elapsed time,
/// and able to modify the elapsed time.
pub trait HookeSpringClock: Send + Sync + core::fmt::Debug + HookeSpringClockClone {
    /// Evaluates how much time has passed since the instance's creation.
    fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs;
    /// Forcibly advances the elapsed time.
    fn time_skip(&mut self, by: ElapsedTimeSecs);
}
/// Alias for a `HookeSpringClock` trait object wrapped in a `Box`.
pub type WrappedHookeSpringClock = Box<dyn HookeSpringClock>;

/// Trait used to make WrappedHookeSpringClock clonable. Implement using the clone trait object pattern.
pub trait HookeSpringClockClone {
    fn clone_box(&self) -> WrappedHookeSpringClock;
}
impl<T> HookeSpringClockClone for T
where T: 'static + HookeSpringClock + Clone {
    fn clone_box(&self) -> WrappedHookeSpringClock {
        Box::new(self.clone())
    }
}
impl Clone for WrappedHookeSpringClock {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Alias for `f64` for clarity.
pub type HookeSpringDamper = f64;
/// Alias for `f64` for clarity.
pub type HookeSpringSpeed = f64;
/// A spring that can be simulated according to Hooke's Law.
/// 
/// It is evaluated lazily (only when needed).
/// 
/// All mutating methods cause the instance to **fully re-evaluate itself**, unless specified otherwise.
/// Full re-evaluation is defined as the instance re-evaluating the elapsed time, `position` and `velocity.`
/// 
/// Timekeeping is done by the `clock` of the instance.
/// `clock` should always return a finite value when `evaluate_elapsed` is called on it.
/// 
/// `T` should be finite for all mutating methods that accept `T` in its parameters.
/// Likewise, all mutating methods that accept `HookeSpringDamper` or `HookeSpringSpeed` should be finite.
/// 
/// If these contracts are broken, then invalid values will propagate through the instance,
/// and downstream behavior of the instance becomes undefined.
/// 
/// To be specific, `position` and `velocity` will always be invalid if any of the properties become invalid.
/// `target`, `damper` and `speed` will not be invalid, unless they are the source of the propagation.
/// Elapsed time will not be invalid, unless `clock` was mutated to return an invalid value. 
#[derive(Debug)]
#[derive(Clone)]
pub struct HookeSpring<T> {
    position: T,
    velocity: T,
    target: T,
    damper: HookeSpringDamper,
    speed: HookeSpringSpeed,
    last_evaluated: ElapsedTimeSecs,
    clock: WrappedHookeSpringClock,
}

impl<T> Default for HookeSpring<T>
where T: Default + Copy + AddAssign + Add<T, Output = T> + Mul<f64, Output = T> + MulAssign<f64>,
for<'a> &'a T: Mul<f64, Output = T> {
    /// The default constructor for `HookeSpring::T`.
    /// 
    /// It is a shorthand for `HookeSpring::<T>::new(None, None, None, None, None, None)`.
    fn default() -> Self {
        Self::new(None, None, None, None, None, None)
    }
}
impl<T> HookeSpring<T> 
where T: Default + Copy + AddAssign + Add<T, Output = T> + Mul<f64, Output = T> + MulAssign<f64>,
for<'a> &'a T: Mul<f64, Output = T> {
    /// The general-purpose constructor for `HookeSpring::T`.
    /// 
    /// `position`, `velocity` and `target` defaults to the default of `T`, if not provided.
    /// `damper` and `speed` defaults to `1.0f64`.
    /// `clock` defaults to a `ManualClock` instance wrapped in a `Box`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use hooke_spring::HookeSpring;
    ///
    /// // Create a spring with default values
    /// let mut spring = HookeSpring::<f64>::new(None, None, None, None, None, None);
    ///
    /// // Query its position and velocity
    /// let (pos, vel) = spring.position_and_velocity();
    /// assert_eq!(*pos, 0.0);
    /// assert_eq!(*vel, 0.0);
    /// ```
    pub fn new(
        position: Option<T>, velocity: Option<T>, target: Option<T>,
        damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>,
        clock: Option<WrappedHookeSpringClock>,
    ) -> Self {
        Self {
            position: position.unwrap_or_default(),
            velocity: velocity.unwrap_or_default(),
            target: target.unwrap_or_default(),
            damper: damper.unwrap_or(1.0f64),
            speed: speed.unwrap_or(1.0f64),
            last_evaluated: 0.0f64,
            clock: clock.unwrap_or_else(|| ManualClock::wrapped()),
        }
    }
    /// Construct a `HookeSpring<T>` instance from the given `damper` and `speed`, and optional `clock`.
    /// 
    /// The position, velocity and target of the resulting instance will be set to the default of `T`.
    /// In other words, it is a shorthand for `HookeSpring::<T>::new(None, None, None, Some(damper), Some(speed), clock)`.
    pub fn from_damper_speed(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<WrappedHookeSpringClock>) -> Self {
        Self::new(None, None, None, Some(damper), Some(speed), clock)
    }

    /// Apply an impulse to the spring, incrementing its velocity.
    /// 
    /// # Examples
    ///
    /// ```
    /// use hooke_spring::HookeSpring;
    ///
    /// let mut spring = HookeSpring::from_damper_speed(0.75, 2.0, None);
    /// spring.impulse(5.0);
    ///
    /// // After the impulse, velocity should be incremented
    /// assert_eq!(*spring.velocity(), 5.0);
    /// ```
    pub fn impulse(&mut self, by: T) {
        self.re_evaluate_and_update();
        self.velocity += by;
    }
    /// Shifts (increments) the position of the spring.
    pub fn shift(&mut self, by: T) {
        self.re_evaluate_and_update();
        self.position += by;
    }
    /// Forcibly advances the elapsed time.
    /// 
    /// This does not cause the instance to re-evaluate itself; it only mutates the `clock`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use hooke_spring::HookeSpring;
    ///
    /// let mut spring = HookeSpring::from_damper_speed(0.75, 2.0, None);
    /// spring.time_skip(4.0);
    ///
    /// // 4 seconds should have elapsed on spring
    /// assert_eq!(spring.elapsed_time(), 4.0);
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.clock.time_skip(by);
    }
    /// Modifies the `target`.
    /// 
    /// If `do_not_animate` is `Some(true)`, the position and target are set immediately
    /// and velocity is reset to zero. This only causes the instance to re-evaluate the elapsed time.
    /// 
    /// Otherwise, the instance fully re-evaluates itself, then the target is updated.
    /// 
    /// # Examples
    ///
    /// ```
    /// use hooke_spring::HookeSpring;
    ///
    /// let mut spring = HookeSpring::from_damper_speed(0.75, 2.0, None);
    /// // Set the target without animating
    /// spring.set_target(10.0, Some(true));
    ///
    /// // Position and target are immediately updated
    /// assert_eq!(*spring.position(), 10.0);
    /// assert_eq!(*spring.target(), 10.0);
    /// 
    /// let mut spring = HookeSpring::from_damper_speed(0.75, 2.0, None);
    /// // Set the target with animating
    /// spring.set_target(10.0, None);
    /// 
    /// // Only the target is updated
    /// assert_eq!(*spring.position(), 0.0);
    /// assert_eq!(*spring.target(), 10.0);
    /// ```
    pub fn set_target(&mut self, to: T, do_not_animate: Option<bool>) {
        let no_anim = do_not_animate.unwrap_or(false);
        if no_anim {
            self.position = to;
            self.velocity *= 0.0;
            self.target = to;
            let new_elapsed = self.elapsed_time();
            self.update_last_evaluated(new_elapsed);
        } else {
            self.re_evaluate_and_update();
            self.target = to;
        }
    }
    /// Modifies the `damper`.
    pub fn set_damper(&mut self, to: HookeSpringDamper) {
        self.re_evaluate_and_update();
        self.damper = to;
    }
    /// Modifies the `speed`.
    pub fn set_speed(&mut self, to: HookeSpringSpeed) {
        self.re_evaluate_and_update();
        self.speed = to;
    }
    /// Modifies the `damper` and `speed` at the same time.
    pub fn set_damper_speed(&mut self, damper_to: HookeSpringDamper, speed_to: HookeSpringSpeed) {
        self.re_evaluate_and_update();
        self.damper = damper_to;
        self.speed = speed_to;
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::set_damper_speed instead!")] // Deprecated due to naming inconsistencies
    pub fn set_damper_and_speed(&mut self, damper_to: HookeSpringDamper, speed_to: HookeSpringSpeed) {
        self.set_damper_speed(damper_to, speed_to);
    }
    /// Modifies the `position`.
    pub fn set_position(&mut self, to: T) {
        let elapsed = self.elapsed_time();
        let (_, vel) = self.re_evaluate(&elapsed);
        self.position = to;
        self.velocity = vel;
        self.update_last_evaluated(elapsed);
    }
    /// Modifies the `velocity`.
    pub fn set_velocity(&mut self, to: T) {
        let elapsed = self.elapsed_time();
        let (pos, _) = self.re_evaluate(&elapsed);
        self.position = pos;
        self.velocity = to;
        self.update_last_evaluated(elapsed);
    }
    /// Modifies the `position` and `velocity` at the same time.
    /// 
    /// This only causes the instance to re-evaluate its elapsed time.
    pub fn set_position_velocity(&mut self, position_to: T, velocity_to: T) {
        // there is no point to call re_evaluate when we will overwrite both position and velocity anyways
        let elapsed = self.elapsed_time();
        self.position = position_to;
        self.velocity = velocity_to;
        self.update_last_evaluated(elapsed);
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::set_position_velocity instead!")] // Deprecated due to naming inconsistencies
    pub fn set_position_and_velocity(&mut self, position_to: T, velocity_to: T) {
        self.set_position_velocity(position_to, velocity_to);
    }
    
    /// Queries the `position`.
    pub fn position(&mut self) -> &T {
        self.re_evaluate_and_update();
        &self.position
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::position instead!")] // Deprecated due to violating C-GETTER
    pub fn get_position(&mut self) -> &T {
        self.position()
    }
    /// Queries the `velocity`.
    pub fn velocity(&mut self) -> &T {
        self.re_evaluate_and_update();
        &self.velocity
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::velocity instead!")] // Deprecated due to violating C-GETTER
    pub fn get_velocity(&mut self) -> &T {
        self.velocity()
    }
    /// Queries the `position` and `velocity` at the same time.
    pub fn position_velocity(&mut self) -> (&T, &T) {
        self.re_evaluate_and_update();
        (&self.position, &self.velocity)
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::position_velocity instead!")] // Deprecated due to violating C-GETTER
    pub fn get_position_and_velocity(&mut self) -> (&T, &T) {
        self.position_velocity()
    }
    /// Queries the `target`.
    pub fn target(&self) -> &T {
        &self.target
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::target instead!")] // Deprecated due to violating C-GETTER
    pub fn get_target(&self) -> &T {
        self.target()
    }
    /// Queries the `damper`.
    pub fn damper(&self) -> &HookeSpringDamper {
        &self.damper
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::damper instead!")] // Deprecated due to violating C-GETTER
    pub fn get_damper(&self) -> &HookeSpringDamper {
        self.damper()
    }
    /// Queries the `speed`.
    pub fn speed(&self) -> &HookeSpringSpeed {
        &self.speed
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::speed instead!")] // Deprecated due to violating C-GETTER
    pub fn get_speed(&self) -> &HookeSpringSpeed {
        self.speed()
    }
    /// Queries how long the instance has been simulating for.
    /// 
    /// This only causes the instance to re-evaluate its elapsed time.
    pub fn elapsed_time(&mut self) -> ElapsedTimeSecs {
        self.clock.evaluate_elapsed()
    }
    #[deprecated(since="0.2.0-dev2", note="Use Self::elapsed_time instead!")] // Deprecated due to violating C-GETTER
    pub fn get_elapsed_time(&mut self) -> ElapsedTimeSecs {
        self.elapsed_time()
    }

    /// Provides a mutable reference to the `clock`.
    /// 
    /// Mutating the clock directly may affect how the instance evaluates elapsed time.
    pub fn clock_mut(&mut self) -> &mut WrappedHookeSpringClock {
        self.re_evaluate_and_update();
        &mut self.clock
    }

    // wrapper for calling `re_evaluate`, updating `position` and `velocity`, then calling `update_last_evaluated`
    fn re_evaluate_and_update(&mut self) {
        let elapsed = self.elapsed_time();
        let (new_position, new_velocity) = self.re_evaluate(&elapsed);
        self.position = new_position;
        self.velocity = new_velocity;
        self.update_last_evaluated(elapsed);
    }
    // calculate where `position` and `velocity` should be at the given `elapsed` time
    fn re_evaluate(&mut self, elapsed: &ElapsedTimeSecs) -> (T, T) {
        const ONE: f64 = 1.0f64;
        const TWO: f64 = 2.0f64;
        let p0 = &self.position;
        let v0 = &self.velocity;
        let p1 = &self.target;
        let d = &self.damper;
        let s = &self.speed;
        
        let t = s * (elapsed - self.last_evaluated);
        let d2 = self.damper * self.damper;

        let (h, si, co): (f64, f64, f64);
        #[cfg(not(feature = "libm"))]
        if d2 < ONE {
            h = f64::sqrt(ONE - d2);
            let ep = f64::exp(-d * t) / h;
            co = ep * f64::cos(h * t);
            si = ep * f64::sin(h * t);
        } else if d2 == ONE {
            h = ONE;
            let ep = f64::exp(-d * t) / h;
            co = ep;
            si = ep * t;
        } else {
            h = f64::sqrt(d2 - ONE);
            let u = f64::exp((-d + h) * t) / (TWO * h);
            let v = f64::exp((-d - h) * t) / (TWO * h);
            co = u + v;
            si = u - v;
        }
        #[cfg(feature = "libm")]
        if d2 < ONE {
            h = sqrt(ONE - d2);
            let ep = exp(-d * t) / h;
            co = ep * cos(h * t);
            si = ep * sin(h * t);
        } else if d2 == ONE {
            h = ONE;
            let ep = exp(-d * t) / h;
            co = ep;
            si = ep * t;
        } else {
            h = sqrt(d2 - ONE);
            let u = exp((-d + h) * t) / (TWO * h);
            let v = exp((-d - h) * t) / (TWO * h);
            co = u + v;
            si = u - v;
        }

        let a0 = h * co + d * si;
        let a1 = ONE - (h * co + d * si);
        let a2 = si / s;

        let b0 = -s * si;
        let b1 = s * si;
        let b2 = h * co - d * si;
        
        (
            p0*a0 + p1*a1 + v0*a2, // Position
            p0*b0 + p1*b1 + v0*b2, // Velocity
        )
    }
    fn update_last_evaluated(&mut self, elapsed: ElapsedTimeSecs) {
        self.last_evaluated = elapsed;
    }
}