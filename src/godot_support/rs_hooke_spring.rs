// pull godot-rust shenanigans into scope
use godot::classes::class_macros::private::virtuals::ZipReader::Variant;
use godot::meta::Element;
use godot::meta::conv::ByValue;
use godot::meta::shape::GodotShape;
use godot::prelude::*;

// pull hooke_spring module into scope
use super::super::hooke_spring;
use hooke_spring::{HookeSpring, HookeSpringDamper, HookeSpringSpeed, HookeSpringClock};
// pull ElapsedTimeSecs type into scope
use super::super::clocks::units::ElapsedTimeSecs;

// pull black magic spells (macro_rules!) into scope
use super::macros::*;
use super::super::macros::*;

use super::rs_stopwatch::*;

use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, MulAssign};
fn make_new_spring<T>(damper: Option<HookeSpringDamper>, speed: Option<HookeSpringSpeed>, clock: Option<Gd<RSStopwatch>>) -> HookeSpring<T>
where T: Default + Copy + AddAssign + Add<T, Output = T> + Mul<f64, Output = T> + MulAssign<f64>,
for<'a> &'a T: Mul<f64, Output = T> {
    let clock_to_inject: InnerRSStopwatch = match clock {
        Some(gd_instance) => {
            gd_instance.bind().inner_clone()
        }
        None => {
            InnerRSStopwatch::new()
        }
    };
    let boxed_clock: Box<dyn HookeSpringClock> = Box::new(clock_to_inject);
    HookeSpring::<T>::from_damper_speed(damper.unwrap_or(1.0), speed.unwrap_or(1.0), Some(boxed_clock))
}

#[derive(Debug)]
pub struct HSFloat(pub f64);
assign_spring_compat_traits_all!(HSFloat, f64, f64);

#[derive(Debug)]
pub struct HSVector2(pub Vector2);
assign_spring_compat_traits_all!(HSVector2, Vector2, f32);

#[derive(Debug)]
pub struct HSVector3(pub Vector3);
assign_spring_compat_traits_all!(HSVector3, Vector3, f32);

#[derive(Debug)]
pub struct HSVector4(pub Vector4);
assign_spring_compat_traits_all!(HSVector4, Vector4, f32);

// Yes, these are kind of DRY violations but there's no pragmatically better way as-is I think.
// This sucks.

#[derive(Debug)]
pub struct HSTransform2D(pub Transform2D);
assign_spring_compat_traits_gdmatrix!(HSTransform2D, Transform2D);
impl Mul<f64> for HSTransform2D {
    type Output = HSTransform2D;
    fn mul(self, rhs: f64) -> Self::Output {
        HSTransform2D(Transform2D::IDENTITY.interpolate_with(&self.0, rhs as f32))
    }
}
impl MulAssign<f64> for HSTransform2D {
    fn mul_assign(&mut self, rhs: f64) {
        let mut lhs = self.0;
        lhs = lhs * (rhs as f32);
        self.0 = lhs;
    }
}
impl<'a> Mul<f64> for &'a HSTransform2D {
    type Output = HSTransform2D;
    fn mul(self, rhs: f64) -> Self::Output {
        HSTransform2D(Transform2D::IDENTITY.interpolate_with(&self.0, rhs as f32))
    }
}

#[derive(Debug)]
pub struct HSTransform3D(pub Transform3D);
assign_spring_compat_traits_gdmatrix!(HSTransform3D, Transform3D);
impl Mul<f64> for HSTransform3D {
    type Output = HSTransform3D;
    fn mul(self, rhs: f64) -> Self::Output {
        HSTransform3D(Transform3D::IDENTITY.interpolate_with(&self.0, rhs as f32))
    }
}
impl MulAssign<f64> for HSTransform3D {
    fn mul_assign(&mut self, rhs: f64) {
        let mut lhs = self.0;
        lhs = lhs * (rhs as f32);
        self.0 = lhs;
    }
}
impl<'a> Mul<f64> for &'a HSTransform3D {
    type Output = HSTransform3D;
    fn mul(self, rhs: f64) -> Self::Output {
        HSTransform3D(Transform3D::IDENTITY.interpolate_with(&self.0, rhs as f32))
    }
}

