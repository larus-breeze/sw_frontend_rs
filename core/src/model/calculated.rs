use crate::{
    controller::SoundParams,
    system_of_units::{FloatToLength, FloatToSpeed, Length, Speed},
    AirSpeed, CoreModel,
};

/// Metastructure for calculated or set values
#[derive(Copy, Clone)]
pub struct Calculated {
    pub circle_hysteresis: i8,
    pub speed_to_fly: AirSpeed,
    pub av_speed_to_fly: Speed, // ref. IAS
    pub speed_to_fly_dif: Speed,
    pub speed_to_fly_1s: Speed, // ref. IAS
    pub thermal_climb_rate: Speed,
    pub av2_climb_rate: Speed, // calculated by frontend
    pub circle_diameter: Length,
    pub circle_diameter_valid: bool,
    pub circle_max_min_last: Speed,
    pub circle_max_min_valid: bool,
    pub frequency: u16,
    pub continuous: bool,
    pub gain: i8,
    pub interpolated_climb_rate: InterpolatedClimbRate,
    pub sound_params: SoundParams,
}

impl Default for Calculated {
    #[allow(unused)]
    fn default() -> Self {
        Calculated {
            circle_hysteresis: 0,
            speed_to_fly: AirSpeed::from_tas_at_nn(100.0.km_h()),
            av_speed_to_fly: Speed::from_km_h(0.0),
            speed_to_fly_dif: 0.0.km_h(),
            speed_to_fly_1s: 0.0.km_h(),
            thermal_climb_rate: 0.0.m_s(),
            av2_climb_rate: 0.0.m_s(),
            circle_diameter: 0.0.m(),
            circle_diameter_valid: false,
            circle_max_min_last: 0.0.m_s(),
            circle_max_min_valid: false,
            frequency: 500,
            continuous: false,
            gain: 2,
            interpolated_climb_rate: InterpolatedClimbRate::default(),
            sound_params: SoundParams::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct InterpolatedClimbRate {
    pub fetch: bool,
    pub delta: Speed,
    pub value: Speed,
}

impl Default for InterpolatedClimbRate {
    fn default() -> Self {
        InterpolatedClimbRate {
            fetch: true,
            delta: 0.0.m_s(),
            value: 0.0.m_s(),
        }
    }
}

impl CoreModel {
    pub fn interpolate_climb_rate(&mut self) {
        let icr = &mut self.calculated.interpolated_climb_rate;
        icr.value += icr.delta;
        if icr.fetch {
            icr.delta = (self.sensor.climb_rate - icr.value) / 2.0;
        }
        icr.fetch = !icr.fetch;
    }
}
