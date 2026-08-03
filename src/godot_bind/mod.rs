//! Contains implementations specifically for Godot Engine.
//! Primarily intended to be used within GDScript.
//! 
//! For examples, please refer to `/examples/godot_bind` in the source code
//! as they are written in GDScript.
use godot::prelude::*;

pub struct GDHookeSpringExtension;
unsafe impl ExtensionLibrary for GDHookeSpringExtension {}

pub mod rs_hooke_spring;
pub mod rs_stopwatch;
mod macros;