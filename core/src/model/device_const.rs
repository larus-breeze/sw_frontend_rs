use crate::{utils::Colors, CoreModel, HwVersion, SwVersion};
use embedded_graphics::geometry::Point;
use u8g2_fonts::FontRenderer;

use super::EditMode;

pub struct DeviceConst {
    pub dark_theme: Palette,
    pub bright_theme: Palette,
    pub big_font: FontRenderer,
    pub small_font: FontRenderer,
    pub images: Images,
    pub sizes: Sizes,
    pub misc: Misc,
}

impl CoreModel {
    pub fn palette(&self) -> &'static Palette {
        self.config.theme
    }
}

pub struct Sizes {
    pub vario: VarioSizes,
    pub horizon: HorizonSizes,
    pub display: DisplaySizes,
}

pub struct Misc {
    pub sw_version: SwVersion,
    pub hw_version: HwVersion,
    pub edit_mode: EditMode,
}

pub struct DisplaySizes {
    pub height: u32,
    pub width: u32,
    pub margin: u32,
    pub radius: u32,
    pub center: Point,
    pub screen_center: Point,
}

pub struct HorizonSizes {
    pub pitch_scale_len: i32,
    pub lt_swing_len: i32,
    pub lt_mid: i32,
    pub lt_rad: u32,
}

pub struct VarioSizes {
    pub stf_diameter: u32, // stf speed to fly
    pub stf_width: u32,
    pub indicator_len: u32,
    pub attention_pos: Point,
    pub glider_pos: Point,
    pub north_pos: Point,
    pub bat_pos: Point,
    pub sat_pos: Point,
    pub unit_pos: Point,
    pub info1_pos: Point,
    pub info2_pos: Point,
    pub info3_pos: Point,
    pub pic_info3_pos: Point,
    pub ta_circle_radius: u32, // ta thermal assistant
    pub ta_point_diameter: u32,
    pub wind_len: i32,
    pub wind_len_min: i32,
    pub scale_factor: f32, // angle in degrees betwenn 0 and the first entry
}

pub struct Images {
    pub attention: &'static [u8],
    pub avg_climb_rate: &'static [u8],
    pub bat_empty: &'static [u8],
    pub bat_full: &'static [u8],
    pub bat_half: &'static [u8],
    pub drift_angle: &'static [u8],
    pub flight_level: &'static [u8],
    pub fpm: &'static [u8],
    pub fpm_100: &'static [u8],
    pub ft: &'static [u8],
    pub gear: &'static [u8],
    pub glider: &'static [u8],
    pub km_h: &'static [u8],
    pub kt: &'static [u8],
    pub mph: &'static [u8],
    pub m: &'static [u8],
    pub m_s: &'static [u8],
    pub north: &'static [u8],
    pub sat: &'static [u8],
    pub small_glider: &'static [u8],
    pub speed_to_fly: &'static [u8],
    pub spiral: &'static [u8],
    pub straight: &'static [u8],
    pub tas: &'static [u8],
    pub true_course: &'static [u8],
    pub wp_editor: &'static [u8],
    pub wp_horizon: &'static [u8],
    pub wp_vario_5: &'static [u8],
    pub wp_vario_10: &'static [u8],
}

#[derive(PartialEq)]
pub struct Palette {
    pub horizon: HorizonPalette,
    pub list_edit: ListEditorPalette,
    pub signal: SignalPalette,
    pub vario: VarioPalette,
}

impl Palette {
    pub const fn default() -> Self {
        Self {
            horizon: HorizonPalette::default(),
            list_edit: ListEditorPalette::default(),
            signal: SignalPalette::default(),
            vario: VarioPalette::default(),
        }
    }
}

#[derive(PartialEq)]
pub struct ListEditorPalette {
    pub background: Colors,
    pub header: Colors,
    pub item: Colors,
    pub frame: Colors,
    pub name: Colors,
    pub value: Colors,
}

impl ListEditorPalette {
    pub const fn default() -> Self {
        Self {
            background: Colors::Black,
            header: Colors::LightBlue,
            item: Colors::Bisque,
            frame: Colors::White,
            name: Colors::Bisque,
            value: Colors::White,
        }
    }
}

#[derive(PartialEq)]
pub struct HorizonPalette {
    pub sky: Colors,
    pub earth: Colors,
    pub scale: Colors,
    pub needle: Colors,
}

impl HorizonPalette {
    pub const fn default() -> Self {
        Self {
            sky: Colors::LightSkyBlue,
            earth: Colors::Sienna,
            scale: Colors::White,
            needle: Colors::Red,
        }
    }
}

#[derive(PartialEq)]
pub struct SignalPalette {
    pub stop: Colors,
    pub warning: Colors,
    pub go: Colors,
    pub alarm: Colors,
}

impl SignalPalette {
    pub const fn default() -> Self {
        Self {
            stop: Colors::Red,
            warning: Colors::Yellow,
            go: Colors::LimeGreen,
            alarm: Colors::OrangeRed,
        }
    }
}

#[derive(PartialEq)]
pub struct VarioPalette {
    pub background: Colors,
    pub scale: Colors,

    pub needle: Colors,
    pub avg_climb: Colors,
    pub mc_cready: Colors,
    pub stf_arc: Colors,

    pub wind_stroke: Colors,
    pub wind_fill: Colors,
    pub avg_wind_stroke: Colors,
    pub avg_wind_fill: Colors,
    pub wind_diff: Colors,

    pub icon: Colors,
    pub value: Colors,
    pub unit: Colors,

    pub therm_ass_best: Colors,
    pub therm_ass_good: Colors,
    pub therm_ass_bad: Colors,
    pub therm2_ass_best: Colors,
    pub therm2_ass_good: Colors,
    pub therm2_ass_bad: Colors,
}

impl VarioPalette {
    pub const fn default() -> Self {
        Self {
            background: Colors::Black,
            scale: Colors::White,

            needle: Colors::DarkRed,
            avg_climb: Colors::Green,
            mc_cready: Colors::Red,
            stf_arc: Colors::Orange,

            wind_stroke: Colors::White,
            wind_fill: Colors::DodgerBlue,
            avg_wind_stroke: Colors::Blue,
            avg_wind_fill: Colors::LightGray,
            wind_diff: Colors::LightPink,

            icon: Colors::Orange,
            value: Colors::White,
            unit: Colors::DarkGray,

            therm_ass_best: Colors::Yellow,
            therm_ass_good: Colors::Red,
            therm_ass_bad: Colors::DeepSkyBlue,
            therm2_ass_best: Colors::Yellow,
            therm2_ass_good: Colors::Red,
            therm2_ass_bad: Colors::Blue,
        }
    }
}
