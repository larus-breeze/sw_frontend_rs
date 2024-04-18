use corelib::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    *,
};
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

pub struct MockDisplay {
    pub display: SimulatorDisplay<Colors>,
}

impl MockDisplay {
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        let display = SimulatorDisplay::with_default_color(size, Colors::Black);
        MockDisplay { display }
    }

    pub fn save_png(&mut self, img_path: &str) {
        let output_settings = OutputSettingsBuilder::new().build();
        let output_image = self.display.to_rgb_output_image(&output_settings);
        output_image.save_png(img_path).unwrap();
    }
}

impl DrawImage for MockDisplay {
    fn draw_img(
        &mut self,
        img: &[u8],
        offset: Point,
        cover_up: Option<Colors>,
    ) -> Result<(), CoreError> {
        // Safety: the img format has been defined in terms of compatibility, so the conversion is ok here
        // At the moment we only know format 1
        let img_vers = img[0];
        assert!((img_vers == 1) || img_vers == 2);

        if img_vers == 1 {
            let img16 =
                unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u16, img.len() / 2) };

            // The image is really built for our display?
            assert!(img16[1] == DISPLAY_WIDTH as u16);
            assert!(img16[2] + offset.y as u16 <= DISPLAY_HEIGHT as u16);

            // Let's write the pixels
            let color_cnt = img16[3];
            let mut idx = 4;
            for _ in 0..color_cnt {
                let color = if let Some(color) = cover_up {
                    Colors::from(color)
                } else {
                    #[cfg(feature = "larus_ad57")]
                    let u16_col = RGB565_COLORS[img16[idx] as usize];
                    #[cfg(feature = "larus_ad57")]
                    let color = Colors::from(u16_col);

                    #[cfg(feature = "air_avionics_ad57")]
                    let color = Colors::from(img16[idx] as u8);
                    color
                };

                let px_cnt = img16[idx + 1] as usize;
                idx += 2;
                for i_idx in img16.iter().skip(idx).take(px_cnt) {
                    let y = *i_idx / (DISPLAY_WIDTH as u16);
                    let x = *i_idx - y * DISPLAY_WIDTH as u16;
                    let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                    let _ = Pixel(p, color).draw(self);
                }
                idx += px_cnt;
            }
        }
        if img_vers == 2 {
            let img32 =
                unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u32, img.len() / 2) };

            // The image is really built for our display?
            assert!(img32[1] == DISPLAY_WIDTH);
            assert!(img32[2] + offset.y as u32 <= DISPLAY_HEIGHT);

            // Let's write the pixels
            let color_cnt = img32[3];
            let mut idx = 4;
            for _ in 0..color_cnt {
                let color = if let Some(color) = cover_up {
                    Colors::from(color)
                } else {
                    #[cfg(feature = "larus_ad57")]
                    let u16_col = RGB565_COLORS[img32[idx] as usize];
                    #[cfg(feature = "larus_ad57")]
                    let color = Colors::from(u16_col);

                    #[cfg(feature = "air_avionics_ad57")]
                    let color = Colors::from(img32[idx] as u8);

                    color
                };

                let px_cnt = img32[idx + 1] as usize;
                idx += 2;
                for i_idx in img32.iter().skip(idx).take(px_cnt) {
                    let y = *i_idx / DISPLAY_WIDTH;
                    let x = *i_idx - y * DISPLAY_WIDTH;
                    let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                    let _ = Pixel(p, color).draw(self);
                }
                idx += px_cnt;
            }
        }

        Ok(())
    }
}

impl DrawTarget for MockDisplay {
    type Color = Colors;
    type Error = CoreError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels).unwrap();

        Ok(())
    }
}

impl OriginDimensions for MockDisplay {
    fn size(&self) -> Size {
        self.display.size()
    }
}
