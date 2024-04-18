use crate::{
    model::CoreModel,
    utils::{Colors, FONT_SMALL},
    CoreError, DrawImage,
};
use embedded_graphics::prelude::*;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub fn draw<D>(display: &mut D, _cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let _ = FONT_SMALL.render_aligned(
        "Demo",
        Point::new(3, 3),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Colors::Yellow),
        display,
    );
    Ok(())
}
