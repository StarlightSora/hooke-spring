//! These macros are intended for use within the godot_support module.
//! 
//! Contains macros used to abstract matching logic away in the rs_hooke_spring module.

/// Shorthand for branching logic when calling operations on RSHookeSpringVariants
/// For a given HookeSpring<HSCompatibleTypes> instance when $primary is type HSCompatibleTypes
macro_rules! hs_variant_match_typed {
    ($spr: expr, $method: expr, $primary: expr $(, $extra: expr)*) => {
        match ($spr, $primary) {
            (RSHookeSpringVariant::Float(spring), HSCompatibleTypes::Float(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Vector2(spring), HSCompatibleTypes::Vector2(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Vector3(spring), HSCompatibleTypes::Vector3(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Vector4(spring), HSCompatibleTypes::Vector4(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Transform2D(spring), HSCompatibleTypes::Transform2D(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Transform3D(spring), HSCompatibleTypes::Transform3D(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Basis(spring), HSCompatibleTypes::Basis(val)) => {
                $method(spring, val$(, $extra)*)
            }
            (RSHookeSpringVariant::Quaternion(spring), HSCompatibleTypes::Quaternion(val)) => {
                $method(spring, val$(, $extra)*)
            }
            _ => {
                godot_error!("Type mismatch!")
            }
        }
    };
}
pub(super) use hs_variant_match_typed;

/// Shorthand for branching logic when calling operations on RSHookeSpringVariants
/// For a given HookeSpring<HSCompatibleTypes> instance when $primary and $secondary are type HSCompatibleTypes
// Literally only used for one case but whatever
macro_rules! hs_variant_match_double_typed {
    ($spr: expr, $method: expr, $primary: expr, $secondary: expr $(, $extra: expr)*) => {
        match ($spr, $primary, $secondary) {
            (RSHookeSpringVariant::Float(spring), HSCompatibleTypes::Float(val), HSCompatibleTypes::Float(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Vector2(spring), HSCompatibleTypes::Vector2(val), HSCompatibleTypes::Vector2(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Vector3(spring), HSCompatibleTypes::Vector3(val), HSCompatibleTypes::Vector3(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Vector4(spring), HSCompatibleTypes::Vector4(val), HSCompatibleTypes::Vector4(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Transform2D(spring), HSCompatibleTypes::Transform2D(val), HSCompatibleTypes::Transform2D(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Transform3D(spring), HSCompatibleTypes::Transform3D(val), HSCompatibleTypes::Transform3D(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Basis(spring), HSCompatibleTypes::Basis(val), HSCompatibleTypes::Basis(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            (RSHookeSpringVariant::Quaternion(spring), HSCompatibleTypes::Quaternion(val), HSCompatibleTypes::Quaternion(val2)) => {
                $method(spring, val, val2$(, $extra)*)
            }
            _ => {
                godot_error!("Type mismatch!")
            }
        }
    };
}
pub(super) use hs_variant_match_double_typed;

/// Shorthand for branching logic when calling operations on RSHookeSpringVariants
/// For a given HookeSpring<HSCompatibleTypes> instance when all the parameters are NOT type HSCompatibleTypes
macro_rules! hs_variant_match_untyped {
    ($spr: expr, $method: expr $(, $extra: expr)*) => {
        match ($spr) {
            RSHookeSpringVariant::Float(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Vector2(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Vector3(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Vector4(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Transform2D(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Transform3D(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Basis(spring) => {
                $method(spring$(, $extra)*)
            }
            RSHookeSpringVariant::Quaternion(spring) => {
                $method(spring$(, $extra)*)
            }
        }
    };
}
pub(super) use hs_variant_match_untyped;

/// Shorthand for branching logic when calling operations on RSHookeSpringVariants
/// For a given HookeSpring<HSCompatibleTypes> instance where it is expected to return **one of** HSCompatibleTypes type
macro_rules! hs_variant_getter_generic_deref {
    ($spr: expr, $method: expr) => {
        match ($spr) {
            RSHookeSpringVariant::Float(spring) => {
                HSCompatibleTypes::Float(*$method(spring))
            }
            RSHookeSpringVariant::Vector2(spring) => {
                HSCompatibleTypes::Vector2(*$method(spring))
            }
            RSHookeSpringVariant::Vector3(spring) => {
                HSCompatibleTypes::Vector3(*$method(spring))
            }
            RSHookeSpringVariant::Vector4(spring) => {
                HSCompatibleTypes::Vector4(*$method(spring))
            }
            RSHookeSpringVariant::Transform2D(spring) => {
                HSCompatibleTypes::Transform2D(*$method(spring))
            }
            RSHookeSpringVariant::Transform3D(spring) => {
                HSCompatibleTypes::Transform3D(*$method(spring))
            }
            RSHookeSpringVariant::Basis(spring) => {
                HSCompatibleTypes::Basis(*$method(spring))
            }
            RSHookeSpringVariant::Quaternion(spring) => {
                HSCompatibleTypes::Quaternion(*$method(spring))
            }
        }
    };
    // $tuple is literally just a placeholder value to match this specific case
    // it means absolutely nothing otherwise
    ($spr: expr, $method: expr, $tuple: literal) => {
        match ($spr) {
            RSHookeSpringVariant::Float(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Float(*tup.0), HSCompatibleTypes::Float(*tup.1))
            }
            RSHookeSpringVariant::Vector2(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Vector2(*tup.0), HSCompatibleTypes::Vector2(*tup.1))
            }
            RSHookeSpringVariant::Vector3(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Vector3(*tup.0), HSCompatibleTypes::Vector3(*tup.1))
            }
            RSHookeSpringVariant::Vector4(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Vector4(*tup.0), HSCompatibleTypes::Vector4(*tup.1))
            }
            RSHookeSpringVariant::Transform2D(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Transform2D(*tup.0), HSCompatibleTypes::Transform2D(*tup.1))
            }
            RSHookeSpringVariant::Transform3D(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Transform3D(*tup.0), HSCompatibleTypes::Transform3D(*tup.1))
            }
            RSHookeSpringVariant::Basis(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Basis(*tup.0), HSCompatibleTypes::Basis(*tup.1))
            }
            RSHookeSpringVariant::Quaternion(spring) => {
                let tup = $method(spring);
                (HSCompatibleTypes::Quaternion(*tup.0), HSCompatibleTypes::Quaternion(*tup.1))
            }
        }
    };
}
pub(super) use hs_variant_getter_generic_deref;

/// Similar to `assign_spring_compat_traits_all!`, but specifically designed for Godot matrix data types
/// 
/// Mul-related traits need to be manually assigned.
/// This is because naively multiplying these data types with a float
/// is clearly not what you ever want to do.
macro_rules! assign_spring_compat_traits_gdmatrix {
    ($what: ident, $inner: ty) => {
        impl core::ops::Add<$what> for $what {
            type Output = $what;
            fn add(self, rhs: $what) -> Self::Output {
                $what(self.0 * rhs.0)
            }
        }
        impl core::ops::AddAssign<$what> for $what {
            fn add_assign(&mut self, rhs: $what) {
                let mut lhs = self.0;
                lhs = lhs * rhs.0;
                self.0 = lhs;
            }
        }
        // DON'T DO THIS! This will naively multiply the stupid data structures!
        // impl core::ops::MulAssign<f64> for $what {
        //     fn mul_assign(&mut self, rhs: f64) {
        //         let mut lhs = self.0;
        //         lhs = lhs * (rhs as $mul_precision);
        //         self.0 = lhs;
        //     }
        // }
        // impl core::ops::Mul<f64> for $what {
        //     type Output = $what;
        //     fn mul(self, rhs: f64) -> Self::Output {
        //         Self(self.0 * rhs as $mul_precision)
        //     }
        // }
        // impl<'a> core::ops::Mul<f64> for &'a $what {
        //     type Output = $what;
        //     fn mul(self, rhs: f64) -> Self::Output {
        //         $what(self.0 * rhs as $mul_precision)
        //     }
        // }
        impl Default for $what where $inner: Default {
            fn default() -> Self {
                Self(<$inner>::default())
            }
        }
        impl Clone for $what where $inner: Copy {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        impl Copy for $what where $inner: Copy {}
    }
}
pub(super) use assign_spring_compat_traits_gdmatrix;
