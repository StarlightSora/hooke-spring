use godot::prelude::*;

pub struct GDHookeSpringExtension;
unsafe impl ExtensionLibrary for GDHookeSpringExtension {}

pub mod rs_hooke_spring;
pub mod rs_stopwatch;
mod macros;