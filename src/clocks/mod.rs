//! Contains simple clocks with integrated support for use in `HookeSpring`.
pub mod manual_clock;
#[cfg(feature = "smol_stopwatch")]
pub mod smol_stopwatch;
pub mod units;