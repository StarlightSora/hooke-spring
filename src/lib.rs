//! A library containing a lightweight spring simulator according to Hooke's law.
pub mod hooke_spring;
pub mod clocks;
pub mod macros; // Needs to be pub to suppress unused import warning
pub mod prelude;

#[cfg(feature = "godot_bind")]
pub mod godot_support;