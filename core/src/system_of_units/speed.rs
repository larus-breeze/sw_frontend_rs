use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::basic_ops;
use crate::system_of_units::{Float, FOOT, HOUR, KILO, MILE, MINUTE, NAUTICAL_MILE};

/// In everyday use and in kinematics, the speed (commonly referred to as v) of an object is the
/// magnitude of the rate of change of its position with time or the magnitude of the change of
/// its position per unit of time; it is thus a scalar quantity
/// ([Wikipedia](https://en.wikipedia.org/wiki/Speed)).
/// SI unit name is metre per second, unit symbol m/s.
#[derive(Copy, Clone, Default)]
pub struct Speed(pub Float);
basic_ops!(Speed);

#[allow(dead_code)]
impl Speed {
    /// Create an instance of type Speed from a float number in meter per second
    #[inline]
    pub fn from_m_s(value: Float) -> Self {
        Speed(value)
    }

    /// Create an instance of type Speed from a float number in kilometer per second
    #[inline]
    pub fn from_km_h(value: Float) -> Self {
        Speed(value * KILO / HOUR)
    }

    /// Create an instance of type Speed from a float number in knots
    #[inline]
    pub fn from_kt(value: Float) -> Self {
        Speed(value * NAUTICAL_MILE / HOUR)
    }

    /// Create an instance of type Speed from a float number in foot per minute
    #[inline]
    pub fn from_ft_min(value: Float) -> Self {
        Speed(value * FOOT / MINUTE)
    }

    /// Create an instance of type Speed from a float number in mile per hour
    #[inline]
    pub fn from_mph(value: Float) -> Self {
        Speed(value * MILE / HOUR)
    }

    #[inline]
    pub fn to_m_s(self) -> Float {
        self.0
    }
    #[inline]
    pub fn to_km_h(self) -> Float {
        self.0 * HOUR / KILO
    }
    #[inline]
    pub fn to_kt(self) -> Float {
        self.0 * HOUR / NAUTICAL_MILE
    }
    #[inline]
    pub fn to_ft_min(self) -> Float {
        self.0 * MINUTE / FOOT
    }
    #[inline]
    pub fn to_mph(self) -> Float {
        self.0 * HOUR / MILE
    }
}

/// Trait to convert data to the struct [Speed]
pub trait FloatToSpeed {
    /// Create an instance of type [Speed] from a number in kilometer per second
    fn km_h(self) -> Speed;
    /// Create an instance of type [Speed] from a number in meter per second
    fn m_s(self) -> Speed;
    /// Create an instance of type [Speed] from a number in knots
    fn kt(self) -> Speed;
    /// Create an instance of type [Speed] from a number in foot per second
    fn ft_min(self) -> Speed;
    /// Create an instance of type [Speed] from a number in mile per hour
    fn mph(self) -> Speed;
}

impl FloatToSpeed for Float {
    #[inline]
    fn km_h(self) -> Speed {
        Speed::from_km_h(self)
    }
    #[inline]
    fn m_s(self) -> Speed {
        Speed::from_m_s(self)
    }
    #[inline]
    fn kt(self) -> Speed {
        Speed::from_kt(self)
    }
    #[inline]
    fn ft_min(self) -> Speed {
        Speed::from_ft_min(self)
    }
    #[inline]
    fn mph(self) -> Speed {
        Speed::from_mph(self)
    }
}
