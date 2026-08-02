//! Contains implementation of the `hooke-spring` crate specifically for Godot 4.
use godot::prelude::*;

pub struct GDHookeSpringExtension;
unsafe impl ExtensionLibrary for GDHookeSpringExtension {}

pub mod rs_hooke_spring;
pub mod rs_stopwatch;
mod macros;