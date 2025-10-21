use super::viewable::dialog_box::DialogBox;
use crate::{model::CoreModel, tformat, utils::Colors, CoreError, DrawImage};

use embedded_graphics::draw_target::DrawTarget;
use heapless::String;

#[derive(PartialEq)]
pub struct DeviceInfo {
    text: String<100>,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        let text = tformat!(100, "Device Info!");
        DeviceInfo {
            text: text.unwrap(),
        }
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let mut dialog_box = DialogBox::new(
            "Device Info",
            cm.palette().background,
            cm.palette().scale,
            cm.palette().scale,
            cm.palette().text1,
        );
        dialog_box.draw(
            display,
            cm.device_const.sizes.display.height,
            cm.device_const.sizes.display.width,
            self.text.as_str(),
            &cm.device_const.big_font,
        )
    }
}
