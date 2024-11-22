#![no_std]

#[allow(unused_imports)]
#[cfg(test)]
#[macro_use]
extern crate std;

/// The Core Crate is a software component that prepares all displays, processes all inputs and
/// measured values, and allows uniform access to all data. The component is not executable on
/// its own. It always requires an adaptation layer for the target hardware, which contains the
/// coupling to the hardware as well as to the real-time operating system. There are no
/// dependencies to the used hardware, so that a porting to a new system is simply possible. Only
/// some optical tweaks and the used images have to be adapted to the used display size.
///
/// Implementations for dedicated hardware environments can be found in the device directory.
///
/// The model-view-controller software pattern was used. The model contains all data relevant for
/// display and control. The controller processes measured values and inputs and indirectly
/// controls the display via the model. The display brings the information to the user (LCD and
/// speaker). An overview of the structure can be found in the doc directory.
mod common;
mod controller;
mod flight_physics;
pub mod macros;
mod model;
mod system_of_units;
mod utils;
mod view;

// The core components
pub use controller::*;
pub use model::{CoreModel, FlyMode, VarioMode};
pub use view::{CoreView, FRAME_RATE};

// Some helper functionality
pub use common::*;
pub use flight_physics::*;
pub use system_of_units::*;
pub use utils::*;

// Re-exports to be used by the hal
use embedded_graphics::{draw_target::DrawTarget, geometry::Point, Drawable, Pixel};

// Only for no_std usage
#[allow(unused_imports)]
use micromath::F32Ext;

#[macro_export]
macro_rules! include_bytes_aligned {
    ($align_to:expr, $path:expr) => {{
        #[repr(C, align($align_to))]
        struct __Aligned<T: ?Sized>(T);

        const __DATA: &'static __Aligned<[u8]> = &__Aligned(*include_bytes!($path));

        &__DATA.0
    }};
}

// Basic config
#[cfg(feature = "air_avionics_ad57")]
pub mod basic_config {
    pub const MAX_TX_FRAMES: usize = 10;
    pub const MAX_RX_FRAMES: usize = 30;
    pub const VDA: u16 = 40; // heartbeat at 0x680

    pub const DISPLAY_WIDTH: u32 = 227;
    pub const DISPLAY_HEIGHT: u32 = 285;
    pub const CONTROLLER_TICK_RATE: u32 = 10;
    pub const SECTION_EDITOR_TIMEOUT: u16 = 3;
    pub const PERSISTENCE_TIMEOUT: u16 = 500;

    pub const BAT_EMPTY_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_227x285/bat_empty.lif");
    pub const BAT_FULL_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_227x285/bat_full.lif");
    pub const BAT_HALF_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_227x285/bat_half.lif");
    pub const GLIDER_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_227x285/glider.lif");
    pub const NORTH_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_227x285/north.lif");
    pub const SPIRAL_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_227x285/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_227x285/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_227x285/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_227x285/m_s.lif");
    pub const SAT_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_227x285/sat.lif");
    pub const WP_HORIZON_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_227x285/wp_horizon.lif");
    pub const WP_VARIO_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_227x285/wp_vario.lif");
    pub const WP_VARIO_SCALE: [(i32, i32, &str); 11] = [
        (202, 248, "5"),
        (156, 267, "4"),
        (105, 265, "3"),
        (61, 241, "2"),
        (30, 202, "1"),
        (20, 153, "0"),
        (30, 103, "1"),
        (61, 64, "2"),
        (105, 40, "3"),
        (156, 38, "4"),
        (202, 57, "5"),
    ];
}

#[cfg(feature = "larus_frontend_v1")]
pub mod basic_config {
    pub const MAX_TX_FRAMES: usize = 10;
    pub const MAX_RX_FRAMES: usize = 30;
    pub const VDA: u16 = 40; // heartbeat at 0x680

    pub const DISPLAY_WIDTH: u32 = 240;
    pub const DISPLAY_HEIGHT: u32 = 320;
    pub const CONTROLLER_TICK_RATE: u32 = 10;
    pub const SECTION_EDITOR_TIMEOUT: u16 = 3;
    pub const PERSISTENCE_TIMEOUT: u16 = 500;

