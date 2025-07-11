use crate::{utils::Colors, CoreError, DrawImage};
use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
use u8g2_fonts::{
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

pub struct DialogBox<'a> {
    header: &'a str,
    background_color: Colors,
    header_color: Colors,
    underline_color: Colors,
    text_color: Colors,
}

impl<'a> DialogBox<'a> {
    pub fn new(
        header: &'a str,
        background_color: Colors,
        header_color: Colors,
        underline_color: Colors,
        text_color: Colors,
    ) -> Self {
        DialogBox {
            header,
            background_color,
            header_color,
            underline_color,
            text_color,
        }
    }

    pub fn draw<D>(
        &mut self,
        display: &mut D,
        height: u32,
        width: u32,
        text: &str,
        font: &FontRenderer,
    ) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        display.clear(self.background_color)?;

        font.render_aligned(
            self.header,
            Point::new(width as i32 / 2, height as i32 / 14),
            VerticalPosition::Top,
            HorizontalAlignment::Center,
            FontColor::Transparent(self.header_color),
            display,
        )?;

        let y = height as i32 / 5;

        Line::new(Point::new(0, y), Point::new(width as i32, y))
            .into_styled(PrimitiveStyle::with_stroke(self.underline_color, 1))
            .draw(display)?;

        let mut y_pos = height as i32 / 4;
        let delty_y = height as i32 / 7;
        for line in text.lines() {
            font.render_aligned(
                line,
                Point::new(width as i32 / 2, y_pos),
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(self.text_color),
                display,
            )?;
            y_pos += delty_y;
        }

        Ok(())
    }
}
