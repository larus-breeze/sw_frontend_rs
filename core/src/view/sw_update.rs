use super::helpers::dialog_box::DialogBox;
use crate::{model::CoreModel, tformat, utils::Colors, CoreError, DeviceEvent, DrawImage};

use embedded_graphics::draw_target::DrawTarget;
use heapless::String;

pub struct SwUpdate {
    text: String<100>,
}

impl SwUpdate {
    pub fn new(cm: &CoreModel) -> SwUpdate {
        let text = match cm.control.firmware_update_state {
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

    pub fn draw<D>(&self, display: &mut D, _cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let mut dialog_box = DialogBox::new(
            "Firmware Update",
            Colors::Black,
            Colors::White,
            Colors::Gray,
            Colors::Coral,
        );
        dialog_box.draw(display, self.text.as_str())
    }
}
