use crate::{
    model::{CoreModel, DisplayActive, EditMode},
    utils::{Colors, TString},
    CoreError, DrawImage,
    view::viewable::circle_area::draw_info,
};

use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub struct Edit {
    name_str: &'static str,
    val_str: TString<20>,
}

impl Edit {
    pub fn new(cm: &CoreModel) -> Edit {
        Edit {
            name_str: cm.control.editor.get_head_line(),
            val_str: cm.control.editor.get_value_line(),
        }
    }

    pub fn draw<D>(&mut self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        self.val_str = cm.control.editor.get_value_line();
        if cm.config.display_active == DisplayActive::Vario
            || cm.config.display_active == DisplayActive::Horizon
        {
            match cm.device_const.misc.edit_mode {
                EditMode::Off => Ok(()),
                EditMode::CircleArea => draw_info(display, cm, self.name_str, self.val_str.as_str()),
                EditMode::Fullscreen => self.draw_rectangle_editor(display, cm, true),
                EditMode::Window => self.draw_rectangle_editor(display, cm, false),
            }
        } else {
            self.draw_rectangle_editor(display, cm, true)
        }
    }

    fn draw_rectangle_editor<D>(
        &self,
        display: &mut D,
        cm: &CoreModel,
        fullscreen: bool,
    ) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let d_sizes = &cm.device_const.sizes.display;
        if fullscreen {
            display.clear(cm.palette().edit_background)?;
        } else {
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(cm.palette().edit_stroke)
                .stroke_width(2)
                .fill_color(cm.palette().edit_background)
                .build();

            let d_sizes = &cm.device_const.sizes.display;
            let width = d_sizes.width * 80 / 100;
            let height = d_sizes.height * 50 / 100;

            Rectangle::with_center(d_sizes.screen_center, Size::new(width, height))
                .into_styled(style)
                .draw(display)?;
        };

        let delta_y = cm.device_const.sizes.display.height as i32 / 15;
        cm.device_const.big_font.render_aligned(
            self.name_str,
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
