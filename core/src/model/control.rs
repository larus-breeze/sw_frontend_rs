use crate::{
    controller::{CanActive, Editor},
    model::CoreModel,
    system_of_units::{FloatToLength, FloatToSpeed, Length, Speed},
    utils::DeviceEvent,
    MenuControl, Rotation,
};

/// Flymode display variants
///
/// Flymode::Climbing is displayed while circling upwind. In addition to vario signal and wind,
/// the average climb is also displayed.
///
/// In Flymode::Gliding the mean climb is hidden and the speed command is displayed
/// graphically and as a number. Flymode::Transition is treatet like Flymode::Gliding
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum FlyMode {
    Circling,
    StraightFlight,
}

/// This enum is relevant for the View component. During Vario mode, information needed to
/// optimize climbing in thermals is displayed. SppedToFly, on the other hand, is intended
/// for optimal pre-flight.
#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum VarioMode {
    Vario,
    SpeedToFly,
}

impl core::ops::Not for VarioMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            VarioMode::Vario => VarioMode::SpeedToFly,
            VarioMode::SpeedToFly => VarioMode::Vario
        }
    }
}

/// This determines how to switch between the two modes of VarioMode Vario and SpeedToFly:
/// automatic, manual Vario or manual SpeedToFly.
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VarioModeControl {
    Auto,
    InputPin,
    Nmea,
}

pub const VARIO_MODE_CONTROL_AUTO: &str = "Auto";
pub const VARIO_MODE_CONTROL_PIN: &str = "Input Pin";
pub const VARIO_MODE_CONTROL_NMEA: &str = "NMEA";


impl From<u8> for VarioModeControl {
    fn from(value: u8) -> Self {
        match value {
            1 => VarioModeControl::InputPin,
            2 => VarioModeControl::Nmea,
            _ => VarioModeControl::Auto,
        }
    }
}

impl From<&str> for VarioModeControl {
    fn from(value: &str) -> Self {
        match value {
            VARIO_MODE_CONTROL_NMEA => VarioModeControl::Nmea,
            VARIO_MODE_CONTROL_PIN => VarioModeControl::InputPin,
            _ => VarioModeControl::Auto,
        }
    }
}

impl VarioModeControl {
    pub fn as_str(&self) -> &'static str {
        match self {
            VarioModeControl::Auto => VARIO_MODE_CONTROL_AUTO,
            VarioModeControl::InputPin => VARIO_MODE_CONTROL_PIN,
            VarioModeControl::Nmea => VARIO_MODE_CONTROL_NMEA,
        }
    }
}

/// Enum mode controls whether the background should be visible or not when editing a data
/// point.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum EditMode {
    Off,
    Window,
    Fullscreen,
    CircleArea,
}

/// Enum for calculation of thermal climb rate
#[derive(Clone, Copy, PartialEq)]
pub enum TcrMode {
    StraightFlight,
    Transition,
    Climbing,
}

/// Enum for Larus System State
#[derive(Clone, Copy, PartialEq)]
pub enum SystemState {
    NoCom,
    CanOk,
    CanAndGpsOk,
}

/// Metastructure for different control variables
#[derive(Clone, Copy)]
pub struct Control {
    /// Count ticks the firmware is alive (100ms)
    pub alive_ticks: u32,
    /// Count ticks average climb rates slave mode is active (1s)
    pub avg_climb_slave_ticks: u16,
    /// State of the Larus system
    pub system_state: SystemState,
    /// Bit pattern of all can bus devices
    pub can_devices: u32,
    /// FlyMode::Circling, FlyMode::StraightFlight
    pub fly_mode: FlyMode,
    /// VarioMode::Vario, VarioMode::SpeedToFly
    pub vario_mode: VarioMode,
    /// VarioMode::Vario, VarioMode::SpeedToFly, VarioMode::Auto
    pub vario_mode_control: VarioModeControl,
    /// Sets the switching point Vario/SpeedToFly in relation to speed of the best l/d ratio
    pub vario_mode_switch_ratio: f32,
    /// Speed limit above which SpeedToFly is activated
    pub speed_to_fly_limit: Speed,
    /// EditMode::Section, EditMode::Fullscreen
    pub pers_ticks: u32,
    /// DeviceEvent::FwAvailable, PrepareFwUpload, ...
    pub firmware_update_state: DeviceEvent,
    /// TcrMode::StraightFlight, TcrMode::Transition, TcrMode::Climbing
    pub tcr_mode: TcrMode,
    /// Measurement of time climbing
    pub tcr_1s_climb_ticks: u32,
    /// Measurement of time transient climb <-> straigt flight
    pub tcr_1s_transient_ticks: u32,
    /// Height at the beginning of the climb
    pub tcr_start: Length,
    /// General Editor
    pub editor: Editor,
    /// Menu Control
    pub menu_control: MenuControl,
    /// Screen Rotation
    pub rotation: Rotation,
    /// Reset Config
    pub reset_config: i8,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            alive_ticks: 0,
            avg_climb_slave_ticks: 0,
            system_state: SystemState::NoCom,
            can_devices: CanActive::None as u32,
            fly_mode: FlyMode::StraightFlight,
            vario_mode: VarioMode::Vario,
            vario_mode_control: VarioModeControl::Auto,
            vario_mode_switch_ratio: 1.10,
            speed_to_fly_limit: 105.0.km_h(),
            pers_ticks: 0,
            firmware_update_state: DeviceEvent::UploadFinished,
            tcr_mode: TcrMode::StraightFlight,
            tcr_1s_climb_ticks: 0,
            tcr_1s_transient_ticks: 0,
            tcr_start: 0.0.m(),
            editor: Editor::new(),
            menu_control: MenuControl::new(),
            rotation: Rotation::Rotate0,
            reset_config: 0,
        }
    }
}

impl CoreModel {
    pub fn set_vario_mode(&mut self, vario_mode: VarioMode, source: VarioModeControl) {
        if source == self.control.vario_mode_control {
            self.control.vario_mode = vario_mode
        }
    }
}
