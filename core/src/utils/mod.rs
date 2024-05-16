mod can_frame;
mod colors16;
mod colors8;
mod crc;
mod date_time;
mod error;
mod events;
mod filter;
mod idle_events;
mod metadata;
mod parse;
mod persistence;
mod rgb565_colors;
pub mod themes;
mod version;
mod version_check;

pub use can_frame::*;
#[cfg(feature = "larus_ad57")]
pub use colors16::Colors;
#[cfg(feature = "air_avionics_ad57")]
pub use colors8::Colors;
pub use colors8::Colors as Colors8;
pub use crc::*;
pub use date_time::*;
pub use error::CoreError;
pub use events::*;
pub use filter::*;
pub use idle_events::*;
pub use metadata::*;
pub use parse::*;
pub use persistence::*;
pub use rgb565_colors::RGB565_COLORS;
pub use version::*;
pub use version_check::*;

use u8g2_fonts::{fonts, FontRenderer};

#[cfg(feature = "air_avionics_ad57")]
pub const FONT_SMALL: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_ad57")]
pub const FONT_SMALL: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();

#[cfg(feature = "air_avionics_ad57")]
pub const FONT_BIG: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
#[cfg(feature = "larus_ad57")]
pub const FONT_BIG: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub20_tf>();

use num::clamp;

pub fn val_manip<T>(val: T, key: &KeyEvent, inc1: T, inc2: T, min: T, max: T) -> T
where
    T: core::ops::Add<Output = T> + core::ops::Sub<Output = T> + core::cmp::PartialOrd,
{
    match key {
        KeyEvent::Rotary1Left => clamp(val - inc2, min, max),
        KeyEvent::Rotary1Right => clamp(val + inc2, min, max),
        KeyEvent::Rotary2Left => clamp(val - inc1, min, max),
        KeyEvent::Rotary2Right => clamp(val + inc1, min, max),
        _ => val,
    }
}
