use embedded_graphics::{
    geometry::{Angle, AngleUnit}
};

pub struct Blackboard {
    pub climb_rate: f32,
    pub average_climb_rate: f32,
    pub wind_angle: Angle,
    pub wind_speed: f32,
    pub average_wind_angle: Angle,
    pub average_wind_speed: f32,
    pub speed_to_fly_dif: f32,
    pub mc_cready: f32,
}

impl Blackboard {

    #[allow(unused)]
    pub fn new() -> Self {
        Blackboard {
            climb_rate: 1.7,
            average_climb_rate: 1.1,
            wind_angle: 66.0.deg(),
            wind_speed: 18.0,
            average_wind_angle: 80.0.deg(),
            average_wind_speed: 15.0,
            speed_to_fly_dif: 3.0,
            mc_cready: 0.7,
        
        }
    }
}