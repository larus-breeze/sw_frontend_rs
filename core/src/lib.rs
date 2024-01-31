#![no_std]

#[allow(unused_imports)]
#[cfg(test)]
#[macro_use]
extern crate std;

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
pub use controller::CoreController;
pub use model::{CoreModel, FlyMode, VarioMode};
pub use view::{CoreView, FRAME_RATE};

// Some helper functionality
pub use flight_physics::*;
pub use system_of_units::*;
pub use utils::*;

// Re-exports to be used by the hal
use embedded_graphics::prelude::*;

// Only for no_std usage
#[allow(unused_imports)]
use micromath::F32Ext;

// Basic config
#[cfg(feature = "air_avionics_ad57")]
pub mod basic_config {
    pub const MAX_TX_FRAMES: usize = 10;
    pub const MAX_RX_FRAMES: usize = 30;
    pub const VDA: u16 = 40; // heartbeat at 0x680

    pub const DISPLAY_WIDTH: u32 = 227;
    pub const DISPLAY_HEIGHT: u32 = 285;
    pub const CONTROLLER_TICK_RATE: u32 = 10;
    pub const SECTION_EDITOR_TIMEOUT: u32 = 3;
    pub const PERSISTENCE_TIMEOUT: u32 = 3;

    pub const GLIDER_IMG: &[u8] = include_bytes!("../assets/size_227x285/glider.lif");
    pub const NORTH_IMG: &[u8] = include_bytes!("../assets/size_227x285/north.lif");
    pub const WALLPAPER_IMG: &[u8] = include_bytes!("../assets/size_227x285/vario_wallpaper.lif");
    pub const SPIRAL_IMG: &[u8] = include_bytes!("../assets/size_227x285/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] = include_bytes!("../assets/size_227x285/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes!("../assets/size_227x285/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes!("../assets/size_227x285/m_s.lif");
    pub const WALLPAPER_SCALE: [(i32, i32, &str); 11] = [
        (194, 238, "5"),
        (152, 255, "4"),
        (106, 253, "3"),
        (66, 232, "2"),
        (38, 196, "1"),
        (29, 152, "0"),
        (38, 107, "1"),
        (66, 71, "2"),
        (106, 50, "3"),
        (152, 48, "4"),
        (194, 65, "5"),
    ];
}

#[cfg(feature = "larus_ad57")]
pub mod basic_config {
    pub const MAX_TX_FRAMES: usize = 10;
    pub const MAX_RX_FRAMES: usize = 30;
    pub const VDA: u16 = 40; // heartbeat at 0x680

    pub const DISPLAY_WIDTH: u32 = 240;
    pub const DISPLAY_HEIGHT: u32 = 320;
    pub const CONTROLLER_TICK_RATE: u32 = 10;
    pub const SECTION_EDITOR_TIMEOUT: u32 = 3;
    pub const PERSISTENCE_TIMEOUT: u32 = 3;

    pub const GLIDER_IMG: &[u8] = include_bytes!("../assets/size_240x320/glider.lif");
    pub const NORTH_IMG: &[u8] = include_bytes!("../assets/size_240x320/north.lif");
    pub const WALLPAPER_IMG: &[u8] = include_bytes!("../assets/size_240x320/vario_wallpaper.lif");
    pub const SPIRAL_IMG: &[u8] = include_bytes!("../assets/size_240x320/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] = include_bytes!("../assets/size_240x320/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes!("../assets/size_240x320/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes!("../assets/size_240x320/m_s.lif");
    pub const WALLPAPER_SCALE: [(i32, i32, &str); 11] = [
        (211, 274, "5"),
        (163, 290, "4"),
        (113, 285, "3"),
        (69, 259, "2"),
        (39, 218, "1"),
        (29, 169, "0"),
        (39, 119, "1"),
        (69, 78, "2"),
        (113, 52, "3"),
        (163, 47, "4"),
        (211, 63, "5"),
    ];
}

/// Trait of a function to bring an image to the screen. The format of the image files is
/// specifically designed to be ultra-fast. It is defined in the Python script
/// assets/convert_pictures.py and is described there.
pub trait DrawImage {
    fn draw_img(&mut self, img: &[u8], offset: Point) -> Result<(), CoreError>;
}
