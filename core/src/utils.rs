mod can_ids;
mod can_rdr;
mod can_wtr;
mod colors;
mod concat;
mod config_item;
mod error;
mod key_event;
mod rgb565_colors;

pub use colors::Colors;
pub use concat::Concat;
pub use error::CoreError;
pub use key_event::*;
pub use rgb565_colors::RGB565_COLORS;

pub use can_ids::{audio, frontend, sensor};
pub(crate) use can_rdr::read_can_frame;

use u8g2_fonts::{fonts, FontRenderer};
pub const FONT_HELV_14: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR14_tf>();
pub const FONT_HELV_18: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR18_tf>();

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
