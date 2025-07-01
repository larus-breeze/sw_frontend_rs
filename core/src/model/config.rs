use num_enum::FromPrimitive;
use core::convert::From;
use crate::{
    system_of_units::{FloatToSpeed, Speed},
    view::viewable::{centerview::CenterView, lineview::LineView},
    Palette,
};

/// Possible displays
#[derive(Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum DisplayActive {
    #[default]
    Vario,
    Horizon,
    Menu,
    FirmwareUpdate,
}

pub const HORIZON: &str = "Horizon";
#[allow(unused)]
pub const VARIO: &str = "Vario";
pub const FIRMWARE_UPDATE: &str = "Firmware Update";
pub const MENU: &str = "Menu";

impl From<&str> for DisplayActive {
    fn from(value: &str) -> Self {
        match value {
            HORIZON => DisplayActive::Horizon,
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

#[derive(Clone, Copy, PartialEq)]
pub enum TypeOfInfo {
    None,
    WaterBallast,
    GearAlarm,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OverlayActive {
    None,
    Editor,
    Menu,
}

/// Metastructur for config variables
#[derive(Clone, Copy)]
pub struct Config {
    pub circle_hysteresis_tc: i8,
    pub display_active: DisplayActive,
    pub overlay_active: OverlayActive,
    pub info_active: TypeOfInfo,
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
    pub alt_stf_thermal_climb: bool,
    pub theme: &'static Palette,
    pub uuid: u32,
    pub info1: LineView,
    pub info2: LineView,
    pub center_circling: CenterView,
    pub center_straight: CenterView,
    pub glider_symbol: bool,
    pub battery_good: f32,
    pub battery_low: f32,
    pub stf_upper_limit: Speed,
    pub stf_lower_limit: Speed,
}

impl Config {
    pub fn default(theme: &'static Palette, uuid: u32) -> Self {
        Self {
            circle_hysteresis_tc: 7,
            display_active: DisplayActive::Vario,
            overlay_active: OverlayActive::None,
            info_active: TypeOfInfo::None,
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
            alt_stf_thermal_climb: true,
            theme,
            uuid,
            info1: LineView::None,
            info2: LineView::WindAndDelta,
            center_circling: CenterView::SingleArrowCircling,
            center_straight: CenterView::SingleArrowStraight,
            glider_symbol: true,
            battery_good: 11.5,
            battery_low: 10.0,
            stf_upper_limit: 10.0.km_h(),
            stf_lower_limit: -10.0.km_h(),
        }
    }
}
