#[cfg(feature = "std")]
fn main() {
    use hooke_spring::prelude::*;
    use std::{time, thread};

    let mut spring = HookeSpring::<f64>::from_damper_speed(0.75, 8.0, Some(SmolStopwatch::wrapped()));
    spring.impulse(10.0);
    for i in 1..=10 {
        let (pos, vel) = spring.position_velocity();
        println!("[Iteration {i}] Position: {}, Velocity: {}", pos, vel);
        thread::sleep(time::Duration::from_millis(100));
    }
}
#[cfg(feature = "no_std")]
fn main() {}