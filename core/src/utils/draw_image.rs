use crate::{Colors, CoreError};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point};
use num_enum::FromPrimitive;

#[allow(unused_imports)]
use crate::RGB565_COLORS;

#[derive(Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum Rotation {
    #[default]
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

    /// # Safety
    ///
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
        assert!(img_vers == 4);

        if img_vers == 4 {
            // The image is really built for our display?
            assert!(img[2] as u32 + (img[3] as u32) * 256 == Self::DISPLAY_WIDTH);
            assert!(
                img[6] as u32 + (img[7] as u32) * 256 + offset.y as u32 <= Self::DISPLAY_HEIGHT
            );

            // Let's write the pixels
            let idx_col_arr: usize = 9;
            let mut idx = img[8] as usize + idx_col_arr;
            let mut img_idx = (offset.x + offset.y * Self::DISPLAY_WIDTH as i32) as u32;
            let mut color = Colors::Black;
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
                            // unsafe is ok here, because all colors in images are generated correctly
                            let stroke_color = unsafe {Colors::from_u16_unchecked(u16_col) };
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
