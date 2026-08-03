//! Contains implementations specifically for Godot Engine.
//! Primarily intended to be used within GDScript.
//! 
//! To properly register the library in an existing project using godot-rust,
//! remember to add this to your root crate's `lib.rs`:
//! ```
//! extern crate hooke_spring;
//! ```
//! 
//! For examples within Godot, please refer to `/examples/godot_bind` in the source code
//! as they are written in GDScript.
//! 
//! The `gdextension` `entry_symbol` of this library is `rs_hooke_spring_lib`.
use godot::prelude::*;

pub struct GDHookeSpringExtension;
#[gdextension(entry_symbol = rs_hooke_spring_lib)]
unsafe impl ExtensionLibrary for GDHookeSpringExtension {}

pub mod rs_hooke_spring;
pub mod rs_stopwatch;
mod macros;