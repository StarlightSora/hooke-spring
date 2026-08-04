use std::ops::{Add, AddAssign, Mul, MulAssign};

#[cfg(feature = "std")]
fn main() {
    // performance test in a somewhat realistic scenario where you'd use this library
    // in a game framework or engine (i.e. Godot through godot-rs or Bevy). This is heavily simplified,
    // and in real world usage there would be some performance overhead,
    // but the library itself should be very performant
    use std::sync::{Mutex, Arc};
    use std::thread;
    use std::f64::consts::PI;
    use std::time::Instant;
    use hooke_spring::prelude::*;

    const fn to_rad(deg: f64) -> f64 {
        deg * PI / 180.0f64
    }
    const SIM_DT: f64 = 1.0/240.0;
    const SIM_STEPS: u32 = 24000;
    const FIRE_RATE_DT: f64 = 60.0/850.0;
    const T_RECOIL_KICK: SimpleVector3 = SimpleVector3 { x: 0.05, y: -0.10, z: -0.04};
    const R_RECOIL_KICK: SimpleVector3 = SimpleVector3 { x: to_rad(10.0), y: to_rad(-7.5), z: to_rad(3.0) };

    let trans_recoil_spr = Arc::new(Mutex::new(HookeSpring::<SimpleVector3>::from_damper_speed(0.75, 15.0, None)));
    let rot_recoil_spr = Arc::new(Mutex::new(HookeSpring::<SimpleVector3>::from_damper_speed(0.65, 10.0, None)));
    let clock = Arc::new(Mutex::new(ManualClock::new()));
    let mut last_fired: f64 = 0.0;

    let clock_clo1 = Arc::clone(&clock);
    let trans_clo1 = Arc::clone(&trans_recoil_spr);
    let rot_clo1 = Arc::clone(&rot_recoil_spr);
    let handle1 = thread::spawn(move || {
        for _ in 0..SIM_STEPS {
            if clock_clo1.lock().unwrap().evaluate_elapsed() >= (last_fired + FIRE_RATE_DT) {
                trans_clo1.lock().unwrap().impulse(T_RECOIL_KICK);
                rot_clo1.lock().unwrap().impulse(R_RECOIL_KICK);
                last_fired += FIRE_RATE_DT;
            }
            clock_clo1.lock().unwrap().time_skip(SIM_DT);
        }
    });

    let now = Instant::now();
    handle1.join().unwrap();
    let wall_clock_elapsed_us = now.elapsed().as_micros();
    let wall_clock_elapsed_ns = now.elapsed().as_nanos();
    let time_spent_per_frame_ns = wall_clock_elapsed_ns/(SIM_STEPS as u128);
    println!("Simulation of {} steps ({} seconds) took {}us", SIM_STEPS, SIM_DT*(SIM_STEPS as f64), &wall_clock_elapsed_us);
    println!("Average time spent per frame: {}ns, target framerate delta time: {}us", time_spent_per_frame_ns, SIM_DT*1000.0*1000.0);
}
#[cfg(feature = "no_std")]
fn main() {}

pub struct SimpleVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for SimpleVector3 {
    fn default() -> Self {
        Self {
            x: 0.0f64,
            y: 0.0f64,
            z: 0.0f64,
        }
    }
}
impl Clone for SimpleVector3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl Copy for SimpleVector3 {}
impl AddAssign for SimpleVector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Add for SimpleVector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Mul<f64> for SimpleVector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl MulAssign<f64> for SimpleVector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl<'a> Mul<f64> for &'a SimpleVector3 {
    type Output = SimpleVector3;
    fn mul(self, rhs: f64) -> Self::Output {
        SimpleVector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

