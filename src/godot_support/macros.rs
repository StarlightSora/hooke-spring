//! You are entering the realm of black magic spells;
//! "dirty" macro hacks because Rust is overly strict

// Is there really no better way to handle these? Seriously?

// Assign necessary traits to newtypes; when multiplying $inner with f64, cast to $mul_precision for compatibility (typically f64 or f32)
// TODO: Maybe split these? So the entire macro doesn't shoot itself on the foot if the original type doesn't have a trait here??
// TODO 2: Move this to src/macros.rs instead, and enable access to external crates to this macro?
macro_rules! assign_spring_compat_traits {
    ($what: ident, $inner: ty, $mul_precision: ty) => {
        impl core::ops::Add<$what> for $what {
            type Output = $what;
            fn add(self, rhs: $what) -> Self::Output {
                $what(self.0 + rhs.0)
            }
        }
        impl core::ops::AddAssign<$what> for $what {
            fn add_assign(&mut self, rhs: $what) {
                self.0 += rhs.0;
            }
        }
        impl core::ops::Mul<f64> for $what {
            type Output = $what;
            fn mul(self, rhs: f64) -> Self::Output {
                $what(self.0 * rhs as $mul_precision)
            }
        }
        impl core::ops::MulAssign<f64> for $what {
            fn mul_assign(&mut self, rhs: f64) {
                self.0 *= (rhs as $mul_precision);
            }
        }
        impl<'a> core::ops::Mul<f64> for &'a $what {
            type Output = $what;
            fn mul(self, rhs: f64) -> Self::Output {
                $what(self.0 * rhs as $mul_precision)
            }
        }
        impl Default for $what {
            fn default() -> Self {
                $what(<$inner>::default())
            }
        }
        impl Clone for $what where $inner: Copy {
            fn clone(&self) -> Self {
                $what(self.0.clone())
            }
        }
        impl Copy for $what where $inner: Copy {}
    };
}
pub(super) use assign_spring_compat_traits;

// Shorthand for branching logic when calling operations on springs
// For a given HookeSpring<T> instance when $primary is type T
macro_rules! hs_variant_match_typed {
    ($spr: expr, $method: expr, $primary: expr $(, $extra: expr)*) => {
        match ($spr, $primary) {
            (RSHookeSpringVariant::Float(spring), HSCompatibleTypes::Float(val)) => {
                $method(spring, val$(, $extra)*);
            }
            (RSHookeSpringVariant::Vector2(spring), HSCompatibleTypes::Vector2(val)) => {
                $method(spring, val$(, $extra)*);
            }
            (RSHookeSpringVariant::Vector3(spring), HSCompatibleTypes::Vector3(val)) => {
                $method(spring, val$(, $extra)*);
            }
            (RSHookeSpringVariant::Vector4(spring), HSCompatibleTypes::Vector4(val)) => {
                $method(spring, val$(, $extra)*);
            }
            _ => {
                godot_error!("Type mismatch!");
            }
        }
    };
}
pub(super) use hs_variant_match_typed;

// Shorthand for branching logic when calling operations on springs
// For a given HookeSpring<T> instance when $primary and $secondary are type T
// Literally only used for one case but whatever
macro_rules! hs_variant_match_double_typed {
    ($spr: expr, $method: expr, $primary: expr, $secondary: expr $(, $extra: expr)*) => {
        match ($spr, $primary, $secondary) {
            (RSHookeSpringVariant::Float(spring), HSCompatibleTypes::Float(val), HSCompatibleTypes::Float(val2)) => {
                $method(spring, val, val2$(, $extra)*);
            }
            (RSHookeSpringVariant::Vector2(spring), HSCompatibleTypes::Vector2(val), HSCompatibleTypes::Vector2(val2)) => {
                $method(spring, val, val2$(, $extra)*);
            }
            (RSHookeSpringVariant::Vector3(spring), HSCompatibleTypes::Vector3(val), HSCompatibleTypes::Vector3(val2)) => {
                $method(spring, val, val2$(, $extra)*);
            }
            (RSHookeSpringVariant::Vector4(spring), HSCompatibleTypes::Vector4(val), HSCompatibleTypes::Vector4(val2)) => {
                $method(spring, val, val2$(, $extra)*);
            }
            _ => {
                godot_error!("Type mismatch!");
            }
        }
    };
}
pub(super) use hs_variant_match_double_typed;

// Shorthand for branching logic when calling operations on springs
// For a given HookeSpring<T> instance when $primary is NOT type T
macro_rules! hs_variant_match_untyped {
    ($spr: expr, $method: expr, $primary: expr $(, $extra: expr)*) => {
        match ($spr) {
            RSHookeSpringVariant::Float(spring) => {
                $method(spring, $primary$(, $extra)*);
            }
            RSHookeSpringVariant::Vector2(spring) => {
                $method(spring, $primary$(, $extra)*);
            }
            RSHookeSpringVariant::Vector3(spring) => {
                $method(spring, $primary$(, $extra)*);
            }
            RSHookeSpringVariant::Vector4(spring) => {
                $method(spring, $primary$(, $extra)*);
            }
        }
    };
}
pub(super) use hs_variant_match_untyped;