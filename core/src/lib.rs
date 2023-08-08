#![no_std]

/// The Core Crate is a software component that prepares all displays, processes all inputs and 
/// measured values, and allows uniform access to all data. The component is not executable on 
/// its own. It always requires an adaptation layer for the target hardware, which contains the 
/// coupling to the hardware as well as to the real-time operating system. There are no 
/// dependencies to the used hardware, so that a porting to a new system is simply possible. Only 
/// some optical tweaks and the used images have to be adapted to the used display size.
/// 
/// Implementations for dedicated hardware environments can be found in the device directory.
/// 
/// The model-view-controller software pattern was used. The model contains all data relevant for 
/// display and control. The controller processes measured values and inputs and indirectly 
/// controls the display via the model. The display brings the information to the user (LCD and 
/// speaker). An overview of the structure can be found in the doc directory.

mod controller;
mod flight_physics;
pub mod macros;
mod model;
mod system_of_units;
mod utils;
mod view;

// The core components
pub use model::{CoreModel, FlyMode, VarioMode};
pub use controller::CoreController;
pub use view::{CoreView, FRAME_RATE};

// Some helper functionality
pub use flight_physics::*;
pub use system_of_units::*;
pub use utils::{
    Colors, RGB565_COLORS,
    Concat, CoreError,
    KeyEvent,
};

// Re-exports to be used by the hal
use embedded_graphics::prelude::*;

// Only for no_std usage
#[allow(unused_imports)]
use micromath::F32Ext;

// Basic dimensions of the used display
#[cfg(feature = "display_size_227x285")]
pub const DISPLAY_WIDTH: u32 = 227;
#[cfg(feature = "display_size_227x285")]
pub const DISPLAY_HEIGHT: u32 = 285;

/// Trait of a function to bring an image to the screen. The format of the image files is 
/// specifically designed to be ultra-fast. It is defined in the Python script 
/// assets/convert_pictures.py and is described there.
pub trait DrawImage {
    fn draw_img(&mut self, img: &[u8], offset: Point) -> Result<(), CoreError>;
}
