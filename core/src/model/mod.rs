mod calculated;
mod config;
mod control;
mod device;
mod device_const;
pub mod editable;
pub mod menu;
mod sensor;

use crate::flight_physics::{polar_store, GliderData};
use calculated::Calculated;
pub use config::{Config, DisplayActive, OverlayActive};
pub use control::{Control, EditMode, FlyMode, SystemState, TcrMode, VarioMode, VarioModeControl};
use device::Device;
pub use device_const::{
    DeviceConst, DisplaySizes, HorizonSizes, Images, Misc, Palette, Sizes, VarioSizes,
};
pub use editable::Editable;
pub use sensor::{GpsState, Sensor};

/// Data model for the entire device
///
/// In the CoreModel all variables of the display are kept in a structure, which is used by
/// different modules. The CoreModel is filled by the CoreController, which holds the different
/// channels to the data sources like Larus, controls, sensors, inputs and time. The View
/// modules display the data contents. The LCD display and the sound system are the most
/// important ones.
#[derive(Clone, Copy)]
pub struct CoreModel {
    pub calculated: Calculated,
    pub config: Config,
    pub control: Control,
    pub device: Device,
    pub device_const: &'static DeviceConst,
    pub glider_data: GliderData,
    pub sensor: Sensor,
}

impl CoreModel {
    pub fn new(device_const: &'static DeviceConst, uuid: u32) -> Self {
        let calculated = Calculated::default();
        let config = Config::default(&device_const.dark_theme, uuid);
        let control = Control::default();
        let device = Device::default();
        let glider_data = GliderData {
            basic_glider_data: *polar_store::from_raw_idx(config.glider_idx as usize),
            ..Default::default()
        };

        let sensor = Sensor::default();
        CoreModel {
            calculated,
            config,
            control,
            device,
            device_const,
            glider_data,
            sensor,
        }
    }
}
