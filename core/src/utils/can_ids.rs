use core::mem::transmute;

/// Definition of special ids (Object Id 4 Vario Display)
pub enum SpecialId {
    Sound = 0,
    SpeedToFly = 1,
    Ignore = 2,
}

impl From<u16> for SpecialId {
    fn from(value: u16) -> Self {
        if value > SpecialId::Ignore as u16 {
            SpecialId::Ignore
        } else {
            // Saftey: only valid values are transmuted
            unsafe { transmute::<u8, SpecialId>(value as u8) }
        }
    }
}

/// Definition of generic ids
pub enum GenericId {
    Heartbeat = 0,
    HwFwVersion = 1,
    SetSysSetting = 2,
    BinaryTransfer = 3,
    Ignore = 4,
}

impl From<u16> for GenericId {
    fn from(value: u16) -> Self {
        if value > GenericId::Ignore as u16 {
            GenericId::Ignore
        } else {
            // Saftey: only valid values are transmuted
            unsafe { transmute::<u8, GenericId>(value as u8) }
        }
    }
}

/// Definition of changeable values in SetSysSetting
#[repr(u16)]
pub enum SysConfigId {
    VolumeVario = 0,
    MacCready = 1,
    WaterBallast = 2,
    Bugs = 3,
    Qnh = 4,
    PilotWeight = 5,
    Ignore = 6,
}

impl From<u16> for SysConfigId {
    fn from(value: u16) -> Self {
        if value > SysConfigId::Ignore as u16 {
            SysConfigId::Ignore
        } else {
            // unsafe: values lower than ::Ignore are ok
            unsafe { transmute::<u16, SysConfigId>(value) }
        }
    }
}
pub enum SysValueId {
    U8(u8),
    F32(f32),
}

#[rustfmt::skip]
#[allow(unused)]
pub mod sensor {
    pub const EULER_ANGLES: u16 = 0x101;    // i16, i16, i16 roll nick yaw / 1/1000 rad
    pub const AIRSPEED: u16 = 0x102;        // i16, i16 TAS, IAS / km/h
    pub const VARIO: u16 = 0x103;           // i16, i16 vario, integrator / mm/s
    pub const GPS_DATE_TIME: u16 = 0x104;   // 6 x u8 year-2000, month, day, hour, mins, secs
    pub const GPS_LAT_LON: u16 = 0x105;     // i32 lat, lon / 10^-7 degrees
    pub const GPS_ALT: u16 = 0x106;         // i32 MSL altitude / mm, i32 geo separation in 1/10 m
    pub const GPS_TRK_SPD: u16 = 0x107;     // i32 ground vector / 1/1000 rad, u16 groundspeed / km/h
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
pub mod audio {
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
pub mod frontend {
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
