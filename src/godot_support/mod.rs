use godot::prelude::*;

pub struct GDHookeSpringExtension;
unsafe impl ExtensionLibrary for GDHookeSpringExtension {}

pub mod data_types;
mod macros;