#![no_std]

mod core_model;
pub use core_model::CoreModel;

pub use system_of_units::{FloatToSpeed, Speed};

pub use flight_physics::*;

mod view;
pub use core_model::{FlyMode, VarioMode};
pub use view::{colors::Colors, draw_view, rgb565_colors::RGB565_COLORS};
pub(crate) mod fmt;

pub mod utils;
pub use utils::{Concat, CoreError};

mod flight_physics;

use embedded_graphics::prelude::*;

#[allow(dead_code)]
pub mod macros;
mod system_of_units;
pub use system_of_units::*;

#[allow(unused_imports)]
use micromath::F32Ext;

#[cfg(feature = "display_size_227x285")]
pub const DISPLAY_WIDTH: u32 = 227;
#[cfg(feature = "display_size_227x285")]
pub const DISPLAY_HEIGHT: u32 = 285;

pub trait DrawImage {
    fn draw_img(&mut self, img: &[u8], offset: Point) -> Result<(), CoreError>;
}
