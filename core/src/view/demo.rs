use crate::{
    model::CoreModel, 
    utils::{FONT_HELV_14, Colors},
    CoreError, DrawImage,
};
use embedded_graphics::prelude::*;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub fn draw<D>(display: &mut D, _cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let _ = FONT_HELV_14.render_aligned(
        "Demo",
        Point::new(3, 3),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Colors::Yellow),
        display,
    );
    Ok(())
}
