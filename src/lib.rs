pub use clocks::manual_clock::ManualClock;
pub use clocks::smol_stopwatch::SmolStopwatch;
pub use hooke_spring::HookeSpring;

pub mod hooke_spring;
pub mod clocks;