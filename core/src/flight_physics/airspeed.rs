use crate::system_of_units::{Float, Speed};

#[cfg(feature = "f32")]
use crate::micromath::F32Ext;


/// Represents the speed with respect to air
/// 
/// Returns either the TAS or the IAS, which can then be used in km/h or other units.
#[derive(Copy, Clone, Default)]
pub struct AirSpeed {
    pub ias: Speed,
    pub tas: Speed,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl AirSpeed {
    
    /// Creates an instance from floats
    #[inline]
    pub fn new(ias: Float, tas: Float) -> Self {
        AirSpeed {
            ias: Speed(ias),
            tas: Speed(tas),
        }
    }

    /// Creates an instance from the TAS in the density height NN (IAS == TAS)
    pub fn from_tas_at_nn(tas: Speed) -> Self {
        AirSpeed { ias: tas, tas }
    }
}
