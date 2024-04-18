use crate::{
    model::CoreModel, system_of_units::{FloatToSpeed, Speed}, utils::themes::{Palette, PaletteColors, DARK_MODE}, Colors, HwVersion, SwVersion
};

/// Possible displays
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum DisplayActive {
    Vario,
    FirmwareUpdate,
}

/// Metastructur for config variables
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
    pub uuid: u32,
    pub hw_version: HwVersion,
    pub sw_version: SwVersion,
    pub av2_climb_rate_tc: f32,
    pub av_speed_to_fly_tc: f32,
    pub theme: &'static PaletteColors,
}

impl Default for Config {
    fn default() -> Self {
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
            uuid: 0,
            hw_version: HwVersion::default(),
            sw_version: SwVersion::default(),
            av2_climb_rate_tc: 30.0,
            av_speed_to_fly_tc: 5.0,
            theme: &DARK_MODE,
        }
    }
}

impl CoreModel {
    pub fn color(&self, color_name: Palette) -> Colors {
        self.config.theme[color_name as usize]
    }
}
