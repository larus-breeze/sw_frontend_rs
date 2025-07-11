use num_enum::FromPrimitive;

use crate::PersistenceId;

// Todo include configuration of center frequency

/// Definition of special ids (Object Id 4 Vario Display)
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum SpecialId {
    Sound = 0,
    VoltTemp = 1,
    AvgClimbRates = 2,
    #[default]
    Ignore,
}

/// Definition of generic ids
#[derive(FromPrimitive)]
#[repr(u16)]
pub enum GenericId {
    Heartbeat = 0,
    HwFwVersion = 1,
    SetSysSetting = 2,
    BinaryTransfer = 3,
    #[default]
    Ignore = 4,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum RemoteConfig {
    Get,
    Set,
}

#[allow(dead_code)]
#[derive(FromPrimitive)]
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum CanConfigId {
    Volume = 0,
    MacCready = 1,
    WaterBallast = 2,
    Bugs = 3,
    Qnh = 4,
    PilotWeight = 5,
    VarioModeControl = 6,
    TcClimbRate = 7,
    TcSpeedToFly = 8,
    VarioMode = 9,
    #[default]
    Ignore = 10,

    SensTiltRoll = 0x2000,
    SensTiltPitch = 0x2001,
    SensTiltYaw = 0x2002,
    PitotOffset = 0x2003,
    PitotSpan = 0x2004,
    QnhDelta = 0x2005,
    MagAutoCalib = 0x2006,
    VarioTc = 0x2007,
    VarioIntTc = 0x2008,
    WindTc = 0x2009,
    MeanWindTc = 0x200a,
    GnssConfig = 0x200b,
    AntBaselen = 0x200c,
    AntSlaveDown = 0x200d,
    AntSlaveRight = 0x200e,
    VarioPressTc = 0x200f,

    CmdMeasure1 = 0x3000,
    CmdMeasure2 = 0x3001,
    CmdMeasure3 = 0x3002,
    CmdCalcSensorOrientation = 0x3003,
    CmdFineTuneCalibration = 0x3004,
    CmdReset = 0x3005,
}

impl From<PersistenceId> for CanConfigId {
    fn from(value: PersistenceId) -> Self {
        match value {
            PersistenceId::Volume => CanConfigId::Volume,
            PersistenceId::McCready => CanConfigId::MacCready,
            PersistenceId::WaterBallast => CanConfigId::WaterBallast,
            PersistenceId::Bugs => CanConfigId::Bugs,
            PersistenceId::Qnh => CanConfigId::Qnh,
            PersistenceId::PilotWeight => CanConfigId::PilotWeight,
            PersistenceId::VarioModeControl => CanConfigId::Ignore,
            PersistenceId::TcClimbRate => CanConfigId::TcClimbRate,
            PersistenceId::TcSpeedToFly => CanConfigId::TcSpeedToFly,
            PersistenceId::VarioMode => CanConfigId::VarioMode,
            _ => CanConfigId::Ignore,
        }
    }
}

#[rustfmt::skip]
#[allow(unused)]
pub mod object_id {
    pub const ARBITRATION: u16 = 0;
    pub const CONFIG: u16 = 1;
    pub const SENSOR: u16 = 2;
    pub const GPS: u16 = 3;
    pub const FRONTEND: u16 = 4;
}

#[rustfmt::skip]
#[allow(unused)]
pub mod sensor {
    pub const EULER_ROLL_NICK: u16 = 0;     // f32 roll, f32 nick
    pub const EULER_YAW_TURN_RATE: u16 = 1; // f32 yaw, f32 turn rate
    pub const TAS_IAS: u16 = 2;             // f32 TAS, f32 IAS
    pub const VARIO_AV_VARIO: u16 = 3;      // f32 vario, f32 av_vario
    pub const WIND_DIR_SPEED: u16 = 4;      // f32 wind_direction, f32 wind_speed
    pub const AV_WIND_DIR_SPEED: u16 = 5;   // f32 av_wind_directin, f32 av_wind_speed
    pub const AMB_PRESS_AIR_DENS: u16 = 6;  // f32 ambient_pressure, f32 air_density,
    pub const G_FORCE_VERTICAL_GF: u16 = 7; // f32 g_force, f32 vertical_g_force
    pub const SLIP_PITCH_ANGLE: u16 = 8;    // f32 slip_angle, f32 pitch_angle
    pub const UBATT_CIRCLE_MODE: u16 = 9;   // f32 supply_voltage, u8 circle_mode
    pub const SYSTEM_STATE_GIT_TAG: u16 = 0x0a; // u32 system_state, git_tag
    pub const CONFIG_VALUE: u16 = 0x0f;     // u32 config_id, f32 value
}

#[rustfmt::skip]
#[allow(unused)]
pub mod gps {
    pub const DATE_TIME: u16 = 0;           // u16 year, u8 month, u8 day, u8 h, u8 min, u8 s
    pub const LATITUDE: u16 = 1;            // f64 latitude
    pub const LONGITUDE: u16 = 2;           // f64 longitude
    pub const ALTITUDE_GEO_SEP: u16 = 3;    // f32 altitude, f32 geo_seperation
    pub const GROUND_TRACK_SPEED: u16 = 4;  // f32 ground_track, f32 ground_speed
    pub const NO_SAT_FIX_TYPE: u16 = 5;     // u8 no sats, u8 sat fix type
}

#[rustfmt::skip]
#[allow(unused)]
pub mod sensor_legacy {
    pub const EULER_ANGLES: u16 = 0x101;    // i16, i16, i16 roll nick yaw / 1/1000 rad
    pub const AIRSPEED: u16 = 0x102;        // i16, i16 TAS, IAS / km/h
    pub const VARIO: u16 = 0x103;           // i16, i16 vario, integrator / mm/s
    pub const GPS_DATE_TIME: u16 = 0x104;   // 6 x u8 year-2000, month, day, hour, mins, secs
    pub const GPS_LAT_LON: u16 = 0x105;     // i32 lat, lon / 10^-7 degrees
    pub const GPS_ALT: u16 = 0x106;         // i32 MSL altitude / mm, i32 geo separation in 1/10 m
    pub const GPS_TRK_SPD: u16 = 0x107;     // i16 ground vector / 1/1000 rad, u16 groundspeed / km/h
    pub const WIND: u16 = 0x108;            // Current Wind i16 1/1000 rad, i16 km/h
                                            // Average Wind i16 1/1000 rad, i16 km/h
    pub const ATHMOSPHERE: u16 = 0x109;     // u32 pressure / Pa, u32 density / g/m^3
    pub const GPS_SATS: u16 = 0x10a;        // u8 No of Sats
                                            // u8 Fix-Type NO=0 2D=1 3D=2 RTK=3
    pub const ACCELERATION: u16 = 0x10b;    // i16  G-force in mm/s^2
                                            // i16 vertical G-force in mm/s^2
                                            // i16 GPS vertical speed  in mm/s
                                            // u8, enum (0 Straight Flight, 1 Transition, 2 Circling)
    pub const TURN_COORD: u16 = 0x10c;      // i16 slip angle 0.001 rad
                                            // i16 turn rate 0.001 rad/s
                                            // i16 nick angle 0.001 rad
    pub const SYSTEM_STATE: u16 = 0x10d;    // u32 system_state, u32 git_tag dec
    pub const VDD: u16 = 0x112;             // u16 voltage * 10
}

#[rustfmt::skip]
#[allow(unused)]
pub mod audio_legacy {
    pub const HEART_BEAT: u16 = 0x200;      // u32  version as 0x0102002a "1.02 Build 42"
    pub const CMD_2_XCSOAR: u16 = 0x201;    // u8 command for XCSoar
                                            // = 0 Unforce XCSoar CLIMB-CRUISE
                                            // = 1 Force XCSoar to CLIMB
                                            // = 2 Force XCSoar to CRUISE
                                            // = 3 Unforce XCSoar WINDUP
                                            // = 4 Force XCSoar to WINDUP
    pub const NOISE: u16 = 0x202;           // ??   TODO
    pub const TEMPERATURE: u16 = 0x203;     // i32  as float temp * 1000
    pub const HUMIDY: u16 = 0x204;          // u32 as float hum * 1000
    pub const PRESSURE: u16 = 0x205;        // u32 as float press * 1000
    pub const FLAPS_DATA: u16 = 0x206;      // u16 position [percent * 100]
                                            // + u8 switch pattern [0b0000-0b1111]*/
}

#[rustfmt::skip]
#[allow(unused)]
pub mod frontend_masster {
    pub const AVG_CLIMB_RATES: u16 = 0x282; // Climb rates fromt the virtual master device
}

#[rustfmt::skip]
#[allow(unused)]
pub mod frontend_legacy {
    pub const HEART_BEAT: u16 = 0x300;      // u32  version as 0x0102002a "1.02 Build 42"
    pub const CMD_2_XCSOAR: u16 = 0x301;    // u8 command for XCSoar
                                            // = 0 Unforce XCSoar CLIMB-CRUISE
                                            // = 1 Force XCSoar to CLIMB
                                            // = 2 Force XCSoar to CRUISE
                                            // = 3 Unforce XCSoar WINDUP
                                            // = 4 Force XCSoar to WINDUP
    pub const NOISE: u16 = 0x302;           // ??   TODO
    pub const TEMPERATURE: u16 = 0x303;     // i32  as float temp * 1000
    pub const HUMIDY: u16 = 0x304;          // u32 as float hum * 1000
    pub const PRESSURE: u16 = 0x305;        // u32 as float press * 1000
    pub const VDD: u16 = 0x306;             // unit16_t as float voltage * 10
    pub const TCS: u16 = 0x307;             // i16 as float sec * 10 tau for fast wind in cruise +
                                            // i16 as float sec * 10 tau for slow wind in cruise +
                                            // i16 as float sec * 10 tau for fast wind in climb +
                                            // i16 as float sec * 10 tau for slow wind in climb
    pub const SW_HYSTERESIS: u16 = 0x308;   // i16 as float sec * 10
    pub const EULER_SETUP: u16 = 0x309;     // i16 as float dec deg * 10 +  // Roll
                                            // i16 as float dec deg * 10 +  // Nick
                                            // i16 as float dec deg * 10    // Yaw
    pub const DEC_INCLINATION: u16 = 0x30a; // i16 as float dec deg * 10 +  // Declination
                                            // i16 as float dec deg * 10    // Inclination
    pub const IAS_OFFSET: u16 = 0x30b;      // i16 as float km/h * 10
    pub const SIGNAL: u16 = 0x310;          // u8 signal_id +
                                            // u8 signal_volume
    pub const AUDIO: u16 = 0x311;           // i16  audio_frequency +
                                            // u16 interval +
                                            // u8  audio-volume +
                                            // u8  duty cycle
                                            // u8  climb-mode
    pub const FLAPS_STATUS: u16 = 0x312;    // u8  0/1 on/off-switch
                                            // u8  CurrentFlapsSetting
                                            // u8  OptimalFlapsSetting
                                            // u8  FlapsFlashControl
                                            // u8  LEDDutyCycle in %
    pub const REBOOT: u16 = 0x313;          // empty package, just a trigger
    pub const MC_CREADY: u16 = 0x320;       // u8 McCready value / 10cm/s, u8 audio volume
    pub const HAVE_CONTROL: u16 = 0x321;    // empty package, just a trigger
    pub const NOTHING: u16 = 0x3ff;         // just a placeholder for a hw filter, no content
}

pub enum CanActive {
    None = 0x00,
    SensorboxLegacy = 0x01,
}
