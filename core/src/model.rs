use embedded_graphics::geometry::{Angle, AngleUnit};
use crate::{IdleEvent, PersistenceItem, PersistenceId, Mass};
use crate::utils::{PIdleEvents, DeviceEvent};

use crate::{
    controller::Editable,
    flight_physics::{GliderData, WindVector},
    system_of_units::{
        Acceleration, AngularVelocity, FloatToAcceleration, FloatToAngularVelocity, FloatToSpeed,
        Pressure, Speed,
    },
    AirSpeed, Density,
};

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
    pub glider_data: GliderData,
    pub sensor: Sensor,
    p_sto_items: PIdleEvents,
}

impl CoreModel {
    pub fn new(p_sto_items: PIdleEvents) -> Self {
        let calculated = Calculated::default();
        let config = Config::default();
        let control = Control::default();
        let glider_data = GliderData::default();
        let sensor = Sensor::default();
        CoreModel { calculated, config, control, glider_data, sensor, p_sto_items }
    }

    pub fn storage_item(&mut self, storage_item: IdleEvent) {
        let _ = self.p_sto_items.enqueue(storage_item);
    }

    pub fn restore_persistent_item(&mut self, item: PersistenceItem) {
        match item.id {
            PersistenceId::Volume => self.config.volume = item.to_i8(),
            PersistenceId::McCready => self.config.mc_cready = Speed::from_m_s(item.to_f32()),
            PersistenceId::WaterBallast => self.glider_data.water_ballast = Mass::from_kg(item.to_f32()),
            PersistenceId::PilotWeight => self.glider_data.pilot_weight = Mass::from_kg(item.to_f32()),
            PersistenceId::Glider => self.config.glider_idx = item.to_i32(),
            _ => (),
        }
    }
}


/// Flymode display variants
///
/// Flymode::Climbing is displayed while circling upwind. In addition to vario signal and wind,
/// the average climb is also displayed.
///
/// In Flymode::Gliding the mean climb is hidden and the speed command is displayed
/// graphically and as a number. Flymode::Transition is treatet like Flymode::Gliding
#[repr(u8)]
pub enum FlyMode {
    Circling,
    Transition,
    StraightFlight,
}

/// This enum is relevant for the View component. During Vario mode, information needed to
/// optimize climbing in thermals is displayed. SppedToFly, on the other hand, is intended
/// for optimal pre-flight.
#[repr(u8)]
pub enum VarioMode {
    Vario,
    SpeedToFly,
}

/// This determines how to switch between the two modes of VarioMode Vario and SpeedToFly:
/// automatic, manual Vario or manual SpeedToFly.
#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum VarioModeControl {
    Vario,
    SpeedToFly,
    Auto,
}

/// Enum mode controls whether the background should be visible or not when editing a data
/// point.
#[repr(u8)]
pub enum EditMode {
    Section,
    Fullscreen,
}

/// Possible displays
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum DisplayActive {
    Vario,
    FirmwareUpdate,
}

/// Metastructure for calculated or set values
pub struct Calculated {
    pub speed_to_fly: AirSpeed,
    pub speed_to_fly_dif: Speed,
    pub speed_to_fly_1s: Speed,     // ref. IAS
    pub thermal_climb_rate: Speed,
}

impl Default for Calculated {
    #[allow(unused)]
    fn default() -> Self {
        Calculated {
            speed_to_fly: AirSpeed::from_tas_at_nn(127.0.km_h()),
            speed_to_fly_dif: 3.0.km_h(),
            speed_to_fly_1s: 0.0.km_h(),
            thermal_climb_rate: 1.3.m_s(),
        }
    }
}

/// Metastructur for config variables
pub struct Config {
    pub display_active: DisplayActive,
    pub last_display_active: DisplayActive,
    pub glider_idx: i32,
    pub volume: i8,
    pub mc_cready: Speed,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_active: DisplayActive::Vario,
            last_display_active: DisplayActive::Vario,
            glider_idx: 104,
            volume: 0,
            mc_cready: 0.7.m_s(),
        }
    }
}

/// Metastructure for different control variables
pub struct Control {
    pub fly_mode: FlyMode,
    pub vario_mode: VarioMode,
    pub vario_mode_control: VarioModeControl,
    // Sets the switching point Vario/SpeedToFly in relation to speed of the best l/d ratio
    pub vario_mode_switch_ratio: f32,
    pub speed_to_fly_limit: Speed, // Speed limit above which SpeedToFly is activated
    pub edit_mode: EditMode,
    pub edit_var: Editable,
    pub edit_ticks: u32,   // Used by the editor for the timeout
    pub demo_acitve: bool, // Activates the demo mode
    pub firmware_update_state: DeviceEvent, 
}

impl Default for Control {
    fn default() -> Self {
        Self {
            fly_mode: FlyMode::Circling,
            vario_mode: VarioMode::Vario,
            vario_mode_control: VarioModeControl::Auto,
            vario_mode_switch_ratio: 1.05,
            speed_to_fly_limit: 105.0.km_h(),
            edit_mode: EditMode::Section,
            edit_var: Editable::ClimbRate,
            edit_ticks: 0,
            demo_acitve: false,
            firmware_update_state: DeviceEvent::UploadFinished,
        }
    }
}

/// Sensor Values
///
/// This structure contains all variables that are generated by the Larus sensor box.
pub struct Sensor {
    pub airspeed: AirSpeed,
    pub average_climb_rate: Speed,
    pub average_wind: WindVector,
    pub climb_rate: Speed,
    pub density: Density,
    pub g_force: Acceleration,
    pub gps_climb_rate: Speed,
    pub nick_angle: Angle,
    pub pressure: Pressure,
    pub slip_angle: Angle,
    pub turn_rate: AngularVelocity,
    pub vertical_g_force: Acceleration,
    pub wind_vector: WindVector,
}

impl Default for Sensor {
    #[allow(unused)]
    fn default() -> Self {
        Sensor {
            airspeed: AirSpeed::from_tas_at_nn(100.0.km_h()),
            average_climb_rate: 1.1.m_s(),
            average_wind: WindVector::new(15.0.km_h(), 80.0.deg()),
            climb_rate: 1.7.m_s(),
            density: Density::AT_NN(),
            g_force: 9.81.m_s2(),
            gps_climb_rate: 0.0.m_s(),
            nick_angle: 0.0.deg(),
            pressure: Pressure::AT_NN(),
            slip_angle: 0.0.deg(),
            turn_rate: 0.0.rad_s(),
            vertical_g_force: 9.81.m_s2(),
            wind_vector: WindVector::new(15.0.km_h(), 66.0.deg()),
        }
    }
}
