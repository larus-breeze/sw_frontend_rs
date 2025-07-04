use crate::system_of_units::{Angle, Speed, FloatToSpeed};

#[allow(unused)]
use crate::{assert_float_eq, system_of_units::AngleUnit};

use core::{ops::Sub, f32::consts::PI};

#[allow(unused)]
use micromath::F32Ext;

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

impl Sub for WindVector {
    type Output = WindVector;
    fn sub(self, rhs: Self) -> Self::Output {
        fn into_range(mut angle: f32) -> f32 {
            while angle < 0.0 {
                angle += 2.0 * PI;
            }
            while angle > 2.0 * PI {
                angle -= 2.0 * PI
            }
            angle
        }

        let gamma = into_range((rhs.angle() - self.angle()).to_radians());
        //println!("gamma {} self {} rhs {}", gamma*180.0/PI, self.angle().to_degrees(), rhs.angle().to_degrees());
        let a = self.speed().to_m_s();
        let b = rhs.speed().to_m_s();
        let c = (a*a + b*b - 2.0*a*b*gamma.cos()).sqrt();
        let alpha = ((b*b + c*c - a*a)/(2.0*b*c)).acos();

        if c.is_finite() && alpha.is_finite() {
            let zeta = into_range(
                if gamma > PI {
                    self.angle().to_radians() + gamma - alpha
                } else {
                    self.angle().to_radians() + gamma + alpha
                }
            );
            WindVector::new(c.m_s(), Angle::from_radians(zeta))
        } else {
            WindVector::new(0.0_f32.m_s(), Angle::from_radians(0.0))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: &[(f32, f32, f32, f32, f32, f32)] = &[
        (6.0, 0.000, 6.0, 0.000, 0.000, 0.000), 
        (6.0, 90.000, 6.0, 0.000, 8.485, 315.000), 
        (6.0, 180.000, 6.0, 0.000, 12.000, 360.000), 
        (6.0, 270.000, 6.0, 0.000, 8.485, 45.000), 
        (6.0, 10.000, 6.0, 0.000, 1.046, 275.000), 
        (6.0, 190.000, 6.0, 0.000, 11.954, 5.000), 
        (6.0, 350.000, 6.0, 0.000, 1.046, 85.000), 
        (6.0, 0.000, 6.0, 0.000, 0.000, 0.000), 
        (6.0, 0.000, 6.0, 90.000, 8.485, 135.000), 
        (6.0, 0.000, 6.0, 180.000, 12.000, 180.000), 
        (6.0, 0.000, 6.0, 270.000, 8.485, 225.000), 
        (6.0, 0.000, 6.0, 10.000, 1.046, 95.000), 
        (6.0, 0.000, 6.0, 190.000, 11.954, 185.000), 
        (6.0, 0.000, 6.0, 350.000, 1.046, 265.000), 
        (6.0, 350.000, 6.0, 350.000, 0.000, 0.000), 
        (6.0, 350.000, 6.0, 80.000, 8.485, 125.000), 
        (6.0, 350.000, 6.0, 170.000, 12.000, 170.000), 
        (6.0, 350.000, 6.0, 260.000, 8.485, 215.000), 
        (6.0, 350.000, 6.0, 0.000, 1.046, 85.000), 
        (6.0, 350.000, 6.0, 180.000, 11.954, 175.000), 
        (6.0, 350.000, 6.0, 340.000, 1.046, 255.000),     
    ];

    #[test]
    fn sub_windvector() {
        for (a, alpha, b, beta, r_speed, r_angle) in TEST_CASES {
            let v1 = WindVector::new(a.m_s(), alpha.deg());
            let v2 = WindVector::new(b.m_s(), beta.deg());
            let v3 = v1 - v2;

            println!("Speed {} == {}", v3.speed().to_m_s(), *r_speed);
            assert_float_eq!(v3.speed().to_m_s(), *r_speed);
            println!("Angle {} == {}", v3.angle().to_degrees(), *r_angle);
            assert_float_eq!(v3.angle().to_degrees(), *r_angle);
        }
    }
}