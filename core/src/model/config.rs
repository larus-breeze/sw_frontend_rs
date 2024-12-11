use crate::{
    system_of_units::{FloatToSpeed, Speed},
    Palette,
};
use core::{convert::From, mem::transmute};

/// Possible displays
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum DisplayActive {
    Vario,
    Horizon,
    Menu,
    FirmwareUpdate,
    TheEnd,
}

impl From<u8> for DisplayActive {
    fn from(value: u8) -> Self {
        if value < DisplayActive::TheEnd as u8 {
            unsafe { transmute::<u8, DisplayActive>(value) }
        } else {
            panic!()
        }
    }
}

/// Metastructur for config variables
#[derive(Clone, Copy)]
pub struct Config {
    pub display_active: DisplayActive,
    pub last_display_active: DisplayActive,
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
    pub alt_stf_thermal_climb: bool,
    pub theme: &'static Palette,
}

impl Config {
    pub fn default(theme: &'static Palette) -> Self {
        Self {
            display_active: DisplayActive::Vario,
            last_display_active: DisplayActive::Vario,
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
            alt_stf_thermal_climb: false,
            theme,
        }
    }
}
