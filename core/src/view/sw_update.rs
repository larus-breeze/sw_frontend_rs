use super::helpers::{dialog_box::DialogBox, themes::Palette};
use crate::{model::CoreModel, tformat, utils::Colors, CoreError, DeviceEvent, DrawImage};

use embedded_graphics::draw_target::DrawTarget;
use heapless::String;

pub struct SwUpdate {
    text: String<100>,
}

impl SwUpdate {
    pub fn new(update_state: DeviceEvent) -> SwUpdate {
        let text = match update_state {
            DeviceEvent::FwAvailable(_version) => tformat!(100, "Not used"),
            DeviceEvent::PrepareFwUpload => tformat!(100, "Preparing..."),
            DeviceEvent::UploadInProgress => {
                tformat!(100, "Installing...\nDo NOT power\noff device")
            }
            _ => tformat!(100, "Error!"),
        };
        SwUpdate {
            text: text.unwrap(),
        }
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let mut dialog_box = DialogBox::new(
            "FW Update",
            cm.color(Palette::Background),
            cm.color(Palette::Scale),
            cm.color(Palette::Scale),
            cm.color(Palette::Text1),
        );
        dialog_box.draw(display, self.text.as_str())
    }
}
