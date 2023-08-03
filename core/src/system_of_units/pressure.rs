use core::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
    cmp::{PartialEq, Ordering}
};

use crate::{
    basic_ops, Float, ISA_PRESSURE_AT_NN, 
};

/// Pressure (symbol: p or P) is the force applied perpendicular to the surface 
/// of an object per unit area over which that force is distributed. Gauge 
/// pressure (also spelled gage pressure)[a] is the pressure relative to the 
/// ambient pressure.
/// 
/// ([Wikipedia](https://en.wikipedia.org/wiki/Pressure)).
/// 
/// SI unit name is kilogram per meter and second², unit symbol is kg/(ms²).
#[derive(Copy, Clone, Default)]
pub struct Pressure(pub Float);
basic_ops!(Pressure);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Pressure {
    pub const fn AT_NN() -> Self {Pressure(ISA_PRESSURE_AT_NN)}

    /// Create an instance of type Pressure from a float number in kilogram per meter and second²
    #[inline]
    pub fn from_kg_ms2(value: Float) -> Self { Pressure(value) }

    /// Create an instance of type Pressure from a float number in newton per square meter
    #[inline]
    pub fn from_n_m2(value: Float) -> Self { Pressure(value) }

    /// Extract a float number in the unit kilogram per meter and second²
    #[inline]
    pub fn to_kg_ms2(self) -> Float { self.0.into() }

    /// Extract a float number in the unit newton per cubic meter
    #[inline]
    pub fn to_n_ms(self) -> Float { self.0.into() }
}

/// Trait to convert data to the struct [Pressure]
#[allow(non_snake_case)]
pub trait FloatToPressure {
    /// Create an instance of type [Pressure] from a number in kilogram per meter and second²
    fn kg_ms2(self) -> Pressure;

    /// Create an instance of type Pressure from a float number in newton per square meter
    fn n_m2(self) -> Pressure;
}

impl FloatToPressure for Float {
    #[inline]
    fn kg_ms2(self) -> Pressure { Pressure::from_kg_ms2(self)}
    #[inline]
    fn n_m2(self) -> Pressure { Pressure::from_n_m2(self)}
}
