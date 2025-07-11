use crate::{
    flight_physics::{PressureAltitude, WindVector},
    system_of_units::{
        Acceleration, AngularVelocity, FloatToAcceleration, FloatToAngularVelocity, FloatToLength,
        FloatToSpeed, Length, Pressure, Speed,
    },
    AirSpeed, DateTime, Density, F64ToCoord, Latitude, Longitude,
};
use embedded_graphics::geometry::{Angle, AngleUnit};

/// Enum for GPS state
#[derive(Clone, Copy, PartialEq)]
pub enum GpsState {
    NoGps,
    PosAvail,
    HeadingAvail,
}

/// Sensor Values
///
/// This structure contains all variables that are generated by the Larus sensor box.
#[derive(Clone, Copy)]
pub struct Sensor {
    pub airspeed: AirSpeed,
    pub average_climb_rate: Speed,
    pub average_wind: WindVector,
    pub climb_rate: Speed,
    pub density: Density,
    pub euler_roll: Angle,
    pub euler_pitch: Angle,
    pub euler_yaw: Angle,
    pub g_force: Acceleration,
    pub gps_altitude: Length,
    pub gps_date_time: DateTime,
    pub gps_geo_seperation: Length,
    pub gps_lat: Latitude,
    pub gps_lon: Longitude,
    pub gps_track: Angle,
    pub gps_ground_speed: Speed,
    pub gps_sats: u8,
    pub gps_state: GpsState,
    pub nick_angle: Angle,
    pub pressure: Pressure,
    pub pressure_altitude: PressureAltitude,
    pub slip_angle: Angle,
    pub turn_rate: AngularVelocity,
    pub vertical_g_force: Acceleration,
    pub wind_vector: WindVector,
    pub horizon_availaable: bool,
    pub gnss_and_compass_ok: bool,
}

impl Default for Sensor {
    #[allow(unused)]
    fn default() -> Self {
        Sensor {
            airspeed: AirSpeed::from_tas_at_nn(0.0.km_h()),
            average_climb_rate: 0.0.m_s(),
            climb_rate: 0.0.m_s(),
            density: Density::AT_NN(),
            euler_roll: 0.0_f32.deg(),
            euler_pitch: 0.0_f32.deg(),
            euler_yaw: 0.0_f32.deg(),
            g_force: 9.81.m_s2(),
            gps_altitude: 0.0.m(),
            gps_date_time: DateTime::new(),
            gps_geo_seperation: 0.0.m(),
            gps_lat: Latitude(0.0_f64.deg()),
            gps_lon: Longitude(0.0_f64.deg()),
            gps_track: 0.0_f32.deg(),
            gps_ground_speed: 0.0.m_s(),
            gps_sats: 0,
            gps_state: GpsState::NoGps,
            nick_angle: 0.0_f32.deg(),
            pressure: Pressure::AT_NN(),
            pressure_altitude: PressureAltitude::default(),
            slip_angle: 0.0_f32.deg(),
            turn_rate: 0.0.rad_s(),
            vertical_g_force: 9.81.m_s2(),
            average_wind: WindVector::new(0.0.km_h(), 0.0_f32.deg()),
            wind_vector: WindVector::new(0.0.km_h(), 0.0_f32.deg()),
            horizon_availaable: true,
            gnss_and_compass_ok: false,
        }
    }
}