    pub const BAT_EMPTY_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_240x320/bat_empty.lif");
    pub const BAT_FULL_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_240x320/bat_full.lif");
    pub const BAT_HALF_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_240x320/bat_half.lif");
    pub const GLIDER_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_240x320/glider.lif");
    pub const NORTH_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_240x320/north.lif");
    pub const SPIRAL_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_240x320/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_240x320/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_240x320/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_240x320/m_s.lif");
    pub const SAT_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_240x320/sat.lif");
    pub const WP_HORIZON_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_240x320/wp_horizon.lif");
    pub const WP_VARIO_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_240x320/wp_vario.lif");
    pub const WP_VARIO_SCALE: [(i32, i32, &str); 11] = [
        (217, 282, "5"),
        (166, 299, "4"),
        (112, 293, "3"),
        (65, 266, "2"),
        (34, 223, "1"),
        (23, 170, "0"),
        (34, 117, "1"),
        (65, 74, "2"),
        (112, 47, "3"),
        (166, 41, "4"),
        (217, 58, "5"),
    ];
}

#[cfg(feature = "larus_frontend_v2")]
pub mod basic_config {
    pub const MAX_TX_FRAMES: usize = 10;
    pub const MAX_RX_FRAMES: usize = 30;
    pub const VDA: u16 = 40; // heartbeat at 0x680

    pub const DISPLAY_WIDTH: u32 = 480;
    pub const DISPLAY_HEIGHT: u32 = 480;
    pub const CONTROLLER_TICK_RATE: u32 = 10;
    pub const SECTION_EDITOR_TIMEOUT: u16 = 3;
    pub const PERSISTENCE_TIMEOUT: u16 = 500;

    pub const BAT_EMPTY_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_480x480/bat_empty.lif");
    pub const BAT_FULL_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_480x480/bat_full.lif");
    pub const BAT_HALF_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_480x480/bat_half.lif");
    pub const GLIDER_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_480x480/glider.lif");
    pub const NORTH_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_480x480/north.lif");
    pub const SPIRAL_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_480x480/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_480x480/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_480x480/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_480x480/m_s.lif");
    pub const SAT_IMG: &[u8] = include_bytes_aligned!(4, "../assets/size_480x480/sat.lif");
    pub const WP_HORIZON_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_480x480/wp_horizon.lif");
    pub const WP_VARIO_IMG: &[u8] =
        include_bytes_aligned!(4, "../assets/size_480x480/wp_vario.lif");
    pub const WP_VARIO_SCALE: [(i32, i32, &str); 11] = [
        (338, 413, "5"),
        (261, 445, "4"),
        (178, 441, "3"),
        (104, 402, "2"),
        (54, 336, "1"),
        (36, 255, "0"),
        (54, 174, "1"),
        (104, 108, "2"),
        (178, 69, "3"),
        (261, 65, "4"),
        (338, 97, "5"),
    ];
}

