use crate::dev_const::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use corelib::*;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    primitives::Rectangle,
    Pixel,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

pub struct MockDisplay {
    pub display: SimulatorDisplay<Colors>,
    rotation: Rotation,
}

use image::{ImageBuffer, ImageFormat::Png, RgbaImage, DynamicImage, imageops, io::Reader};

const HOUSING_IMG: &[u8] = include_bytes!("../housing.png");

impl MockDisplay {
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        let display = SimulatorDisplay::with_default_color(size, Colors::Black);
        MockDisplay {
            display,
            rotation: Rotation::Rotate0,
        }
    }

    pub fn save_png(&mut self, img_path: &str) {
        let output_settings = OutputSettingsBuilder::new().build();
        let output_image = self.display.to_rgb_output_image(&output_settings);
        output_image.save_png(img_path).unwrap();
    }

    pub fn save_png_with_housing(&mut self, img_path: &str) {
        let output_settings = OutputSettingsBuilder::new().build();
        let output_image = self.display.to_rgb_output_image(&output_settings);
        let buf = output_image.as_image_buffer().into_raw().to_vec();
        let img= ImageBuffer::from_raw(480, 480, buf).unwrap();
        let sim_img = DynamicImage::ImageRgb8(img).into_rgba8();

        let mut img_with_housing = RgbaImage::new(620, 620);
        let housing = Reader::with_format(std::io::Cursor::new(HOUSING_IMG), Png)
            .decode()
            .unwrap()
            .to_rgba8();
        imageops::overlay(&mut img_with_housing, &sim_img, 71, 70);
        imageops::overlay(&mut img_with_housing, &housing, 0, 0);
        img_with_housing.save_with_format(img_path, Png).unwrap();
    }
}

impl DrawImage for MockDisplay {
    const DISPLAY_HEIGHT: u32 = DISPLAY_HEIGHT;
    const DISPLAY_WIDTH: u32 = DISPLAY_WIDTH;

    fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    unsafe fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors) {
        let x = (idx % (DISPLAY_WIDTH as usize)) as i32;
        let y = (idx / (DISPLAY_WIDTH as usize)) as i32;
        let top_left = Point::new(x, y);
        let size = Size::new(len as u32, 1);
        let area = Rectangle::new(top_left, size);
        let _ = self.fill_solid(&area, color);
    }
}

impl DrawTarget for MockDisplay {
    type Color = Colors;
    type Error = CoreError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        match self.rotation {
            Rotation::Rotate180 => {
                for pixel in pixels {
                    let (x, y) = (
                        DISPLAY_WIDTH as i32 - 1 - pixel.0.x,
                        DISPLAY_HEIGHT as i32 - 1 - pixel.0.y,
                    );
                    self.display
                        .draw_iter(core::iter::once(Pixel(Point::new(x, y), pixel.1)))
                        .unwrap();
                }
            }
            _ => {
                self.display.draw_iter(pixels).unwrap();
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.
        let area = area.intersection(&self.display.bounding_box());

        match self.rotation {
            Rotation::Rotate180 => {
                let x = DISPLAY_WIDTH as i32 - 1 - area.top_left.x - area.size.width as i32;
                let y = DISPLAY_HEIGHT as i32 - 1 - area.top_left.y - area.size.height as i32;
                let area_inv = Rectangle::new(Point::new(x, y), area.size);
                self.display.fill_solid(&area_inv, color).unwrap();
            }
            _ => {
                self.display.fill_solid(&area, color).unwrap();
            }
        }
        Ok(())
    }
}

impl OriginDimensions for MockDisplay {
    fn size(&self) -> Size {
        self.display.size()
    }
}
