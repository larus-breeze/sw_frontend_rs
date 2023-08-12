use crate::system_of_units::{Angle, Speed};

/// Represents a wind vector
///
/// Returns either the speed or angle, which can then be used
#[derive(Copy, Clone)]
pub struct WindVector {
    speed: Speed,
    angle: Angle,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl WindVector {
    /// Creates an instance from floats
    #[inline]
    pub const fn new(speed: Speed, angle: Angle) -> Self {
        WindVector { speed, angle }
    }

    pub fn speed(&self) -> Speed {
        self.speed
    }

    pub fn angle(&self) -> Angle {
        self.angle
    }

    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    pub fn set_angle(&mut self, angle: Angle) {
        self.angle = angle;
    }
}

impl Default for WindVector {
    fn default() -> Self {
        WindVector {
            speed: Speed(0.0),
            angle: Angle::zero(),
        }
    }
}
