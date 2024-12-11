use crate::{utils::Colors, CoreModel};
use u8g2_fonts::FontRenderer;

pub struct DeviceConst {
    pub dark_theme: Palette,
    pub bright_theme: Palette,
    pub big_font: FontRenderer,
    pub small_font: FontRenderer,
}

impl CoreModel {
    pub fn palette(&self) -> &'static Palette {
        self.config.theme
    }
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
}

impl Palette {
    pub const fn default() -> Self {
        Self {
            background: Colors::Black,
            scale: Colors::White,
            needle1: Colors::Black,
            needle2: Colors::Red,
            needle3: Colors::Green,
            needle4: Colors::Yellow,
            needle5: Colors::Red,
            sprite1_stroke: Colors::White,
            sprite1_fill: Colors::DodgerBlue,
            sprite2_stroke: Colors::White,
            sprite2_fill: Colors::DodgerBlue,
            signal_stop: Colors::Red,
            signal_warning: Colors::Yellow,
            signal_go: Colors::LimeGreen,
            text1: Colors::Coral,
            text1_bold: Colors::Bisque,
            text2: Colors::LightSkyBlue,
            text2_bold: Colors::White,

            edit_background: Colors::DarkBlue,
            edit_stroke: Colors::DodgerBlue,

            horizon_sky: Colors::LightSkyBlue,
            horizon_earth: Colors::Sienna,

            vario_speed_to_fly: Colors::Orange,
            vario_pic_info1: Colors::Orange,
            vario_wind_plus: Colors::Orange,
            vario_wind_minus: Colors::LightPink,
        }
    }
}
