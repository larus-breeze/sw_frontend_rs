use corelib::{Palette, DeviceConst};
use u8g2_fonts::{fonts, FontRenderer};

pub const DEVICE_CONST: DeviceConst = DeviceConst {
    dark_theme: Palette::default(),
    bright_theme: Palette::default(),
    big_font: BIG_FONT,
    small_font: SMALL_FONT,
};

pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();


/*#[cfg(feature = "air_avionics_ad57")]
pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_frontend_v1")]
pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_frontend_v2")]
pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();

#[cfg(feature = "air_avionics_ad57")]
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
#[cfg(feature = "larus_frontend_v1")]
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub20_tf>();
#[cfg(feature = "larus_frontend_v2")]
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();
*/
