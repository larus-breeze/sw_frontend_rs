use core::{
    cmp::{Ordering, PartialEq},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{basic_ops, system_of_units::ISA_DENSITY_AT_NN, Float};

/// The density (more precisely, the volumetric mass density; also known as specific mass), of
/// a substance is its mass per unit volume. The symbol most often used for density is ρ (the
/// lower case Greek letter rho), although the Latin letter D can also be used. Mathematically,
/// density is defined as mass divided by volume.
/// ([Wikipedia](https://en.wikipedia.org/wiki/Ddensity)).
/// SI unit name is kilogram per cubic metre, unit symbol is kg/m³.
#[derive(Copy, Clone, Default)]
pub struct Density(pub Float);
basic_ops!(Density);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Density {
    pub const fn AT_NN() -> Self {
        Density(ISA_DENSITY_AT_NN)
    }

    /// Create an instance of type Density from a float number in kilogram per cubic meter
    #[inline]
    pub fn from_kg_m3(value: Float) -> Self {
        Density(value)
    }

    /// Create an instance of type Density from a float number in gram per cubic meter
    #[inline]
    pub fn from_g_m3(value: Float) -> Self {
        Density(value * 0.001)
    }

    /// Extract a float number in the unit kilogram per cubic meter
    #[inline]
    pub fn to_kg_m3(self) -> Float {
        self.0
    }

    /// Extract a float number in the unit gram per cubic meter
    #[inline]
    pub fn to_g_m3(self) -> Float {
        self.0 * 1000.0
    }
}

/// Trait to convert data to the struct [Density]
#[allow(non_snake_case)]
pub trait FloatToDensity {
    /// Create an instance of type [Density] from a number in kilogram per square meter
    fn kg_m3(self) -> Density;
    /// Create an instance of type [Density] from a number in gram per square meter
    fn g_m3(self) -> Density;
}

impl FloatToDensity for Float {
    #[inline]
    fn kg_m3(self) -> Density {
        Density::from_kg_m3(self)
    }
    #[inline]
    fn g_m3(self) -> Density {
        Density::from_g_m3(self)
    }
}
