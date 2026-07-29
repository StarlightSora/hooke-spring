// TODO: This module name is kind of misleading?

// pull godot-rust shenanigans into scope
use godot::classes::class_macros::private::virtuals::ZipReader::Variant;
use godot::meta::Element;
use godot::meta::conv::ByValue;
use godot::meta::shape::GodotShape;
use godot::prelude::*;

// pull hooke_spring module into scope
use super::super::hooke_spring;
use hooke_spring::{HookeSpring, HookeSpringDamper, HookeSpringSpeed};
// pull ElapsedTimeSecs type into scope
use super::super::clocks::units::ElapsedTimeSecs;

// pull black magic spells (macro_rules!) into scope
use super::macros::*;
use super::super::macros::assign_spring_compat_traits;

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign};
fn make_new_spring<T>(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>) -> HookeSpring<T>
where T: Default + Copy + AddAssign + Add<T, Output = T> + Mul<f64, Output = T> + MulAssign<f64>,
for<'a> &'a T: Mul<f64, Output = T> {
    // TODO: None is a placeholder, replace with Godot's timekeeper instead
    HookeSpring::<T>::from_damper_speed(damper.unwrap_or(1.0), speed.unwrap_or(1.0), None)
}

#[derive(Debug)]
pub struct HSFloat(pub f64);
assign_spring_compat_traits!(HSFloat, f64, f64);

#[derive(Debug)]
pub struct HSVector2(pub Vector2);
assign_spring_compat_traits!(HSVector2, Vector2, f32);

#[derive(Debug)]
pub struct HSVector3(pub Vector3);
assign_spring_compat_traits!(HSVector3, Vector3, f32);

#[derive(Debug)]
pub struct HSVector4(pub Vector4);
assign_spring_compat_traits!(HSVector4, Vector4, f32);

#[derive(Debug)]
pub enum HSCompatibleTypes {
    Float(HSFloat),
    Vector2(HSVector2),
    Vector3(HSVector3),
    Vector4(HSVector4),
}
impl GodotConvert for HSCompatibleTypes {
    type Via = Variant;
    fn godot_shape() -> godot::meta::shape::GodotShape {
        GodotShape::Variant
    }
}
impl FromGodot for HSCompatibleTypes {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        if let Ok(f) = via.try_to::<f64>() {
            Ok(HSCompatibleTypes::Float(HSFloat(f)))
        } else if let Ok(v2) = via.try_to::<Vector2>() {
            Ok(HSCompatibleTypes::Vector2(HSVector2(v2)))
        } else if let Ok(v3) = via.try_to::<Vector3>() {
            Ok(HSCompatibleTypes::Vector3(HSVector3(v3)))
        } else if let Ok(v4) = via.try_to::<Vector4>() {
            Ok(HSCompatibleTypes::Vector4(HSVector4(v4)))
        } else {
            Err(ConvertError::with_error("Conversion failed!"))
        }
    }
}
impl ToGodot for HSCompatibleTypes {
    type Pass = ByValue;
    fn to_godot(&self) -> godot::meta::ToArg<'_, Self::Via, Self::Pass> {
        match self {
            HSCompatibleTypes::Float(val) => val.0.to_variant(),
            HSCompatibleTypes::Vector2(val) => val.0.to_variant(),
            HSCompatibleTypes::Vector3(val) => val.0.to_variant(),
            HSCompatibleTypes::Vector4(val) => val.0.to_variant(),
        }
    }
}
impl Element for HSCompatibleTypes {}

#[derive(Debug)]
pub enum RSHookeSpringVariant {
    Float(HookeSpring<HSFloat>),
    Vector2(HookeSpring<HSVector2>),
    Vector3(HookeSpring<HSVector3>),
    Vector4(HookeSpring<HSVector4>),
}
impl Default for RSHookeSpringVariant {
    fn default() -> Self {
        RSHookeSpringVariant::Float(make_new_spring::<HSFloat>(None, None))
    }
}

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct RSHookeSpring {
    spring: RSHookeSpringVariant,
    base: Base<RefCounted>
}

#[godot_api]
impl RSHookeSpring {
    // Constructors //
    #[func]
    pub fn new_float(damper: HookeSpringDamper, speed: HookeSpringSpeed) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Float(make_new_spring::<HSFloat>(Some(damper), Some(speed))),
            base,
        })
    }
    #[func]
    pub fn new_vector2(damper: HookeSpringDamper, speed: HookeSpringSpeed) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector2(make_new_spring::<HSVector2>(Some(damper), Some(speed))),
            base,
        })
    }
    #[func]
    pub fn new_vector3(damper: HookeSpringDamper, speed: HookeSpringSpeed) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector3(make_new_spring::<HSVector3>(Some(damper), Some(speed))),
            base,
        })
    }
    #[func]
    pub fn new_vector4(damper: HookeSpringDamper, speed: HookeSpringSpeed) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector4(make_new_spring::<HSVector4>(Some(damper), Some(speed))),
            base,
        })
    }
    // Setters //
    // See ../macros.rs for more information on what the heck these macros do... //
    #[func]
    pub fn impulse(&mut self, by: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::impulse, by)
    }
    #[func]
    pub fn shift(&mut self, by: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::shift, by)
    }
    #[func]
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::time_skip, by)
    }
    #[func]
    pub fn set_target(&mut self, to: HSCompatibleTypes, do_not_animate: bool) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::set_target, to, Some(do_not_animate))
    }
    #[func]
    pub fn set_damper(&mut self, to: HookeSpringDamper) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_damper, to)
    }
    #[func]
    pub fn set_speed(&mut self, to: HookeSpringSpeed) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_speed, to)
    }
    #[func]
    pub fn set_damper_and_speed(&mut self, damper_to: HookeSpringDamper, speed_to: HookeSpringSpeed) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_damper_and_speed, damper_to, speed_to)
    }
    #[func]
    pub fn set_position(&mut self, to: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::set_position, to)
    }
    #[func]
    pub fn set_velocity(&mut self, to: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::set_velocity, to)
    }
    #[func]
    pub fn set_position_and_velocity(&mut self, position_to: HSCompatibleTypes, velocity_to: HSCompatibleTypes) {
        hs_variant_match_double_typed!(&mut self.spring, HookeSpring::set_position_and_velocity, position_to, velocity_to)
    }
    // Getters //
    #[func]
    pub fn get_position(&mut self) -> HSCompatibleTypes {
        hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::get_position)
    }
    #[func]
    pub fn get_velocity(&mut self) -> HSCompatibleTypes {
        hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::get_velocity)
    }
    #[func]
    pub fn get_position_and_velocity(&mut self) -> Array<HSCompatibleTypes> {
        let (pos, vel) = hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::get_position_and_velocity, true);
        let mut arr = Array::new();
        arr.push(pos);
        arr.push(vel);
        arr
    }
    #[func]
    pub fn get_target(&mut self) -> HSCompatibleTypes {
        hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::get_target)
    }
    #[func]
    pub fn get_damper(&mut self) -> f64 {
        *hs_variant_match_untyped!(&mut self.spring, HookeSpring::get_damper)
    }
    #[func]
    pub fn get_speed(&mut self) -> f64 {
        *hs_variant_match_untyped!(&mut self.spring, HookeSpring::get_speed)
    }
    #[func]
    pub fn get_elapsed_time(&mut self) -> f64 {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::get_elapsed_time)
    }

    // clock_mut // 
    pub fn clock_mut(&mut self) {
        // TODO: Figure out what the hell to link as the `clock` for Godot's case first
        unimplemented!()
    }
}

#[godot_api]
impl IRefCounted for RSHookeSpring {}