use crate::{
    basic_config::DISPLAY_WIDTH,
    utils::{Colors, FONT_BIG},
    CoreError, DrawImage,
};
use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

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

    pub fn set_text_color(&mut self, color: Colors) {
        self.text_color = color;
    }

    pub fn draw<D>(&mut self, display: &mut D, text: &str) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        display.clear(self.background_color)?;

        FONT_BIG.render_aligned(
            self.header,
            Point::new(DISPLAY_WIDTH as i32 / 2, 20),
            VerticalPosition::Top,
            HorizontalAlignment::Center,
            FontColor::Transparent(self.header_color),
            display,
        )?;

        Line::new(Point::new(0, 60), Point::new(DISPLAY_WIDTH as i32, 60))
            .into_styled(PrimitiveStyle::with_stroke(self.underline_color, 1))
            .draw(display)?;

        let mut y_pos = 80;
        for line in text.lines() {
            FONT_BIG.render_aligned(
                line,
                Point::new(DISPLAY_WIDTH as i32 / 2, y_pos),
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(self.text_color),
                display,
            )?;
            y_pos += 40;
        }

        Ok(())
    }
}
