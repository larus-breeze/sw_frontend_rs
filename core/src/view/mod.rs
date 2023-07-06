use embedded_graphics::prelude::*;

pub mod colors;
pub mod rgb565_colors;

pub(crate) mod elements;
pub(crate) mod vario;

use crate::{core_model::CoreModel, CoreError, DrawImage, DISPLAY_HEIGHT};

pub const MARGIN: i32 = 2;
pub const DIAMETER: u32 = DISPLAY_HEIGHT - 2 * MARGIN as u32;
pub const RADIUS: u32 = DIAMETER / 2;
pub const CENTER: Point = Point::new(RADIUS as i32 + MARGIN, RADIUS as i32 + MARGIN);

pub fn draw_view<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = colors::Colors, Error = CoreError> + DrawImage,
{
    vario::draw(display, cm)
}
