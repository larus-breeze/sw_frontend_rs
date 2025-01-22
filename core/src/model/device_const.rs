use crate::{utils::Colors, CoreModel, HwVersion, SwVersion};
use embedded_graphics::geometry::{Point, Size};
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
    pub m_s: Size,
    pub km_h: Size,
}

pub struct HorizonSizes {
    pub t_width: i32,
    pub rm_len: i32,
    pub rm_width: f32,
    pub stroke_width: i32,
    pub box_height: i32,
    pub tc_pos_y: i32,
    pub tc_needle_y: i32,
    pub tc_needle_delta: i32,
    pub pitch_scale_len: i32,
}

pub struct VarioSizes {
    pub stf_diameter: u32, // stf speed to fly
    pub stf_width: u32,
    pub indicator_len: u32,
    pub glider_pos: Point,
    pub north_pos: Point,
    pub bat_pos: Point,
    pub sat_pos: Point,
    pub unit_pos: Point,
    pub info1_pos: Point,
    pub info2_pos: Point,
    pub info3_pos: Point,
    pub pic_info3_pos: Point,
    pub small_gld_size: Size,
    pub ta_circle_radius: u32,  // ta thermal assistant
    pub ta_point_diameter: u32,
    pub wind_len: i32,
    pub wind_len_min: i32,
    pub angle_m_s: f32,
}

pub struct Images {
    pub bat_empty: &'static [u8],
    pub bat_full: &'static [u8],
    pub bat_half: &'static [u8],
    pub glider: &'static [u8],
    pub north: &'static [u8],
    pub spiral: &'static [u8],
    pub straight: &'static [u8],
    pub km_h: &'static [u8],
    pub m_s: &'static [u8],
    pub sat: &'static [u8],
    pub small_glider: &'static [u8],
    pub wp_editor: &'static [u8],
    pub wp_horizon: &'static [u8],
    pub wp_vario: &'static [u8],
}

#[derive(PartialEq)]
pub struct Palette {
    pub background: Colors,
    pub scale: Colors,
    pub needle1: Colors,
    pub needle2: Colors,
    pub needle3: Colors,
    pub needle4: Colors,
    pub needle5: Colors,
    pub sprite1_stroke: Colors,
    pub sprite1_fill: Colors,
    pub sprite2_stroke: Colors,
    pub sprite2_fill: Colors,
    pub signal_stop: Colors,
    pub signal_warning: Colors,
    pub signal_go: Colors,
    pub text1: Colors,
    pub text1_bold: Colors,
    pub text2: Colors,
    pub text2_bold: Colors,

    pub edit_background: Colors,
    pub edit_stroke: Colors,

    pub horizon_sky: Colors,
    pub horizon_earth: Colors,

    pub vario_speed_to_fly: Colors,
    pub vario_pic_info1: Colors,
    pub vario_wind_plus: Colors,
    pub vario_wind_minus: Colors,

    pub therm_ass_best: Colors,
    pub therm_ass_good: Colors,
    pub therm_ass_bad: Colors,
    pub therm2_ass_best: Colors,
    pub therm2_ass_good: Colors,
    pub therm2_ass_bad: Colors,

}

impl Palette {
    pub const fn default() -> Self {
        Self {
            background: Colors::Black,
            scale: Colors::White,
            needle1: Colors::DarkRed,
            needle2: Colors::Red,
            needle3: Colors::Green,
            needle4: Colors::Yellow,
            needle5: Colors::Red,
            sprite1_stroke: Colors::White,
            sprite1_fill: Colors::DodgerBlue,
            sprite2_stroke: Colors::Blue,
            sprite2_fill: Colors::LightGray,
            signal_stop: Colors::Red,
            signal_warning: Colors::Yellow,
            signal_go: Colors::LimeGreen,
            text1: Colors::LightSalmon,
            text1_bold: Colors::Wheat,
            text2: Colors::DeepSkyBlue,
            text2_bold: Colors::LightSkyBlue,

            edit_background: Colors::DarkBlue,
            edit_stroke: Colors::DodgerBlue,

            horizon_sky: Colors::LightSkyBlue,
            horizon_earth: Colors::Sienna,

            vario_speed_to_fly: Colors::Orange,
            vario_pic_info1: Colors::Orange,
            vario_wind_plus: Colors::Orange,
            vario_wind_minus: Colors::LightPink,

            therm_ass_best: Colors::White,
            therm_ass_good: Colors::Red,
            therm_ass_bad: Colors::Blue,
            therm2_ass_best: Colors::Yellow,
            therm2_ass_good: Colors::Red,
            therm2_ass_bad: Colors::Blue,
        }
    }
}
