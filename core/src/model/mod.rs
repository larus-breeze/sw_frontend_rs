mod calculated;
mod config;
mod control;
mod device;
mod persistence;
mod sensor;

use crate::{
    basic_config::MAX_TX_FRAMES, common::PTxFrames, flight_physics::GliderData, utils::PIdleEvents,
    HwVersion, SwVersion,
};
use calculated::Calculated;
pub use config::{Config, DisplayActive};
pub use control::{
    Control, EditMode, FlyMode, SystemState, TcrMode, VarioMode, VarioModeControl, MAX_PERS_IDS,
};
use device::Device;
pub use sensor::{GpsState, Sensor};

/// Data model for the entire device
///
/// In the CoreModel all variables of the display are kept in a structure, which is used by
/// different modules. The CoreModel is filled by the CoreController, which holds the different
/// channels to the data sources like Larus, controls, sensors, inputs and time. The View
/// modules display the data contents. The LCD display and the sound system are the most
/// important ones.
pub struct CoreModel {
    pub calculated: Calculated,
    pub config: Config,
    pub control: Control,
    pub device: Device,
    pub glider_data: GliderData,
    pub sensor: Sensor,
    p_idle_events: PIdleEvents,
    pub p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
}

impl CoreModel {
    pub fn new(
        p_idle_events: PIdleEvents,
        p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
        uuid: u32,
        hw_version: HwVersion,
        sw_version: SwVersion,
    ) -> Self {
        let calculated = Calculated::default();
        let config = Config {
            uuid,
            hw_version,
            sw_version,
            ..Default::default()
        };
        let control = Control::default();
        let device = Device::default();
        let glider_data = GliderData::default();
        let sensor = Sensor::default();
        CoreModel {
            calculated,
            config,
            control,
            device,
            glider_data,
            sensor,
            p_idle_events,
            p_tx_frames,
        }
    }
}
