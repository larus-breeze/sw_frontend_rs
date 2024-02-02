use crate::view::dialog_box::DialogBox;
use crate::{
    model::CoreModel,
    utils::{Colors, Concat},
    CoreError, DeviceEvent, DrawImage,
};
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

    let text = Concat::<100>::new();
    let text = match cm.control.firmware_update_state {
        DeviceEvent::FwAvailable(version) => {
            let v_str = version.as_string();
            text.push_str(v_str.as_str())
                .push_str("\nAccept with key 1\nother keys reject")
        }
        DeviceEvent::PrepareFwUpload => text.push_str("Preparing..."),
        DeviceEvent::UploadInProgress => {
            dialog_box.set_text_color(Colors::Coral);
            text
                .push_str("Installing...\nDo NOT power\noff device")
        }
        _ => text,
    };
    dialog_box.draw(display, text.as_str())?;
    Ok(())
}
