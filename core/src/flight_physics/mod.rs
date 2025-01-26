mod airspeed;
mod athmodphere;
mod polar;
pub(crate) mod polar_store;
pub(crate) mod polar_store_idx;
mod wind_vector;

pub use airspeed::*;
pub use athmodphere::*;
pub use polar::{GliderData, Polar};
pub use wind_vector::*;

/// Structure with the basic data of a sailplane
///
/// The contents are kept as natural Rust data types and not in the physical sizes. This data
/// structure is used as the basis for the polar. The contents correspond to the polar store
/// in XCSoar
#[derive(Copy, Clone)]
pub struct BasicGliderData {
    pub name: &'static str,
    pub wing_area: f32,        // m²
    pub max_speed: f32,        // km/h
    pub empty_mass: f32,       // km/h
    pub max_ballast: f32,      // kg
    pub reference_weight: f32, // kg
    pub handicap: u16,
    pub polar_values: [[f32; 2]; 3], // (km/h, m/s) * 3
}

impl Default for BasicGliderData {
    fn default() -> Self {
        BasicGliderData {
            name: "",
            wing_area: 0.0,
            max_speed: 0.0,
            empty_mass: 0.0,
            max_ballast: 0.0,
            reference_weight: 0.0,
            handicap: 0,
            polar_values: [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
        }
    }
}
