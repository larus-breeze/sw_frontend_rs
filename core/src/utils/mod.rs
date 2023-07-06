mod concat;
mod error;

pub use concat::Concat;
pub use error::CoreError;

use u8g2_fonts::{fonts, FontRenderer};
pub const FONT_HELV_14: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR14_tf>();
pub const FONT_HELV_18: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR18_tf>();
