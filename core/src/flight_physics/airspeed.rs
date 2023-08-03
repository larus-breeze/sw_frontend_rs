use crate::system_of_units::{Float, Speed};

#[cfg(feature = "f32")]
use crate::micromath::F32Ext;

/// Represents the speed with respect to air
///
/// Returns either the TAS or the IAS, which can then be used in km/h or other units.
#[derive(Copy, Clone, Default)]
pub struct AirSpeed {
    ias: Speed,
    tas: Speed,
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

    /// Creates an instance from speeds
    pub fn from_speeds(ias: Speed, tas: Speed) -> Self {
        AirSpeed { ias, tas }
    }

    /// Creates an instance from the TAS in the density height NN (IAS == TAS)
    pub fn from_tas_at_nn(tas: Speed) -> Self {
        AirSpeed { ias: tas, tas }
    }

    /// Returns the indicated airspeed
    pub fn ias(&self) -> Speed {
        self.ias
    }

    /// Returns the true airspeed
    pub fn tas(&self) -> Speed {
        self.tas
    }
}
