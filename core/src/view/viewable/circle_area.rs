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
    let delta_y = d_sizes.height as i32 / 15;
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

pub fn draw_alarm_info<D>(display: &mut D, cm: &CoreModel, header: &str, img: &'static [u8]) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    display.draw_img(cm.device_const.images.wp_editor, Point::new(0, 0), None)?;

    let d_sizes = &cm.device_const.sizes.display;
    let delta_y = d_sizes.height as i32 / 15;
    cm.device_const.big_font.render_aligned(
        header,
        d_sizes.screen_center + Point::new(0, -delta_y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().alarm),
        display,
    )?;

    if (cm.control.alive_ticks / 10) % 2 == 1 {
        let x = -(d_sizes.alarm.width as i32) / 2;
        let y = -(d_sizes.alarm.height as i32) / 2 + d_sizes.height as i32 / 8;
        display.draw_img(img, d_sizes.screen_center + Point { x: x, y: y }, None)?;
    }
    Ok(())
}
