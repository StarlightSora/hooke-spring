//! Contains `HookeSpring`, a spring you can simulate based on Hooke's Law.
use std::ops::{Add, AddAssign, Mul, MulAssign};
use super::clocks::{smol_stopwatch::SmolStopwatch, units::ElapsedTimeSecs};

/// Traits that constitute a `HookeSpringClock`.
/// It is able to evaluate the elapsed time,
/// and able to modify the elapsed time.
pub trait HookeSpringClock {
    /// Evaluates how much time has passed since the instance's creation.
    fn evaluate_elapsed(&mut self) -> ElapsedTimeSecs;
    /// Forcibly advances the elapsed time.
    fn time_skip(&mut self, by: ElapsedTimeSecs);
}
/// Alias for a `HookeSpringClock` trait object wrapped in a `Box`.
pub type WrappedHookeSpringClock = Box<dyn HookeSpringClock>;

/// Alias for `f64` for clarity.
pub type HookeSpringDamper = f64;
/// Alias for `f64` for clarity.
pub type HookeSpringSpeed = f64;
/// A spring that can be simulated according to Hooke's law.
/// 
/// It is evaluated lazily (only when needed).
/// All methods that causes the instance to re-evaluate itself are marked as such.
/// 
/// Timekeeping is done by the `clock` of the instance.
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
    /// The default constructor for `HookeSpring<T>`.
    /// 
    /// It is a shorthand for `HookeSpring<T>::new(None, None, None, None, None, None)`.
    fn default() -> Self {
        Self::new(None, None, None, None, None, None)
    }
}
impl<T> HookeSpring<T> 
where T: Default + Copy + AddAssign + Add<T, Output = T> + Mul<f64, Output = T> + MulAssign<f64>,
for<'a> &'a T: Mul<f64, Output = T> {
    /// The general-purpose constructor for `HookeSpring<T>`.
    /// 
    /// `position`, `velocity` and `target` defaults to the default of `T`, if not provided.
    /// `damper` and `speed` defaults to `1.0f64`.
    /// `clock` defaults to a `SmolStopwatch` instance wrapped in a `Box`.
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
            clock: clock.unwrap_or_else(|| Box::new(SmolStopwatch::new())),
        }
    }
    /// Construct a `HookeSpring<T>` instance from the given `damper` and `speed`, and optional `clock`.
    /// 
    /// The position, velocity and target of the resulting instance will be set to the default of `T`.
    /// In other words, it is a shorthand for `HookeSpring<T>::new(None, None, None, Some(damper), Some(speed), clock)`.
    pub fn from_damper_speed(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<WrappedHookeSpringClock>) -> Self {
        Self::new(None, None, None, Some(damper), Some(speed), clock)
    }

    /// Apply an impulse to the spring, incrementing its velocity.
    pub fn impulse(&mut self, by: T) {
        self.velocity += by
    }
    /// Forcibly advances the elapsed time.
    /// 
    /// This internally calls `clock.time_skip` of the instance.
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        self.clock.time_skip(by);
    }
    /// Modifies the `target`.
    /// 
    /// If `do_not_animate` is `Some(true)`, the position and target are set immediately
    /// and velocity is reset to zero. Otherwise, only the target is updated.
    pub fn set_target(&mut self, to: T, do_not_animate: Option<bool>) {
        let no_anim = do_not_animate.unwrap_or(false);
        if no_anim {
            self.position = to;
            self.velocity *= 0.0;
            self.target = to;
            let new_elapsed = self.get_elapsed_time();
            self.update_last_evaluated(new_elapsed);
        } else {
            self.target = to;
        }
    }
    /// Modifies the `damper`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn set_damper(&mut self, to: HookeSpringDamper) {
        self.re_evaluate_and_update();
        self.damper = to;
    }
    /// Modifies the `speed`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn set_speed(&mut self, to: HookeSpringSpeed) {
        self.re_evaluate_and_update();
        self.speed = to;
    }
    /// Modifies the `damper` and `speed` at the same time.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn set_damper_and_speed(&mut self, damper_to: HookeSpringDamper, speed_to: HookeSpringSpeed) {
        self.re_evaluate_and_update();
        self.damper = damper_to;
        self.speed = speed_to;
    }
    /// Modifies the `position`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn set_position(&mut self, to: T) {
        let elapsed = self.get_elapsed_time();
        let (_, vel) = self.re_evaluate(&elapsed);
        self.position = to;
        self.velocity = vel;
        self.update_last_evaluated(elapsed);
    }
    /// Modifies the `velocity`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn set_velocity(&mut self, to: T) {
        let elapsed = self.get_elapsed_time();
        let (pos, _) = self.re_evaluate(&elapsed);
        self.position = pos;
        self.velocity = to;
        self.update_last_evaluated(elapsed);
    }
    /// Modifies the `position` and `velocity` at the same time.
    /// 
    /// This does *not* cause the instance to re-evaluate itself.
    pub fn set_position_velocity(&mut self, position_to: T, velocity_to: T) {
        let elapsed = self.get_elapsed_time();
        self.position = position_to;
        self.velocity = velocity_to;
        self.update_last_evaluated(elapsed);
    }
    
    /// Queries the `position`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn get_position(&mut self) -> &T {
        self.re_evaluate_and_update();
        &self.position
    }
    /// Queries the `velocity`.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn get_velocity(&mut self) -> &T {
        self.re_evaluate_and_update();
        &self.velocity
    }
    /// Queries the `position` and `velocity` at the same time.
    /// 
    /// This causes the instance to re-evaluate itself.
    pub fn get_position_and_velocity(&mut self) -> (&T, &T) {
        self.re_evaluate_and_update();
        (&self.position, &self.velocity)
    }
    /// Queries the `target`.
    pub fn get_target(&self) -> &T {
        &self.target
    }
    /// Queries the `damper`.
    pub fn get_damper(&self) -> &HookeSpringDamper {
        &self.damper
    }
    /// Queries the `speed`.
    pub fn get_speed(&self) -> &HookeSpringSpeed {
        &self.speed
    }
    /// Queries how long the instance has been simulating for.
    pub fn get_elapsed_time(&mut self) -> ElapsedTimeSecs {
        self.clock.evaluate_elapsed()
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
        let elapsed = self.get_elapsed_time();
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
        let d2 = self.damper.powi(2);

        let (h, si, co): (f64, f64, f64);
        if d2 < ONE { // Underdampened
            h = f64::sqrt(ONE - d2);
            let ep = f64::exp(-d * t) / h;
            co = ep * f64::cos(h * t);
            si = ep * f64::sin(h * t);
        } else if d2 == ONE { // Critically damped
            h = ONE;
            let ep = f64::exp(-d * t) / h;
            co = ep;
            si = ep * t;
        } else { // Overdamped
            h = f64::sqrt(d2 - ONE);
            let u = f64::exp((-d + h) * t) / (TWO * h);
            let v = f64::exp((-d - h) * t) / (TWO * h);
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