/// Trait of a function to bring an image to the screen. The format of the image files is
/// specifically designed to be ultra-fast. It is defined in the Python script
/// assets/convert_pictures.py and is described there.
pub trait DrawImage {
    fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors);

    fn draw_img(
        &mut self,
        img: &[u8],
        offset: Point,
        cover_up: Option<Colors>,
    ) -> Result<(), CoreError>
    where
        Self: DrawTarget<Color = Colors>,
        Self: Sized,
    {
        let img_vers = img[0];
        assert!((img_vers == 1) || img_vers == 2 || img_vers == 3);

        if img_vers == 1 {
            // Safety: the img format has been defined in terms of compatibility, so the conversion is ok here
            let img16 =
                unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u16, img.len() / 2) };

            // The image is really built for our display?
            assert!(img16[1] == basic_config::DISPLAY_WIDTH as u16);
            assert!(img16[2] + offset.y as u16 <= basic_config::DISPLAY_HEIGHT as u16);

            // Let's write the pixels
            let color_cnt = img16[3];
            let mut idx = 4;
            for _ in 0..color_cnt {
                let color = if let Some(color) = cover_up {
                    color
                } else {
                    #[cfg(feature = "larus_frontend_v1")]
                    let u16_col = RGB565_COLORS[img16[idx] as usize];
                    #[cfg(feature = "larus_frontend_v1")]
                    let color = Colors::from(u16_col);

                    #[cfg(feature = "larus_frontend_v2")]
                    let color = Colors::from(img16[idx] as u8);

                    #[cfg(feature = "air_avionics_ad57")]
                    let color = Colors::from(img16[idx] as u8);

                    color
                };

                let px_cnt = img16[idx + 1] as usize;
                idx += 2;
                for i_idx in img16.iter().skip(idx).take(px_cnt) {
                    let y = *i_idx / (basic_config::DISPLAY_WIDTH as u16);
                    let x = *i_idx - y * basic_config::DISPLAY_WIDTH as u16;
                    let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                    let _ = Pixel(p, color).draw(self);
                }
                idx += px_cnt;
            }
        }
        if img_vers == 2 {
            // Safety: the img format has been defined in terms of compatibility, so the conversion is ok here
            let img32 =
                unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u32, img.len() / 4) };

            // The image is really built for our display?
            assert!(img32[1] == basic_config::DISPLAY_WIDTH);
            assert!(img32[2] + offset.y as u32 <= basic_config::DISPLAY_HEIGHT);

            // Let's write the pixels
            let color_cnt = img32[3];
            let mut idx = 4;
            for _ in 0..color_cnt {
                let color = if let Some(color) = cover_up {
                    color
                } else {
                    #[cfg(feature = "larus_frontend_v1")]
                    let u16_col = RGB565_COLORS[img32[idx] as usize];
                    #[cfg(feature = "larus_frontend_v1")]
                    let color = Colors::from(u16_col);

                    #[cfg(feature = "larus_frontend_v2")]
                    let color = Colors::from(img32[idx] as u8);

                    #[cfg(feature = "air_avionics_ad57")]
                    let color = Colors::from(img32[idx] as u8);

                    color
                };

                let px_cnt = img32[idx + 1] as usize;
                idx += 2;
                for i_idx in img32.iter().skip(idx).take(px_cnt) {
                    let y = *i_idx / basic_config::DISPLAY_WIDTH;
                    let x = *i_idx - y * basic_config::DISPLAY_WIDTH;
                    let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                    let _ = Pixel(p, color).draw(self);
                }
                idx += px_cnt;
            }
        }
        if img_vers == 3 {
            // The image is really built for our display?
            assert!(img[2] as u32 + (img[3] as u32) * 256 == basic_config::DISPLAY_WIDTH);
            assert!(
                img[4] as u32 + (img[5] as u32) * 256 + offset.y as u32
                    <= basic_config::DISPLAY_HEIGHT
            );

            // Let's write the pixels
            let idx_col_arr: usize = 7;
            let mut idx = img[6] as usize + 7;
            let mut img_idx = (offset.x + offset.y * basic_config::DISPLAY_WIDTH as i32) as usize;
            let mut color = Colors::from(0);
            while idx < img.len() {
                let n = img[idx] & 0b0011_1111;
                match img[idx] & 0b1100_0000 {
                    0b0000_0000 => {
                        self.draw_line_unchecked(img_idx, n as usize, color);
                        img_idx += n as usize;
                    }
                    0b0100_0000 => img_idx += n as usize,
                    0b1000_0000 => img_idx += 64 * n as usize,
                    0b1100_0000 => {
                        color = if let Some(color) = cover_up {
                            color
                        } else {
                            let u8_col = img[idx_col_arr + n as usize];

                            #[cfg(feature = "larus_frontend_v1")]
                            let u16_col = RGB565_COLORS[u8_col as usize];
                            #[cfg(feature = "larus_frontend_v1")]
                            let stroke_color = Colors::from(u16_col);

                            #[cfg(feature = "larus_frontend_v2")]
                            let stroke_color = Colors::from(u8_col);

                            #[cfg(feature = "air_avionics_ad57")]
                            let stroke_color = Colors::from(u8_col);

                            stroke_color
                        };
                    }
                    _ => (),
                }
                idx += 1;
            }
        }

        Ok(())
    }
}
