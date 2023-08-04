use crate::system_of_units::{Speed, Angle};

/// Represents a wind vector
///
/// Returns either the speed or angle, which can then be used
#[derive(Copy, Clone)]
pub struct Wind {
    speed: Speed,
    angle: Angle,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Wind {
    /// Creates an instance from floats
    #[inline]
    pub const fn new(speed: Speed, angle: Angle) -> Self {
        Wind {speed, angle}
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

impl Default for Wind {
    fn default() -> Self {
        Wind{speed: Speed(0.0), angle: Angle::zero()}
    }
}