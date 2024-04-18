use crate::view::dialog_box::DialogBox;
use crate::{model::CoreModel, tformat, utils::Colors, CoreError, DeviceEvent, DrawImage};
use embedded_graphics::draw_target::DrawTarget;

pub fn draw<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let mut dialog_box = DialogBox::new(
        "Firmware Update",
        Colors::Black,
        Colors::White,
        Colors::Gray,
        Colors::LightSkyBlue,
    );

    let text = match cm.control.firmware_update_state {
        DeviceEvent::FwAvailable(_version) => tformat!(100, "Not used"),
        DeviceEvent::PrepareFwUpload => tformat!(100, "Preparing..."),
        DeviceEvent::UploadInProgress => {
            dialog_box.set_text_color(Colors::Coral);
            tformat!(100, "Installing...\nDo NOT power\noff device")
        }
        _ => tformat!(100, "Error!"),
    };
    dialog_box.draw(display, text.unwrap().as_str())?;
    Ok(())
}
