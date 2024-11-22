use basic_config::DISPLAY_WIDTH;
use corelib::*;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    primitives::Rectangle,
    Pixel,
};
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
    fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors) {
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
        self.display.draw_iter(pixels).unwrap();

        Ok(())
    }
}

impl OriginDimensions for MockDisplay {
    fn size(&self) -> Size {
        self.display.size()
    }
}
