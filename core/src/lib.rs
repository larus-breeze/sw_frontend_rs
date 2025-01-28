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
mod common;
mod controller;
mod flight_physics;
pub mod macros;
mod model;
mod system_of_units;
mod utils;
mod view;

// The core components
pub use controller::*;
pub use model::{
    CoreModel, DeviceConst, DisplaySizes, EditMode, Editable, FlyMode, HorizonSizes, Images, Misc,
    Palette, Sizes, VarioMode, VarioSizes,
};
pub use view::{CoreView, FRAME_RATE};

// Some helper functionality
pub use common::*;
pub use flight_physics::*;
pub use system_of_units::*;
pub use utils::*;

// Only for no_std usage
#[allow(unused_imports)]
use micromath::F32Ext;

// basic_config
pub mod basic_config {
    pub const MAX_TX_FRAMES: usize = 10;
    pub const MAX_RX_FRAMES: usize = 30;
    pub const VDA: u16 = 40; // heartbeat at 0x680
    pub const CONTROLLER_TICK_RATE: u32 = 10;
    pub const SECTION_EDITOR_TIMEOUT: u16 = 7;
    pub const MENU_TIMEOUT: u16 = 30;
    pub const PERSISTENCE_TIMEOUT: u16 = 500;
}
