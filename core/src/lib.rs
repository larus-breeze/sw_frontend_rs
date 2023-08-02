#![no_std]

mod model;
pub use model::CoreModel;

pub use system_of_units::{FloatToSpeed, Speed};

pub use flight_physics::*;

mod view;
pub use model::{FlyMode, VarioMode};
pub use utils::{Colors, RGB565_COLORS};
pub use view::{CoreView, FRAME_RATE};

mod controller;
pub use controller::CoreController;

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
