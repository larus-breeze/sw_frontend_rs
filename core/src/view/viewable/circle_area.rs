use crate::{model::CoreModel, utils::Colors, CoreError, DrawImage, Image};
use embedded_graphics::prelude::*;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub fn draw_info<D>(
    display: &mut D,
    cm: &CoreModel,
    header: &str,
    value: &str,
    color: Option<Colors>,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    display.draw_img(cm.device_const.images.wp_editor, Point::new(0, 0), None)?;

    let d_sizes = &cm.device_const.sizes.display;
    let delta_y = d_sizes.height as i32 / 15;
    let color = if let Some(color) = color {
        color
    } else {
        cm.palette().list_edit.item
    };
    cm.device_const.big_font.render_aligned(
        header,
        d_sizes.screen_center + Point::new(0, -delta_y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().list_edit.item),
        display,
    )?;

    cm.device_const.big_font.render_aligned(
        value,
        d_sizes.screen_center + Point::new(0, delta_y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;
    Ok(())
}

pub fn draw_alarm_info<D>(
    display: &mut D,
    cm: &CoreModel,
    header: &str,
    img: &'static [u8],
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    display.draw_img(cm.device_const.images.wp_editor, Point::new(0, 0), None)?;

    let d_sizes = &cm.device_const.sizes.display;
    let delta_y = d_sizes.height as i32 / 15;
    cm.device_const.big_font.render_aligned(
        header,
        d_sizes.center + Point::new(0, -delta_y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().signal.alarm),
        display,
    )?;

    if (cm.control.alive_ticks / 10) % 2 == 1 {
        let alarm_img = Image::new(img);
        let x = -(alarm_img.width() as i32) / 2;
        let y = -(alarm_img.height() as i32) / 2 + d_sizes.height as i32 / 8;
        alarm_img.draw(display, d_sizes.center + Point { x, y }, None)?;
    }
    Ok(())
}
