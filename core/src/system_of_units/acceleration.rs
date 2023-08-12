use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{basic_ops, Float};

/// In mechanics, acceleration is the rate of change of the velocity of an object with respect 
/// to time. Accelerations are vector quantities (in that they have magnitude and direction).
/// The orientation of an object's acceleration is given by the orientation of the net force 
/// acting on that object.
/// ([Wikipedia](https://en.wikipedia.org/wiki/Acceleration)).
/// The SI unit for acceleration is metre per second squared, unit symbol is m/s².
#[derive(Copy, Clone, Default)]
pub struct Acceleration(pub Float);

basic_ops!(Acceleration);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Acceleration {
    /// Create an instance of type Acceleration from a float number in metre per second squared
    #[inline]
    pub fn from_m_s2(value: Float) -> Self {
        Acceleration(value)
    }

    /// Extract a float number in the unit metre per second squared
    #[inline]
    pub fn to_m_s2(self) -> Float {
        self.0
    }
}

/// Trait to convert data to the struct [Acceleration]
#[allow(non_snake_case)]
pub trait FloatToAcceleration {
    /// Create an instance of type [Acceleration] from a number in metre per second squared
    fn m_s2(self) -> Acceleration;
}

impl FloatToAcceleration for Float {
    #[inline]
    fn m_s2(self) -> Acceleration {
        Acceleration::from_m_s2(self)
    }
}
