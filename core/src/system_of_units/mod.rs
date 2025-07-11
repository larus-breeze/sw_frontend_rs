mod acceleration;
mod angular_velocity;
mod area;
mod coord;
mod density;
mod length;
mod mass;
mod pressure;
mod speed;

use core::f32::consts::PI;

pub use acceleration::{Acceleration, FloatToAcceleration};
pub use angular_velocity::{AngularVelocity, FloatToAngularVelocity};
pub use area::{Area, FloatToArea};
pub use coord::{Coord, F64ToCoord, Latitude, Longitude};
pub use density::{Density, FloatToDensity};
pub use embedded_graphics::geometry::{Angle, AngleUnit};
pub use length::{FloatToLength, Length};
pub use mass::{FloatToMass, Mass};
pub use pressure::{FloatToPressure, Pressure};
pub use speed::{FloatToSpeed, Speed};

pub type Float = f32;

// see https://en.wikipedia.org/wiki/Conversion_of_units

// length
pub(crate) const NAUTICAL_MILE: Float = 1852.0; // NM -> m
pub(crate) const FOOT: Float = 0.3048; // ft -> m
pub(crate) const MILE: Float = 1609.344; // mi -> m
pub(crate) const INCH: Float = 0.0254; // inch -> m

// mass
pub(crate) const POUND: Float = 0.45359237; // lb -> kg

// acceleration
pub(crate) const STANDARD_GRAVITY: Float = 9.80665; // m/s²

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
pub(crate) const ISA_PRESSURE_AT_NN: Float = 101325.0; // pressure at NN in Pa

// RAD
#[allow(dead_code)]
pub(crate) const RAD_PER_DEGREE: Float = PI / 180.0;
#[allow(dead_code)]
pub(crate) const DEGREE_PER_RAD: Float = 180.0 / PI;

// Angle ranges
pub fn into_range_0_360(angle: Angle) -> Angle {
    let mut angle = angle.to_radians();
    while angle < 0.0 {
        angle += 2.0 * PI;
    }
    while angle > 2.0 * PI {
        angle -= 2.0 * PI
    }
    Angle::from_radians(angle)
}

// Angle ranges
pub fn into_range_180_180(angle: Angle) -> Angle {
    let mut angle = angle.to_radians();
    while angle < -PI {
        angle += 2.0 * PI;
    }
    while angle > PI {
        angle -= 2.0 * PI
    }
    Angle::from_radians(angle)
}
