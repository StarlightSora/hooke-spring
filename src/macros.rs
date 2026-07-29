//! Contains macros to reduce boilerplate

/// Assign necessary traits to newtypes; when multiplying $inner with f64, cast to $mul_precision for compatibility (typically f64 or f32)
// TODO: Maybe split these? So the entire macro doesn't shoot itself on the foot if the original type doesn't have a trait here??
#[macro_export]
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
pub use assign_spring_compat_traits;