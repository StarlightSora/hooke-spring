//! You are entering the realm of black magic spells;
//! dirty(?) macro hacks because Rust is overly strict
//! and godot is the polar opposite of that
//! 
//! These macros are intended for use within the godot_support module

// Is there really no better way to handle these? Seriously? This is idiomatic Rust?

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
        }
    };
}
pub(super) use hs_variant_getter_generic_deref;