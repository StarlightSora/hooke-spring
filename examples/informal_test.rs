#[cfg(feature = "std")]
fn main() {
    use std::time::{Instant, Duration};
    use hooke_spring::HookeSpring;

    let mut spring = HookeSpring::new(
        Some(0.0),   // position
        Some(0.0),   // velocity
        Some(1.0),   // target
        Some(0.75),  // damper (defaults to 1.0)
        Some(2.0),  // speed (defaults to 1.0)
        None,  // clock (defaults to Box::new(SmolStopwatch::new()))
    );

    // Kick the spring with an impulse
    spring.impulse(5.0);

    // Print a few steps
    let now = Instant::now();
    for _ in 0..10 {
        let (pos, vel) = spring.get_position_and_velocity();
        println!("t+:{}ms: pos = {}, vel = {}", now.elapsed().as_millis(), pos, vel);
        std::thread::sleep(Duration::from_millis(200));
    }
}
