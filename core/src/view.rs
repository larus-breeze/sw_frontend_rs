use embedded_graphics::prelude::*;

pub mod demo;
pub mod edit;

pub(crate) mod elements;
pub(crate) mod vario;

use crate::{model::CoreModel, utils::Colors, CoreError, DrawImage, DISPLAY_HEIGHT, DISPLAY_WIDTH};

// Debug build runs at 10 Hz
#[cfg(debug_assertions)]
pub const FRAME_RATE: u32 = 10;

// Release build runs at 30 Hz
#[cfg(not(debug_assertions))]
pub const FRAME_RATE: u32 = 20;

pub const MARGIN: i32 = 2;
pub const DIAMETER: u32 = DISPLAY_HEIGHT - 2 * MARGIN as u32;
pub const RADIUS: u32 = DIAMETER / 2;
pub const CENTER: Point = Point::new(RADIUS as i32 + MARGIN, RADIUS as i32 + MARGIN);
pub const SCREEN_CENTER: Point = Point::new(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 2);

pub struct CoreView<D>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    pub display: D,
}

impl<D> CoreView<D>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    pub fn new(display: D) -> Self {
        CoreView { display }
    }

    pub fn draw(&mut self, core_model: &mut CoreModel) -> Result<(), CoreError> {
        vario::draw(&mut self.display, core_model)?;

        if core_model.control.edit_ticks > 0 {
            edit::draw(&mut self.display, core_model)?;
        }

        if core_model.control.demo_acitve {
            demo::draw(&mut self.display, core_model)?;
        }

        Ok(())
    }
}
