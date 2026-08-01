//! A library containing a lightweight spring simulator according to Hooke's law.
pub use clocks::manual_clock::ManualClock;
#[cfg(feature = "smol_stopwatch")]
pub use clocks::smol_stopwatch::SmolStopwatch;
pub use clocks::units::ElapsedTimeSecs;

pub use hooke_spring::HookeSpringSpeed;
pub use hooke_spring::HookeSpringDamper;
pub use hooke_spring::HookeSpring;

pub mod hooke_spring;
pub mod clocks;
pub mod macros; // Needs to be pub to suppress unused import warning
pub mod prelude;

#[cfg(feature = "godot_bind")]
pub mod godot_support;