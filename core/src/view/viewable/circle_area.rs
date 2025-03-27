use crate::{
    model::CoreModel,
    utils::Colors,
    CoreError, DrawImage,
};
use embedded_graphics::prelude::*;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};


pub fn draw_info<D>(display: &mut D, cm: &CoreModel, header: &str, value: &str) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    display.draw_img(cm.device_const.images.wp_editor, Point::new(0, 0), None)?;

    let d_sizes = &cm.device_const.sizes.display;
    let delta_y = cm.device_const.sizes.display.height as i32 / 15;
    cm.device_const.big_font.render_aligned(
        header,
        d_sizes.screen_center + Point::new(0, -delta_y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().text2),
        display,
    )?;

    cm.device_const.big_font.render_aligned(
        value,
        d_sizes.screen_center + Point::new(0, delta_y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().text2_bold),
        display,
    )?;
    Ok(())
}
