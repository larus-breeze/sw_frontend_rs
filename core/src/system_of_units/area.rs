use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::system_of_units::FOOT;
use crate::{basic_ops, Float};

/// Area is the quantity that expresses the extent of a two-dimensional region, shape, or planar
/// lamina (see [Wikipedia](https://en.wikipedia.org/wiki/Area)).
/// SI unit name is square meter, unit symbol is m².
#[derive(Copy, Clone, Default)]
pub struct Area(pub Float);
basic_ops!(Area);

#[allow(dead_code)]
impl Area {
    /// Create an instance of type Area from a float number in square meter
    #[inline]
    pub fn from_m2(value: Float) -> Self {
        Area(value)
    }

    /// Create an instance of type Area from a float number in square foot
    #[inline]
    pub fn from_sqft(value: Float) -> Self {
        Area(value * FOOT * FOOT)
    }

    /// Extract a float number in the unit square meter
    #[inline]
    pub fn to_m2(self) -> Float {
        self.0
    }

    /// Extract a float number in the unit square foot
    #[inline]
    pub fn to_sqft(self) -> Float {
        self.0 * 1.0 / (FOOT * FOOT)
    }
}

/// Trait to convert data to the struct [Area]
pub trait FloatToArea {
    /// Create an instance of type [Area] from a number in square meter
    fn m2(self) -> Area;
    /// Create an instance of type [Area] from a number in square foot
    fn sqft(self) -> Area;
}

impl FloatToArea for Float {
    #[inline]
    fn m2(self) -> Area {
        Area::from_m2(self)
    }
    #[inline]
    fn sqft(self) -> Area {
        Area::from_sqft(self)
    }
}
