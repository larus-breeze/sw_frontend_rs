use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::POUND;
use crate::{basic_ops, Float};

/// Mass is both a property of a physical body and a measure of its resistance to acceleration
/// (rate of change of velocity with respect to time) when a net force is applied. An object's
/// mass also determines the strength of its gravitational attraction to other bodies
/// ([Wikipedia](https://en.wikipedia.org/wiki/Mass)).
/// SI unit name is kilogram, unit symbol is kg.
#[derive(Copy, Clone, Default)]
pub struct Mass(pub Float);
basic_ops!(Mass);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Mass {
    /// Create an instance of type Mass from a float number in pound
    #[inline]
    pub fn from_lb(value: Float) -> Self {
        Mass(value * POUND)
    }

    /// Create an instance of type Mass from a float number in kilogram
    #[inline]
    pub fn from_kg(value: Float) -> Self {
        Mass(value)
    }

    /// Extract a float number in the unit pound
    #[inline]
    pub fn to_lb(self) -> Float {
        self.0 * 1.0 / POUND
    }

    /// Extract a float number in the unit kilogram
    #[inline]
    pub fn to_kg(self) -> Float {
        self.0
    }
}

/// Trait to convert data to the struct [Mass]
#[allow(non_snake_case)]
pub trait FloatToMass {
    /// Create an instance of type [Mass] from a number in kilogram
    fn kg(self) -> Mass;
    /// Create an instance of type [Mass] from a number in pound
    fn lb(self) -> Mass;
}

impl FloatToMass for Float {
    #[inline]
    fn kg(self) -> Mass {
        Mass::from_kg(self)
    }
    #[inline]
    fn lb(self) -> Mass {
        Mass::from_lb(self)
    }
}
