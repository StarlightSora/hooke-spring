//! Contains macros to reduce boilerplate

/// Implement necessary traits to newtypes; when multiplying $inner with f64, cast to $mul_precision for compatibility (typically f64 or f32)
/// 
/// This macro will fail if the inner type does not implement any of the required traits.
#[macro_export]
macro_rules! assign_spring_compat_traits_all {
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
        impl Default for $what where $inner: Default {
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
pub use assign_spring_compat_traits_all;