#[derive(Debug)]
pub struct HSBasis(pub Basis);
assign_spring_compat_traits_gdmatrix!(HSBasis, Basis);
impl Mul<f64> for HSBasis {
    type Output = HSBasis;
    fn mul(self, rhs: f64) -> Self::Output {
        HSBasis(Basis::IDENTITY.slerp(&self.0, rhs as f32))
    }
}
impl MulAssign<f64> for HSBasis {
    fn mul_assign(&mut self, rhs: f64) {
        let mut lhs = self.0;
        lhs = lhs * (rhs as f32);
        self.0 = lhs;
    }
}
impl<'a> Mul<f64> for &'a HSBasis {
    type Output = HSBasis;
    fn mul(self, rhs: f64) -> Self::Output {
        HSBasis(Basis::IDENTITY.slerp(&self.0, rhs as f32))
    }
}

#[derive(Debug)]
pub struct HSQuaternion(pub Quaternion);
assign_spring_compat_traits_gdmatrix!(HSQuaternion, Quaternion);
impl Mul<f64> for HSQuaternion {
    type Output = HSQuaternion;
    fn mul(self, rhs: f64) -> Self::Output {
        HSQuaternion(Quaternion::IDENTITY.slerp(self.0.normalized(), rhs as f32))
    }
}
impl MulAssign<f64> for HSQuaternion {
    fn mul_assign(&mut self, rhs: f64) {
        let mut lhs = self.0;
        lhs = lhs * (rhs as f32);
        self.0 = lhs;
    }
}
impl<'a> Mul<f64> for &'a HSQuaternion {
    type Output = HSQuaternion;
    fn mul(self, rhs: f64) -> Self::Output {
        HSQuaternion(Quaternion::IDENTITY.slerp(self.0.normalized(), rhs as f32))
    }
}


#[derive(Debug)]
pub enum HSCompatibleTypes {
    Float(HSFloat),
    Vector2(HSVector2),
    Vector3(HSVector3),
    Vector4(HSVector4),
    Transform2D(HSTransform2D),
    Transform3D(HSTransform3D),
    Basis(HSBasis),
    Quaternion(HSQuaternion),
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
        } else if let Ok(t2) = via.try_to::<Transform2D>() {
            Ok(HSCompatibleTypes::Transform2D(HSTransform2D(t2)))
        } else if let Ok(t3) = via.try_to::<Transform3D>() {
            Ok(HSCompatibleTypes::Transform3D(HSTransform3D(t3)))
        } else if let Ok(bs) = via.try_to::<Basis>() {
            Ok(HSCompatibleTypes::Basis(HSBasis(bs)))
        } else if let Ok(qt) = via.try_to::<Quaternion>() {
            Ok(HSCompatibleTypes::Quaternion(HSQuaternion(qt)))
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
            HSCompatibleTypes::Transform2D(val) => val.0.to_variant(),
            HSCompatibleTypes::Transform3D(val) => val.0.to_variant(),
            HSCompatibleTypes::Basis(val) => val.0.to_variant(),
            HSCompatibleTypes::Quaternion(val) => val.0.to_variant(),
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
    Transform2D(HookeSpring<HSTransform2D>),
    Transform3D(HookeSpring<HSTransform3D>),
    Basis(HookeSpring<HSBasis>),
    Quaternion(HookeSpring<HSQuaternion>),
}
impl Default for RSHookeSpringVariant {
    fn default() -> Self {
        RSHookeSpringVariant::Float(make_new_spring::<HSFloat>(None, None, None))
    }
}

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct RSHookeSpring {
    spring: RSHookeSpringVariant,
    base: Base<RefCounted>,
}

