//! Contains implementation of the `hooke-spring` crate specifically for Godot Engine.
//! 
//! For examples, please refer to `/examples/godot_bind` in the source code.
use godot::prelude::*;

pub struct GDHookeSpringExtension;
unsafe impl ExtensionLibrary for GDHookeSpringExtension {}

pub mod rs_hooke_spring;
pub mod rs_stopwatch;
mod macros;