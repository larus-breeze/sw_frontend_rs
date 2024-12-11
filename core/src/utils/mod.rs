mod colors16;
mod colors8;
mod crc;
mod date_time;
mod draw_image;
mod error;
mod events;
mod filter;
mod idle_events;
mod metadata;
mod parse;
mod persistence;
mod rgb565_colors;
mod tstring;
mod version;
mod version_check;

#[cfg(feature = "colors_rgb565")]
pub use colors16::Colors;
#[cfg(feature = "colors_8_indexed")]
pub use colors8::Colors;
pub use colors8::Colors as Colors8;
pub use crc::*;
pub use date_time::*;
pub use draw_image::*;
pub use error::CoreError;
pub use events::*;
pub use filter::*;
pub use idle_events::*;
pub use metadata::*;
pub use parse::*;
pub use persistence::*;
pub use rgb565_colors::RGB565_COLORS;
pub use tstring::*;
pub use version::*;
pub use version_check::*;

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

/*#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        basic_config::MAX_TX_FRAMES, CoreController, CoreModel, DeviceConst, HwVersion, Images, Palette, QIdleEvents, QTxFrames, SwVersion
    };
    use heapless::spsc::Queue;
    use u8g2_fonts::{fonts, FontRenderer};

    const HW_VERSION: HwVersion = HwVersion::from_bytes([1, 3, 1, 0]);
    const SW_VERSION: SwVersion = SwVersion {
        version: [0, 0, 0, 0],
    };

    #[allow(unused)]
    pub(crate) fn cores() -> (CoreModel, CoreController) {
        let (p_tx_frames, _c_tx_frames) = {
            static mut Q_TX_FRAMES: QTxFrames<MAX_TX_FRAMES> = Queue::new();
            // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
            unsafe { Q_TX_FRAMES.split() }
        };

        // This queue routes the StorageItems from the controller to the idle loop.
        let (p_idle_events, _c_idle_events) = {
            static mut Q_IDLE_EVENTS: QIdleEvents = Queue::new();
            // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
            unsafe { Q_IDLE_EVENTS.split() }
        };

        const DEVICE_CONST: DeviceConst = DeviceConst {
            dark_theme: Palette::default(),
            bright_theme: Palette::default(),
            big_font: FontRenderer::new::<fonts::u8g2_font_fub20_tf>(),
            small_font: FontRenderer::new::<fonts::u8g2_font_fub20_tf>(),
            images: Images::new(),
        };

        let mut model = CoreModel::new(1234_u32, HW_VERSION, SW_VERSION, &DEVICE_CONST);
        let controller = CoreController::new(&mut model, p_idle_events, p_tx_frames);
        (model, controller)
    }
}*/
