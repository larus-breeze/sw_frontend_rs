use crate::{CoreModel, FloatToSpeed, AirSpeed, FloatToPressure, FloatToDensity};
use byteorder::{LittleEndian as LE, ByteOrder};
use embedded_graphics::prelude::AngleUnit;


#[rustfmt::skip]
#[allow(unused)]
mod sensor {
    pub const HEART_BEAT: u32 = 0x100;      // u32  version as 0x0102002a "1.02 Build 42"
    pub const EULER_ANGLES: u32 = 0x101;    // i16, i16, i16 roll nick yaw / 1/1000 rad
    pub const AIRSPEED: u32 = 0x102;        // u16, u16 TAS, IAS / km/h
    pub const VARIO: u32 = 0x103;           // i16, i16 vario, integrator / mm/s
    pub const GPS_DATE_TIME: u32 = 0x104;   // u8 year-2000, month, day, hour, mins, secs
    pub const GPS_LAT_LON: u32 = 0x105;     // i32 lat, lon / 10^-7 degrees
    pub const GPS_ALT: u32 = 0x106;         // i64 MSL altitude / mm
    pub const GPS_TRK_SPD: u32 = 0x107;     // i16 ground vector / 1/1000 rad, u16 groundspeed / km/h
    pub const WIND: u32 = 0x108;            // i16 1/1000 rad, u16 km/h    Current Wind
                                            // i16 1/1000 rad, u16 km/h    Average Wind
    pub const ATHMOSPHERE: u32 = 0x109;     // u16 pressure / Pa u16 density / g/m^3
    pub const GPS_SATS: u32 = 0x10a;        // u8 No of Sats
                                            // u8 Fix-Type NO=0 2D=1 3D=2 RTK=3
    pub const ACCELERATION: u32 = 0x10b;    // i16 representing TOTAL G-force in mm/s^2
                                            // acceleration counted positive downward relative to plane
                                            // stored as float m/s^2 counted positive upward
                                            // i16 representing NETTO G-force in mm/s^2
                                            // acceleration counted positive downward relative to plane
                                            // stored as float m/s^2 counted positive upward
                                            // i16 representing GPS vertical speed as seen from earth
                                            // in mm/s, stored as float m/s
                                            // u8, representing enum ( c_Gliding, c_Transition, c_Climbing )
    pub const TURN_COORD: u32 = 0x10c;      // float  in mm/s^2 , float in mm/s^2
    pub const SYSTEM_STATE: u32 = 0x10d;    // u32 Bitmuster
    pub const NOISE: u32 = 0x10e;           // ??   TODO
    pub const TEMPERATURE: u32 = 0x10f;     // i32  as float temp * 1000
    pub const HUMIDY: u32 = 0x110;          // u32 as float hum * 1000
    pub const PRESSURE: u32 = 0x111;        // u32 as float press * 1000
    pub const VDD: u32 = 0x112;             // unit16_t as float voltage * 10
    pub const TCS: u32 = 0x113;             // i16 as float sec * 10 tau for fast wind in cruise +
                                            // i16 as float sec * 10 tau for slow wind in cruise +
                                            // i16 as float sec * 10 tau for fast wind in climb +
                                            // i16 as float sec * 10 tau for slow wind in climb
    pub const SW_HYSTERESIS: u32 = 0x114;   // i16 as float sec * 10
    pub const EULER_SETUP: u32 = 0x115;     // i16 as float dec deg * 10 +  // Roll
                                            // i16 as float dec deg * 10 +  // Nick
                                            // i16 as float dec deg * 10    // Yaw
    pub const DEC_INCLINATION: u32 = 0x116; // i16 as float dec deg * 10 +  // Declination
                                            // i16 as float dec deg * 10    // Inclination
    pub const IAS_OFFSET: u32 = 0x117;      // i16 as float km/h * 10}
}

#[rustfmt::skip]
#[allow(unused)]
mod audio {
    pub const HEART_BEAT: u32 = 0x200;      // u32  version as 0x0102002a "1.02 Build 42"
    pub const CMD_2_XCSOAR: u32 = 0x201;    // u8 command for XCSoar
                                            // = 0 Unforce XCSoar CLIMB-CRUISE
                                            // = 1 Force XCSoar to CLIMB
                                            // = 2 Force XCSoar to CRUISE
                                            // = 3 Unforce XCSoar WINDUP
                                            // = 4 Force XCSoar to WINDUP
    pub const NOISE: u32 = 0x202;           // ??   TODO
    pub const TEMPERATURE: u32 = 0x203;     // i32  as float temp * 1000
    pub const HUMIDY: u32 = 0x204;          // u32 as float hum * 1000
    pub const PRESSURE: u32 = 0x205;        // u32 as float press * 1000
    pub const FLAPS_DATA: u32 = 0x206;      // u16 position [percent * 100]
                                            // + u8 switch pattern [0b0000-0b1111]*/
}

