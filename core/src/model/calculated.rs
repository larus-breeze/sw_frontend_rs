use crate::{
    system_of_units::{FloatToSpeed, Speed},
    AirSpeed,
};

/// Metastructure for calculated or set values
#[derive(Copy, Clone)]
pub struct Calculated {
    pub speed_to_fly: AirSpeed,
    pub av_speed_to_fly: Speed, // ref. IAS
    pub speed_to_fly_dif: Speed,
    pub speed_to_fly_1s: Speed, // ref. IAS
    pub thermal_climb_rate: Speed,
    pub av2_climb_rate: Speed, // calculated by frontend
    pub frequency: u16,
    pub continuous: bool,
    pub gain: i8,
    pub av_supply_voltage: f32,
}

impl Default for Calculated {
    #[allow(unused)]
    fn default() -> Self {
        Calculated {
            speed_to_fly: AirSpeed::from_tas_at_nn(100.0.km_h()),
            av_speed_to_fly: Speed::from_km_h(0.0),
            speed_to_fly_dif: 0.0.km_h(),
            speed_to_fly_1s: 0.0.km_h(),
            thermal_climb_rate: 0.0.m_s(),
            av2_climb_rate: 0.0.m_s(),
            frequency: 500,
            continuous: false,
            gain: 2,
            av_supply_voltage: 12.0,
        }
    }
}
