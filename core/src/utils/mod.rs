mod can_frame;
mod can_ids;
mod can_rdr;
mod can_wtr;
mod colors16;
mod colors8;
mod crc;
mod date_time;
mod error;
mod events;
mod filter;
mod idle_events;
mod metadata;
mod persistence;
mod rgb565_colors;
mod version;
mod version_check;

pub use can_frame::*;
pub use can_ids::*;
pub use can_wtr::*;
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
pub use persistence::*;
pub use rgb565_colors::RGB565_COLORS;
pub use version::*;
pub use version_check::*;

pub use can_ids::{audio_legacy, frontend_legacy, sensor_legacy};
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
