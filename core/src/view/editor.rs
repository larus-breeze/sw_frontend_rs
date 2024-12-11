use crate::{
    model::CoreModel,
    utils::{Colors, TString},
    CoreError, DrawImage,
};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

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
            .stroke_color(cm.palette().edit_stroke)
            .stroke_width(2)
            .fill_color(cm.palette().edit_background)
            .build();

        let d_sizes = &cm.device_const.sizes.display;
        let height = d_sizes.height * 50 / 100;
        let width = d_sizes.width * 90 / 100;

        Rectangle::with_center(d_sizes.screen_center, Size::new(width, height))
            .into_styled(style)
            .draw(display)?;

        let delta_y = cm.device_const.sizes.display.height as i32 / 15;
        cm.device_const.big_font.render_aligned(
            self.name_str.as_str(),
            d_sizes.screen_center + Point::new(0, -delta_y),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.palette().text2),
            display,
        )?;

        cm.device_const.big_font.render_aligned(
            self.val_str.as_str(),
            d_sizes.screen_center + Point::new(0, delta_y),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.palette().text2_bold),
            display,
        )?;
        Ok(())
    }
}
