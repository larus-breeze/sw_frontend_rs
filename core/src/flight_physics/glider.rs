/// Structure with the basic data of a sailplane
/// 
/// The contents are kept as natural Rust data types and not in the physical sizes. This data 
/// structure is used as the basis for the polar. The contents correspond to the polar store 
/// in XCSoar
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

