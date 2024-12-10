use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    model::CoreModel,
    utils::{Colors, TString},
    view::{
        helpers::themes::{FONT_BIG, Palette},
        SCREEN_CENTER,
    },
    CoreError, DrawImage,
};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

const WIDTH: u32 = DISPLAY_WIDTH * 90 / 100;
const HEIGHT: u32 = DISPLAY_HEIGHT * 50 / 100;

pub struct Edit {
    name_str: TString<16>,
    val_str: TString<20>,
}

impl Edit {
    pub fn new(cm: &CoreModel) -> Edit {
        Edit {
            name_str: cm.control.editor.get_head_line(),
            val_str: cm.control.editor.get_value_line(),
        }
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(cm.color(Palette::EditStroke))
            .stroke_width(2)
            .fill_color(cm.color(Palette::EditBackground))
            .build();

        Rectangle::with_center(SCREEN_CENTER, Size::new(WIDTH, HEIGHT))
            .into_styled(style)
            .draw(display)?;

        const DELTA_Y: i32 = DISPLAY_HEIGHT as i32 / 15;

        FONT_BIG.render_aligned(
            self.name_str.as_str(),
            SCREEN_CENTER + Point::new(0, -DELTA_Y),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.color(Palette::Text1Bold)),
            display,
        )?;

        FONT_BIG.render_aligned(
            self.val_str.as_str(),
            SCREEN_CENTER + Point::new(0, DELTA_Y),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.color(Palette::Text2Bold)),
            display,
        )?;
        Ok(())
    }
}
