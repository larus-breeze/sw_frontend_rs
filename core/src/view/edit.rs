use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    model::CoreModel,
    utils::{Colors, TString, FONT_BIG, FONT_SMALL},
    view::SCREEN_CENTER,
    CoreError, DrawImage,
};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use tfmt::Convert;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

const VALUE_COLOR: Colors = Colors::White;
const DESCRIPTION_COLOR: Colors = Colors::Yellow;
const BACKGROUND_COLOR: Colors = Colors::MidnightBlue;
const BORDER_COLOR: Colors = Colors::LightSteelBlue;

const WIDTH: u32 = DISPLAY_WIDTH * 90 / 100;
const HEIGHT: u32 = DISPLAY_HEIGHT * 32 / 100;

pub struct Edit {
    name_str: TString<16>,
    val_str: Convert<20>,
}

impl Edit {
    pub fn new(cm: &CoreModel) -> Edit {
        Edit {
            name_str: cm.control.editor.get_head_line(),
            val_str: cm.control.editor.get_value_line(),
        }
    }

    pub fn draw<D>(&self, display: &mut D, _cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BORDER_COLOR)
            .stroke_width(2)
            .fill_color(BACKGROUND_COLOR)
            .build();

        Rectangle::with_center(SCREEN_CENTER, Size::new(WIDTH, HEIGHT))
            .into_styled(style)
            .draw(display)?;

        FONT_SMALL.render_aligned(
            self.name_str.as_str(),
            SCREEN_CENTER + Point::new(0, -13),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(DESCRIPTION_COLOR),
            display,
        )?;

        FONT_BIG.render_aligned(
            self.val_str.as_str(),
            SCREEN_CENTER + Point::new(0, 25),
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(VALUE_COLOR),
            display,
        )?;
        Ok(())
    }
}
