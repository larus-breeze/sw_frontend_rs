use crate::{Colors, CoreError};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point, Drawable, Pixel};

#[allow(unused_imports)]
use crate::RGB565_COLORS;

#[derive(Clone, Copy)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Rotation {
    pub fn name(&self) -> &'static str {
        match self {
            Rotation::Rotate0 => "Rotate 0°",
            Rotation::Rotate90 => "Rotate 90°",
            Rotation::Rotate180 => "Rotate 180°",
            Rotation::Rotate270 => "Rotate 270°",
        }
    }
}

impl From<u32> for Rotation {
    fn from(value: u32) -> Self {
        if value <= Rotation::Rotate270 as u32 {
            // It is garanteed, that value is in range and so safe
            unsafe { core::mem::transmute::<u8, Rotation>(value as u8) }
        } else {
            Rotation::Rotate0
        }
    }
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        match value {
            "Rotate 90°" => Rotation::Rotate90,
            "Rotate 180°" => Rotation::Rotate180,
            "Rotate 270°" => Rotation::Rotate270,
            _ => Rotation::Rotate0,
        }
    }
}

/// Trait of a function to bring an image to the screen. The format of the image files is
/// specifically designed to be ultra-fast. It is defined in the Python script
/// assets/convert_pictures.py and is described there.
pub trait DrawImage {
    const DISPLAY_WIDTH: u32;
    const DISPLAY_HEIGHT: u32;

    /// unsafe in this context means, that the caller has to check display limits
    unsafe fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors);

    fn set_rotation(&mut self, rotation: Rotation);

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
            assert!(img16[1] == Self::DISPLAY_WIDTH as u16);
            assert!(img16[2] + offset.y as u16 <= Self::DISPLAY_HEIGHT as u16);

            // Let's write the pixels
            let color_cnt = img16[3];
            let mut idx = 4;
            for _ in 0..color_cnt {
                let color = if let Some(color) = cover_up {
                    color
                } else {
                    #[cfg(feature = "colors_rgb565")]
                    let u16_col = RGB565_COLORS[img16[idx] as usize];
                    #[cfg(feature = "colors_rgb565")]
                    let color = Colors::from(u16_col);

                    #[cfg(feature = "colors_8_indexed")]
                    let color = Colors::from(img16[idx] as u8);

                    color
                };

                let px_cnt = img16[idx + 1] as usize;
                idx += 2;
                for i_idx in img16.iter().skip(idx).take(px_cnt) {
                    let y = *i_idx / (Self::DISPLAY_WIDTH as u16);
                    let x = *i_idx - y * Self::DISPLAY_WIDTH as u16;
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
            assert!(img32[1] == Self::DISPLAY_WIDTH);
            assert!(img32[2] + offset.y as u32 <= Self::DISPLAY_HEIGHT);

            // Let's write the pixels
            let color_cnt = img32[3];
            let mut idx = 4;
            for _ in 0..color_cnt {
                let color = if let Some(color) = cover_up {
                    color
                } else {
                    #[cfg(feature = "colors_rgb565")]
                    let u16_col = RGB565_COLORS[img32[idx] as usize];
                    #[cfg(feature = "colors_rgb565")]
                    let color = Colors::from(u16_col);

                    #[cfg(feature = "colors_8_indexed")]
                    let color = Colors::from(img32[idx] as u8);

                    color
                };

                let px_cnt = img32[idx + 1] as usize;
                idx += 2;
                for i_idx in img32.iter().skip(idx).take(px_cnt) {
                    let y = *i_idx / Self::DISPLAY_WIDTH;
                    let x = *i_idx - y * Self::DISPLAY_WIDTH;
                    let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                    let _ = Pixel(p, color).draw(self);
                }
                idx += px_cnt;
            }
        }
        if img_vers == 3 {
            // The image is really built for our display?
            assert!(img[2] as u32 + (img[3] as u32) * 256 == Self::DISPLAY_WIDTH);
            assert!(
                img[4] as u32 + (img[5] as u32) * 256 + offset.y as u32 <= Self::DISPLAY_HEIGHT
            );

            // Let's write the pixels
            let idx_col_arr: usize = 7;
            let mut idx = img[6] as usize + 7;
            let mut img_idx = (offset.x + offset.y * Self::DISPLAY_WIDTH as i32) as u32;
            let mut color = Colors::from(0);
            while idx < img.len() {
                let n = img[idx] & 0b0011_1111;
                match img[idx] & 0b1100_0000 {
                    0b0000_0000 => {
                        // We know, that we are within the display limits, so unsafe is ok
                        unsafe {
                            self.draw_line_unchecked(img_idx as usize, n as usize, color);
                        }
                        img_idx += n as u32;
                    }
                    0b0100_0000 => img_idx += n as u32,
                    0b1000_0000 => img_idx += 64 * n as u32,
                    0b1100_0000 => {
                        color = if let Some(color) = cover_up {
                            color
                        } else {
                            let u8_col = img[idx_col_arr + n as usize];

                            #[cfg(feature = "colors_rgb565")]
                            let u16_col = RGB565_COLORS[u8_col as usize];
                            #[cfg(feature = "colors_rgb565")]
                            let stroke_color = Colors::from(u16_col);

                            #[cfg(feature = "colors_8_indexed")]
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
