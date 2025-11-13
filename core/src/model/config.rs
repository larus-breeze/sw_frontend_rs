use num_enum::FromPrimitive;
use core::convert::From;
use crate::{
    Palette, system_of_units::{FloatToSpeed, Speed}, view::viewable::{centerview::CenterView, vario_infoview::{Info3View, LineView}}
};

/// Possible displays
#[derive(Clone, Copy, PartialEq, FromPrimitive, Debug)]
#[repr(u8)]
pub enum DisplayActive {
    #[default]
    Vario,
    Horizon,
    DeviceInfo,
    Menu,
    FirmwareUpdate,
}

pub const DEVICE_INFO: &str = "Device Info";
pub const HORIZON: &str = "Horizon";
#[allow(unused)]
pub const VARIO: &str = "Vario";
pub const FIRMWARE_UPDATE: &str = "Firmware Update";
pub const MENU: &str = "Menu";

impl From<&str> for DisplayActive {
    fn from(value: &str) -> Self {
        match value {
            HORIZON => DisplayActive::Horizon,
            DEVICE_INFO => DisplayActive::DeviceInfo,
            FIRMWARE_UPDATE => DisplayActive::FirmwareUpdate,
            MENU => DisplayActive::Menu,
            _ => DisplayActive::Vario,
        }
    }
}

#[derive(Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum DisplayTheme {
    #[default]
    Dark,
    Bright,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TypeOfInfo {
    None,
    WaterBallast,
    GearAlarm,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OverlayActive {
    None,
    Editor,
    Menu,
    Info,
}

#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
#[repr(u8)]
pub enum UnitHorizontalSpeed {
    #[default]
    Kmph,
    Mph,
    Knots,
}

pub const UNIT_KMPH: &'static str = "km/h";
pub const UNIT_KNOTS: &'static str = "knots";
pub const UNIT_MPH: &'static str = "mph";

impl UnitHorizontalSpeed {
    pub fn from_str(name: &str) -> Self {
        match name {
            UNIT_KNOTS => UnitHorizontalSpeed::Knots,
            UNIT_MPH => UnitHorizontalSpeed::Mph,
            _ => UnitHorizontalSpeed::Kmph,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UnitHorizontalSpeed::Kmph => UNIT_KMPH,
            UnitHorizontalSpeed::Knots => UNIT_KNOTS,
            UnitHorizontalSpeed::Mph => UNIT_MPH,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
#[repr(u8)]
pub enum UnitVerticalSpeed {
    #[default]
    Mps,
    Fpm,
    Knots,
}

pub const UNIT_MPS: &'static str = "m/s";
pub const UNIT_FPM: &'static str = "fpm";

impl UnitVerticalSpeed {
    pub fn from_str(name: &str) -> Self {
        match name {
            UNIT_KNOTS => UnitVerticalSpeed::Knots,
            UNIT_FPM => UnitVerticalSpeed::Fpm,
            _ => UnitVerticalSpeed::Mps,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UnitVerticalSpeed::Mps => UNIT_MPS,
            UnitVerticalSpeed::Fpm => UNIT_FPM,
            UnitVerticalSpeed::Knots => UNIT_KNOTS,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
#[repr(u8)]
pub enum UnitHeight {
    #[default]
    Meter,
    Feet,
}

pub const UNIT_METER: &'static str = "m";
pub const UNIT_FEET: &'static str = "ft";

impl UnitHeight {
    pub fn from_str(name: &str) -> Self {
        match name {
            UNIT_FEET => UnitHeight::Feet,
            _ => UnitHeight::Meter,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UnitHeight::Meter => UNIT_METER,
            UnitHeight::Feet => UNIT_FEET,
        }
    }
}

/// Metastructur for config variables
#[derive(Clone, Copy)]
pub struct Config {
    pub circle_hysteresis_tc: i8,
    pub display_active: DisplayActive,
    pub overlay_active: OverlayActive,
    pub type_of_info: TypeOfInfo,
    pub last_display_active: DisplayActive,
    pub user_profile: u8,
    pub glider_idx: i32,
    pub volume: i8,
    pub mc_cready: Speed,
    pub snd_min_freq: f32,
    pub snd_center_freq: f32,
    pub snd_max_freq: f32,
    pub snd_exp_mul: f32,
    pub snd_duty_cycle: u16, // Oscillations, symetric on/off
    pub av2_climb_rate_tc: f32,
    pub av_speed_to_fly_tc: f32,
    pub av_supply_voltage_tc: f32,
    pub theme: &'static Palette,
    pub uuid: u32,
    pub info1_vario: LineView,
    pub info2_vario: LineView,
    pub info3_vario: Info3View,
    pub info1_stf: LineView,
    pub info2_stf: LineView,
    pub info3_stf: Info3View,
    pub center_circling: CenterView,
    pub center_straight: CenterView,
    pub glider_symbol: bool,
    pub battery_good: f32,
    pub battery_low: f32,
    pub stf_upper_limit: Speed,
    pub stf_lower_limit: Speed,
    pub vario_lower_limit: Speed,
    pub vario_upper_limit: Speed,
    pub unit_horizontal_spped: UnitHorizontalSpeed,
    pub unit_vertical_spped: UnitVerticalSpeed,
    pub unit_height: UnitHeight,
}

impl Config {
    pub fn default(theme: &'static Palette, uuid: u32) -> Self {
        Self {
            circle_hysteresis_tc: 7,
            display_active: DisplayActive::Vario,
            overlay_active: OverlayActive::None,
            type_of_info: TypeOfInfo::None,
            last_display_active: DisplayActive::Vario,
            user_profile: 0,
            glider_idx: 104,
            volume: 2,
            mc_cready: 0.7.m_s(),
            snd_min_freq: 233.0,    // -7,5
            snd_center_freq: 659.0, // e2
            snd_max_freq: 1864.0,   // +7,5
            snd_exp_mul: 0.138629,  // -5 .. 5 two octaves
            snd_duty_cycle: 200,
            av2_climb_rate_tc: 30.0,
            av_speed_to_fly_tc: 5.0,
            av_supply_voltage_tc: 3.0,
            theme,
            uuid,
            info1_vario: LineView::None,
            info2_vario: LineView::WindAndDelta,
            info3_vario: Info3View::Climbing,
            info1_stf: LineView::None,
            info2_stf: LineView::WindAndDelta,
            info3_stf: Info3View::SpeedToFly,
            center_circling: CenterView::SingleArrowCircling,
            center_straight: CenterView::SingleArrowStraight,
            glider_symbol: true,
            battery_good: 11.5,
            battery_low: 10.0,
            stf_upper_limit: 10.0.km_h(),
            stf_lower_limit: -10.0.km_h(),
            vario_upper_limit: 0.0.m_s(),
            vario_lower_limit: 0.0.m_s(),
            unit_horizontal_spped: UnitHorizontalSpeed::Kmph,
            unit_vertical_spped: UnitVerticalSpeed::Mps,
            unit_height: UnitHeight::Meter,
        }
    }

    pub fn is_base_display(&self) -> bool {
        self.display_active == DisplayActive::Vario ||
        self.display_active == DisplayActive::Horizon ||
        self.display_active == DisplayActive::DeviceInfo
    }
}
