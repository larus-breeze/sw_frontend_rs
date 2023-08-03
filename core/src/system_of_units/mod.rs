mod area;
mod density;
mod mass;
mod pressure;
mod speed;

pub use area::{Area, FloatToArea};
pub use density::{Density, FloatToDensity};
pub use embedded_graphics::geometry::{Angle, AngleUnit};
pub use mass::{FloatToMass, Mass};
pub use speed::{FloatToSpeed, Speed};
pub use pressure::{FloatToPressure, Pressure};

pub type Float = f32;

// see https://en.wikipedia.org/wiki/Conversion_of_units

// length
pub(crate) const NAUTICAL_MILE: Float = 1852.0; // NM -> m
pub(crate) const FOOT: Float = 0.3048; // ft -> m
pub(crate) const MILE: Float = 1609.344; // mi -> m

// mass
pub(crate) const POUND: Float = 0.45359237; // lb -> kg

//
#[allow(dead_code)]
pub(crate) const MILLI: Float = 1e-3;
#[allow(dead_code)]
pub(crate) const CENTI: Float = 1e-2;
pub(crate) const KILO: Float = 1e3;

// time constants
#[allow(dead_code)]
pub(crate) const MINUTE: Float = 60.0; // min -> s
#[allow(dead_code)]
pub(crate) const HOUR: Float = 3600.0; // h -> s
#[allow(dead_code)]
pub(crate) const DAY: Float = 86400.0; // d -> s

// ISA 
#[allow(dead_code)]
pub(crate) const ISA_DENSITY_AT_NN: Float = 1.225; // density at NN in kg/m³
#[allow(dead_code)]
pub(crate) const ISA_PRESSURE_AT_NN: Float = 101335.0; // pressure at NN in Pa
