use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{basic_ops, Float, CENTI, FOOT, INCH, KILO, MILE, MILLI, NAUTICAL_MILE};

/// Length is a measure of distance. In the International System of Quantities, length is a
/// quantity with dimension distance (see [Wikipedia](https://en.wikipedia.org/wiki/Length)).
/// SI unit name is metre, unit symbol is m.
#[derive(Copy, Clone, Default)]
pub struct Length(pub Float);
basic_ops!(Length);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Length {
    /// Create an instance of type Length from a float number in milimeters
    #[inline]
    pub fn from_mm(value: Float) -> Self {
        Length(value * MILLI)
    }

    /// Create an instance of type Length from a float number in centimeters
    #[inline]
    pub fn from_cm(value: Float) -> Self {
        Length(value * CENTI)
    }

    /// Create an instance of type Length from a float number in meters
    #[inline]
    pub fn from_m(value: Float) -> Self {
        Length(value)
    }

    /// Create an instance of type Length from a float number in kilometers
    #[inline]
    pub fn from_km(value: Float) -> Self {
        Length(value * KILO)
    }

    /// Extract a float number in the unit milimeter
    #[inline]
    pub fn to_mm(self) -> Float {
        (self.0 * 1.0 / MILLI).into()
    }

    /// Extract a float number in the unit centimeter
    #[inline]
    pub fn to_cm(self) -> Float {
        (self.0 * 1.0 / CENTI).into()
    }

    /// Extract a float number in the unit meter
    #[inline]
    pub fn to_m(self) -> Float {
        self.0.into()
    }

    /// Extract a float number in the unit kilometer
    #[inline]
    pub fn to_km(self) -> Float {
        (self.0 * 1.0 / KILO).into()
    }

    /// Create an instance of type Length from a float number in foot
    #[inline]
    pub fn from_ft(value: Float) -> Self {
        Length(value * FOOT)
    }

    /// Create an instance of type Length from a float number in inch
    #[inline]
    pub fn from_inch(value: Float) -> Self {
        Length(value * INCH)
    }

    /// Create an instance of type Length from a float number in nautical miles
    #[inline]
    pub fn from_NM(value: Float) -> Self {
        Length(value * NAUTICAL_MILE)
    }

    /// Create an instance of type Length from a float number in static miles
    #[inline]
    pub fn from_mi(value: Float) -> Self {
        Length(value * MILE)
    }

    /// Extract a float number in the unit foot
    #[inline]
    pub fn to_ft(self) -> Float {
        (self.0 * 1.0 / FOOT).into()
    }

    /// Extract a float number in the unit inch
    #[inline]
    pub fn to_inch(self) -> Float {
        (self.0 * 1.0 / INCH).into()
    }

    /// Extract a float number in the unit nautical mile
    #[inline]
    pub fn to_NM(self) -> Float {
        (self.0 * 1.0 / NAUTICAL_MILE).into()
    }

    /// Extract a float number in the unit statue mile
    #[inline]
    pub fn to_mi(self) -> Float {
        (self.0 * 1.0 / MILE).into()
    }
}

/// Trait to convert data to the struct [Length]
#[allow(non_snake_case)]
pub trait FloatToLength {
    /// Create an instance of type [Length] from a number in milimeter
    fn mm(self) -> Length;

    /// Create an instance of type [Length] from a number in centimeter
    fn cm(self) -> Length;

    /// Create an instance of type [Length] from a number in meter
    fn m(self) -> Length;

    /// Create an instance of type [Length] from a number in kilometer
    fn km(self) -> Length;

    /// Create an instance of type [Length] from a number in foot
    fn ft(self) -> Length;

    /// Create an instance of type [Length] from a number in inch
    fn inch(self) -> Length;

    /// Create an instance of type [Length] from a number in nautical mile
    fn NM(self) -> Length;

    /// Create an instance of type [Length] from a number in statue mile
    fn mi(self) -> Length;
}

impl FloatToLength for Float {
    #[inline]
    fn mm(self) -> Length {
        Length::from_mm(self)
    }
    #[inline]
    fn cm(self) -> Length {
        Length::from_cm(self)
    }
    #[inline]
    fn m(self) -> Length {
        Length::from_m(self)
    }
    #[inline]
    fn km(self) -> Length {
        Length::from_km(self)
    }
    #[inline]
    fn ft(self) -> Length {
        Length::from_ft(self)
    }
    #[inline]
    fn inch(self) -> Length {
        Length::from_inch(self)
    }
    #[inline]
    #[allow(non_snake_case)]
    fn NM(self) -> Length {
        Length::from_NM(self)
    }
    #[inline]
    fn mi(self) -> Length {
        Length::from_mi(self)
    }
}