#[rustfmt::skip]
#[allow(unused)]
mod ad57 {
    pub const HEART_BEAT: u32 = 0x300;      // u32  version as 0x0102002a "1.02 Build 42"
    pub const CMD_2_XCSOAR: u32 = 0x301;    // u8 command for XCSoar
                                            // = 0 Unforce XCSoar CLIMB-CRUISE
                                            // = 1 Force XCSoar to CLIMB
                                            // = 2 Force XCSoar to CRUISE
                                            // = 3 Unforce XCSoar WINDUP
                                            // = 4 Force XCSoar to WINDUP
    pub const NOISE: u32 = 0x302;           // ??   TODO
    pub const TEMPERATURE: u32 = 0x303;     // i32  as float temp * 1000
    pub const HUMIDY: u32 = 0x304;          // u32 as float hum * 1000
    pub const PRESSURE: u32 = 0x305;        // u32 as float press * 1000
    pub const VDD: u32 = 0x306;             // unit16_t as float voltage * 10
    pub const TCS: u32 = 0x307;             // i16 as float sec * 10 tau for fast wind in cruise +
                                            // i16 as float sec * 10 tau for slow wind in cruise +
                                            // i16 as float sec * 10 tau for fast wind in climb +
                                            // i16 as float sec * 10 tau for slow wind in climb
    pub const SW_HYSTERESIS: u32 = 0x308;   // i16 as float sec * 10
    pub const EULER_SETUP: u32 = 0x309;     // i16 as float dec deg * 10 +  // Roll
                                            // i16 as float dec deg * 10 +  // Nick
                                            // i16 as float dec deg * 10    // Yaw
    pub const DEC_INCLINATION: u32 = 0x30a; // i16 as float dec deg * 10 +  // Declination
                                            // i16 as float dec deg * 10    // Inclination
    pub const IAS_OFFSET: u32 = 0x30b;      // i16 as float km/h * 10
    pub const SIGNAL: u32 = 0x310;          // u8 signal_id +
                                            // u8 signal_volume
    pub const AUDIO: u32 = 0x311;           // i16  audio_frequency +
                                            // u16 interval +
                                            // u8  audio-volume +
                                            // u8  duty cycle
                                            // u8  climb-mode
    pub const FLAPS_STATUS: u32 = 0x312;    // u8  0/1 on/off-switch
                                            // u8  CurrentFlapsSetting
                                            // u8  OptimalFlapsSetting
                                            // u8  FlapsFlashControl
                                            // u8  LEDDutyCycle in %
    pub const REBOOT: u32 = 0x313;          // empty package, just a trigger
    pub const MC_CREADY: u32 = 0x320;       // u8 McCready value / 10cm/s, u8 audio volume
    pub const HAVE_CONTROL: u32 = 0x321;    // empty package, just a trigger
}

pub fn read_can_frame(core_model: &mut CoreModel, id: u32, data: &[u8;8]) {
    match id {
        sensor::AIRSPEED => {
            let tas = (LE::read_u16(&data[..2]) as f32).km_h();
            let ias = (LE::read_u16(&data[2..4]) as f32).km_h();
            core_model.sensor.airspeed = AirSpeed::from_speeds(ias, tas);
        },
        sensor::VARIO => {
            core_model.sensor.climb_rate = (LE::read_i16(&data[..2]) as f32 * 0.001).m_s();
            core_model.sensor.average_climb_rate = (LE::read_i16(&data[2..4]) as f32 * 0.001).m_s();
        },
        sensor::WIND => {
            core_model.sensor.wind_angle = (LE::read_i16(&data[..2]) as f32 * 0.001).rad();
            core_model.sensor.wind_speed = (LE::read_u16(&data[2..4]) as f32).km_h();
            core_model.sensor.average_wind_angle = (LE::read_i16(&data[4..6]) as f32 * 0.001).rad();
            core_model.sensor.average_wind_speed = (LE::read_u16(&data[6..8]) as f32).km_h();
        },
        sensor::ATHMOSPHERE => {
            core_model.sensor.pressure = (LE::read_u16(&data[..2]) as f32).n_m2();
            core_model.sensor.density = (LE::read_u16(&data[2..4]) as f32).g_m3();
        },
        _ => (), // all other frames are ignored
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
