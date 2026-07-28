use godot::prelude::*;

// pull hooke_spring module into scope
use super::super::hooke_spring;
use hooke_spring::{HookeSpring, HookeSpringDamper, HookeSpringSpeed};
// pull ElapsedTimeSecs type into scope
use super::super::clocks::units::ElapsedTimeSecs;

// pull black magic spells into scope
use super::macros::{assign_spring_compat_traits, hs_variant_match_typed, hs_variant_match_untyped, hs_variant_match_double_typed};

use std::ops::{Add, AddAssign, Mul, MulAssign};
fn make_new_spring<T>(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>) -> HookeSpring<T>
where T: Default + Copy + AddAssign + Add<T, Output = T> + Mul<f64, Output = T> + MulAssign<f64>,
for<'a> &'a T: Mul<f64, Output = T> {
    // TODO: None is a placeholder, replace with Godot's timekeeper instead
    HookeSpring::<T>::from_damper_speed(damper.unwrap_or(1.0), speed.unwrap_or(1.0), None)
}

pub struct HSFloat(pub f64);
assign_spring_compat_traits!(HSFloat, f64, f64);
pub struct HSVector2(pub Vector2);
assign_spring_compat_traits!(HSVector2, Vector2, f32);
pub struct HSVector3(pub Vector3);
assign_spring_compat_traits!(HSVector3, Vector3, f32);
pub struct HSVector4(pub Vector4);
assign_spring_compat_traits!(HSVector4, Vector4, f32);

pub enum HSCompatibleTypes {
    Float(HSFloat),
    Vector2(HSVector2),
    Vector3(HSVector3),
    Vector4(HSVector4),
}

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
    pub fn new_float(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Float(make_new_spring::<HSFloat>(damper, speed)),
            base,
        })
    }
    pub fn new_vector2(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector2(make_new_spring::<HSVector2>(damper, speed)),
            base,
        })
    }
    pub fn new_vector3(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector3(make_new_spring::<HSVector3>(damper, speed)),
            base,
        })
    }
    pub fn new_vector4(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector4(make_new_spring::<HSVector4>(damper, speed)),
            base,
        })
    }
    // Setters //
    // See ../macros.rs for more information on what the heck these macros do... //
    pub fn impulse(&mut self, by: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::impulse, by)
    }
    pub fn shift(&mut self, by: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::shift, by)
    }
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::time_skip, by)
    }
    pub fn set_target(&mut self, to: HSCompatibleTypes, do_not_animate: Option<bool>) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::set_target, to, do_not_animate)
    }
    pub fn set_damper(&mut self, to: HookeSpringDamper) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_damper, to)
    }
    pub fn set_speed(&mut self, to: HookeSpringSpeed) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_speed, to)
    }
    pub fn set_damper_and_speed(&mut self, damper_to: HookeSpringDamper, speed_to: HookeSpringSpeed) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_damper_and_speed, damper_to, speed_to)
    }
    pub fn set_position(&mut self, to: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::set_position, to)
    }
    pub fn set_velocity(&mut self, to: HSCompatibleTypes) {
        hs_variant_match_typed!(&mut self.spring, HookeSpring::set_velocity, to)
    }
    pub fn set_position_and_velocity(&mut self, position_to: HSCompatibleTypes, velocity_to: HSCompatibleTypes) {
        hs_variant_match_double_typed!(&mut self.spring, HookeSpring::set_position_velocity, position_to, velocity_to)
    }
    // TODO getters
}

#[godot_api]
impl IRefCounted for RSHookeSpring {}