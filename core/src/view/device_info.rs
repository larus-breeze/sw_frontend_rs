use crate::{model::{device_info::DEVICE_INFO_CONTENT, CoreModel}, utils::Colors, CoreError, DrawImage};

use embedded_graphics::{
    draw_target::DrawTarget, 
    prelude::Point,
};

pub const DEVICE_INFO_LINES: usize = 11;

#[derive(PartialEq)]
pub struct DeviceInfo {}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {}
    }

    pub fn draw<D>(&mut self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        display.clear(cm.palette().background)?;
        let width = cm.device_const.sizes.display.width as i32;
        let height = cm.device_const.sizes.display.height as i32;

        for pos in 0..DEVICE_INFO_LINES {
            let pos_y = height / 20 + pos as i32 * height / 12;
            let point = Point::new(width / 2, pos_y);
            let index = pos as usize + cm.control.device_info_control.index as usize;
            let dev_lineview = DEVICE_INFO_CONTENT[index];
            dev_lineview.draw(
                display,
                cm,
                point
            )?;
        }
        Ok(())
    }
}
