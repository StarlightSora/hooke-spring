#[cfg(test)]
mod hooke_spring_tests {
    use std::{f64::consts::PI};
    use hooke_spring::{ManualClock, hooke_spring::*};

    #[test]
    fn constructors() {
        let mut hs = HookeSpring::default();
        let (pos, vel) = hs.get_position_and_velocity();
        // both should be 0.0 even though SmolStopwatch is nondeterministic as the instance
        // initialized with 0.0 position, velocity and target
        assert_eq!(*pos, 0.0);
        assert_eq!(*vel, 0.0);
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);

        let mut hs = HookeSpring::from_damper_speed(2.0, 3.0, Some(ManualClock::wrapped()));
        let (pos, vel) = hs.get_position_and_velocity();
        assert_eq!(*pos, 0.0);
        assert_eq!(*vel, 0.0);
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 2.0);
        assert_eq!(*hs.get_speed(), 3.0);

        let mut hs = HookeSpring::new(Some(2.0), Some(-1.0), Some(3.0), Some(1.5), Some(5.0), Some(ManualClock::wrapped()));
        let (pos, vel) = hs.get_position_and_velocity();
        assert_eq!(*pos, 2.0);
        assert_eq!(*vel, -1.0);
        assert_eq!(*hs.get_target(), 3.0);
        assert_eq!(*hs.get_damper(), 1.5);
        assert_eq!(*hs.get_speed(), 5.0);
    }
    #[test]
    fn simulations() {
        // helper function to account for floating point precision errors
        fn close_enough(a: f64, b: f64) -> bool {
            println!("a: {}, b: {}", a, b);
            f64::abs(a - b) < 0.0000001
        }

        // make an overdampened spring
        let mut hs = HookeSpring::from_damper_speed(2.0, 3.0, Some(ManualClock::wrapped()));
        hs.impulse(1.0);
        hs.shift(-0.5);
        let (pos, vel) = hs.get_position_and_velocity();
        assert_eq!(*pos, -0.5);
        assert_eq!(*vel, 1.0);
        
        hs.time_skip(2.0);
        let (pos, vel) = hs.get_position_and_velocity();
        // position should be < 0.0 because hs is overdampened (2.0), and it was shifted below its target (0.0)
        assert!(*pos < 0.0);
        // velocity should be > 0.0 because hs is overdampened, and it was impulsed towards its target
        assert!(*vel > 0.0);

        // make an undampened spring with speed set to PI, so the frequency of both position and velocity are 1.0,
        // and so that we can ignore logarithmic decay (as it is undampened), simplifying our sanity check here
        let mut hs = HookeSpring::from_damper_speed(0.0, PI, Some(ManualClock::wrapped()));
        hs.set_target(1.0, None); // set the target to 1.0
        assert_eq!(*hs.get_target(), 1.0);

        // skip ahead by 0.5, the spring should have velocity PI with position at 1.0
        hs.time_skip(0.5);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(close_enough(*pos, 1.0));
        assert!(close_enough(*vel, PI));

        // skip ahead by 0.5 again, now velocity should be 0.0 with position at 2.0
        hs.time_skip(0.5);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(close_enough(*pos, 2.0));
        assert!(close_enough(*vel, 0.0));

        // skip ahead by 0.5 again, now velocity should be -PI with position at 1.0
        hs.time_skip(0.5);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(close_enough(*pos, 1.0));
        assert!(close_enough(*vel, -PI));

        // skip ahead by 0.5 again, now velocity should be 0.0 with position at 0.0
        hs.time_skip(0.5);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(close_enough(*pos, 0.0));
        assert!(close_enough(*vel, 0.0));

        // skip ahead by 0.25 again, now velocity should be PI*sin(45deg) with position at 1.0 + (-cos(45deg))
        hs.time_skip(0.25);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(close_enough(*pos, 1.0 -f64::cos(PI/4.0)));
        assert!(close_enough(*vel, PI*f64::sin(PI/4.0)));
    }
    #[test]
    fn edge_cases() {
        // preventing edge cases is the caller's responsibility,
        // but we check them here to verify invalid states propagates through the instance as we logically expect it to
        // and not to invariants that were not messed with

        // Case A: target is now invalid
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.set_target(f64::INFINITY, None);
        let (pos, vel) = hs.get_position_and_velocity();
        // these are now invalid values
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert!(!&hs.get_target().is_finite());
        // since damper and speed were not mutated, these should still be 1.0
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert_eq!(hs.get_elapsed_time(), 0.0); // clock was never advanced

        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.set_target(f64::NAN, None);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert!(!&hs.get_target().is_finite());
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert_eq!(hs.get_elapsed_time(), 0.0);

        // Case B: velocity is now invalid
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.impulse(f64::INFINITY);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        // these were never touched
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert_eq!(hs.get_elapsed_time(), 0.0);

        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.impulse(f64::NAN);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert_eq!(hs.get_elapsed_time(), 0.0);

        // Case C: position is now invalid
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.shift(f64::INFINITY);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert_eq!(hs.get_elapsed_time(), 0.0);

        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.shift(f64::NAN);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert_eq!(hs.get_elapsed_time(), 0.0);

        // Case D: damper and speed are now invalid
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.set_damper_and_speed(f64::INFINITY, f64::INFINITY);
        // This time, everything but target and elapsed time should be invalid because the damper and speed was passed an invalid value
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert!(!&hs.get_damper().is_finite());
        assert!(!&hs.get_speed().is_finite());
        assert_eq!(hs.get_elapsed_time(), 0.0);
        
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.set_damper_and_speed(f64::NAN, f64::NAN);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert!(!&hs.get_damper().is_finite());
        assert!(!&hs.get_speed().is_finite());
        assert_eq!(hs.get_elapsed_time(), 0.0);

        // Case E: time_skipped by an invalid value
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.time_skip(f64::INFINITY);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        // these 3 were never touched
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert!(!hs.get_elapsed_time().is_finite()); // we messed up the clock with indirection, so this is invalid

        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        hs.time_skip(f64::NAN);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert!(!hs.get_elapsed_time().is_finite());

        // Case F: mut_clock was used irresponsibly, causing the next re-evaluation of the clock to give an invalid value
        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        let clock = hs.clock_mut().as_mut();
        clock.time_skip(f64::INFINITY);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        // these 3 were never touched
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert!(!hs.get_elapsed_time().is_finite()); // we messed up the clock directly, so this is invalid

        let mut hs = HookeSpring::from_damper_speed(1.0, 1.0, Some(ManualClock::wrapped()));
        let clock = hs.clock_mut().as_mut();
        clock.time_skip(f64::NAN);
        let (pos, vel) = hs.get_position_and_velocity();
        assert!(!&pos.is_finite());
        assert!(!&vel.is_finite());
        assert_eq!(*hs.get_target(), 0.0);
        assert_eq!(*hs.get_damper(), 1.0);
        assert_eq!(*hs.get_speed(), 1.0);
        assert!(!hs.get_elapsed_time().is_finite());
    }
}