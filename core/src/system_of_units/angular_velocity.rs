use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{basic_ops, Float, RAD_PER_DEGREE, DEGREE_PER_RAD};

#[derive(Copy, Clone, Default)]
pub struct AngularVelocity(pub Float);
basic_ops!(AngularVelocity);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl AngularVelocity {
    /// Create an instance of type AngularVelocity from a float number in RAD per s
    #[inline]
    pub fn from_rad_s(value: Float) -> Self {
        AngularVelocity(value)
    }

    /// Create an instance of type AngularVelocity from a float number in DEG per s
    #[inline]
    pub fn from_deg_s(value: Float) -> Self {
        AngularVelocity(value * RAD_PER_DEGREE)
    }

    /// Create an instance of type AngularVelocity from a float number in DEG per min
    #[inline]
    pub fn from_deg_min(value: Float) -> Self {
        AngularVelocity(value * RAD_PER_DEGREE / 60.0)
    }

    /// Extract a float number in the unit RAD per s
    #[inline]
    pub fn to_rad_s(self) -> Float {
        self.0
    }

    /// Extract a float number in the unit DEG per s
    #[inline]
    pub fn to_deg_s(self) -> Float {
        self.0 * DEGREE_PER_RAD
    }

    /// Extract a float number in the unit DEG per min
    #[inline]
    pub fn to_deg_min(self) -> Float {
        self.0 * DEGREE_PER_RAD * 60.0
    }

}

/// Trait to convert data to the struct [AngularVelocity]
#[allow(non_snake_case)]
pub trait FloatToAngularVelocity {
    /// Create an instance of type [AnglularVelocity] from a number RAD per s
    fn rad_s(self) -> AngularVelocity;
    /// Create an instance of type [AnglularVelocity] from a number DEG per s
    fn deg_s(self) -> AngularVelocity;
    /// Create an instance of type [AnglularVelocity] from a number DEG per min
    fn deg_min(self) -> AngularVelocity;
}

impl FloatToAngularVelocity for Float {
    #[inline]
    fn rad_s(self) -> AngularVelocity {
        AngularVelocity::from_rad_s(self)
    }
    #[inline]
    fn deg_s(self) -> AngularVelocity {
        AngularVelocity::from_deg_s(self)
    }
    #[inline]
    fn deg_min(self) -> AngularVelocity {
        AngularVelocity::from_deg_min(self)
    }
}