#[godot_api]
impl RSHookeSpring {
    // Constructors //
    #[func]
    pub fn new_float(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Float(make_new_spring::<HSFloat>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_vector2(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector2(make_new_spring::<HSVector2>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_vector3(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector3(make_new_spring::<HSVector3>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_vector4(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Vector4(make_new_spring::<HSVector4>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_transform2d(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Transform2D(make_new_spring::<HSTransform2D>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_transform3d(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Transform3D(make_new_spring::<HSTransform3D>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_basis(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Basis(make_new_spring::<HSBasis>(Some(damper), Some(speed), clock)),
            base,
        })
    }
    #[func]
    pub fn new_quaternion(damper: HookeSpringDamper, speed: HookeSpringSpeed, clock: Option<Gd<RSStopwatch>>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            spring: RSHookeSpringVariant::Quaternion(make_new_spring::<HSQuaternion>(Some(damper), Some(speed), clock)),
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
    pub fn set_damper_speed(&mut self, damper_to: HookeSpringDamper, speed_to: HookeSpringSpeed) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::set_damper_speed, damper_to, speed_to)
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
    pub fn set_position_velocity(&mut self, position_to: HSCompatibleTypes, velocity_to: HSCompatibleTypes) {
        hs_variant_match_double_typed!(&mut self.spring, HookeSpring::set_position_velocity, position_to, velocity_to)
    }
    // Getters //
    #[func]
    pub fn get_position(&mut self) -> HSCompatibleTypes {
        hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::position)
    }
    #[func]
    pub fn get_velocity(&mut self) -> HSCompatibleTypes {
        hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::velocity)
    }
    #[func]
    pub fn get_position_and_velocity(&mut self) -> Array<HSCompatibleTypes> {
        let (pos, vel) = hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::position_velocity, true);
        let mut arr = Array::new();
        arr.push(pos);
        arr.push(vel);
        arr
    }
    #[func]
    pub fn get_target(&mut self) -> HSCompatibleTypes {
        hs_variant_getter_generic_deref!(&mut self.spring, HookeSpring::target)
    }
    #[func]
    pub fn get_damper(&mut self) -> f64 {
        *hs_variant_match_untyped!(&mut self.spring, HookeSpring::damper)
    }
    #[func]
    pub fn get_speed(&mut self) -> f64 {
        *hs_variant_match_untyped!(&mut self.spring, HookeSpring::speed)
    }
    #[func]
    pub fn get_elapsed_time(&mut self) -> f64 {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::elapsed_time)
    }
    #[func]
    pub fn get_clock_copy(&self) -> Gd<RSStopwatch> {
        let clock = hs_variant_match_untyped!(&self.spring, HookeSpring::clock);
        if let Some(inner) = clock.as_any().downcast_ref::<InnerRSStopwatch>() {
            RSStopwatch::from_inner(inner)
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
            RSStopwatch::new_running()
        }
    } 

    #[func]
    pub fn time_skip(&mut self, by: ElapsedTimeSecs) {
        hs_variant_match_untyped!(&mut self.spring, HookeSpring::time_skip, by)
    }
    #[func]
    pub fn time_dilate(&mut self, multiplier: f64) {
        let clock_mut = hs_variant_match_untyped!(&mut self.spring, HookeSpring::clock_mut);
        if let Some(inner) = clock_mut.as_any_mut().downcast_mut::<InnerRSStopwatch>() {
            inner.time_dilate(multiplier);
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
        }
    }
    #[func]
    pub fn time_skip_raw(&mut self, by: ElapsedTimeSecs) {
        let clock_mut = hs_variant_match_untyped!(&mut self.spring, HookeSpring::clock_mut);
        if let Some(inner) = clock_mut.as_any_mut().downcast_mut::<InnerRSStopwatch>() {
            inner.time_skip_raw(by);
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
        }
    }
    #[func]
    pub fn get_time_scale(&self) -> f64 {
        let clock = hs_variant_match_untyped!(&self.spring, HookeSpring::clock);
        if let Some(inner) = clock.as_any().downcast_ref::<InnerRSStopwatch>() {
            *inner.time_scale()
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
            1.0f64
        }
    }
    #[func]
    pub fn get_engine_elapsed_time(&self) -> ElapsedTimeSecs {
        let clock = hs_variant_match_untyped!(&self.spring, HookeSpring::clock);
        if let Some(inner) = clock.as_any().downcast_ref::<InnerRSStopwatch>() {
            inner.real_elapsed()
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
            0.0f64
        }
    }
    #[func]
    pub fn get_created_time(&self) -> ElapsedTimeSecs {
        let clock = hs_variant_match_untyped!(&self.spring, HookeSpring::clock);
        if let Some(inner) = clock.as_any().downcast_ref::<InnerRSStopwatch>() {
            inner.created_time()
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
            0.0f64
        }
    }

    #[func]
    pub fn is_paused(&self) -> bool {
        let clock = hs_variant_match_untyped!(&self.spring, HookeSpring::clock);
        if let Some(inner) = clock.as_any().downcast_ref::<InnerRSStopwatch>() {
            inner.is_paused()
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
            false
        }
    }
    #[func]
    pub fn pause(&mut self) {
        let clock_mut = hs_variant_match_untyped!(&mut self.spring, HookeSpring::clock_mut);
        if let Some(inner) = clock_mut.as_any_mut().downcast_mut::<InnerRSStopwatch>() {
            inner.pause();
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
        }
    }
    #[func]
    pub fn resume(&mut self) {
        let clock_mut = hs_variant_match_untyped!(&mut self.spring, HookeSpring::clock_mut);
        if let Some(inner) = clock_mut.as_any_mut().downcast_mut::<InnerRSStopwatch>() {
            inner.resume();
        } else {
            godot_error!("Expected InnerRSStopwatch, got something else");
        }
    }
}

#[godot_api]
impl IRefCounted for RSHookeSpring {}