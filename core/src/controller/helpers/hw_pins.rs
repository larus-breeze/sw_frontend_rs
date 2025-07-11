use crate::{model::TypeOfInfo, CoreModel, FloatToMass, PinState, VarioMode};

pub const PIN_NONE: &str = "Not connected";
pub const PIN_IN_CLOSE: &str = "When closed";
pub const PIN_IN_OPEN: &str = "When opened";

#[derive(Clone, Copy, PartialEq)]
pub enum InPinFunction {
    None,
    OnClose,
    OnOpen,
}

impl From<u8> for InPinFunction {
    fn from(value: u8) -> Self {
        match value {
            1 => InPinFunction::OnClose,
            2 => InPinFunction::OnOpen,
            _ => InPinFunction::None,
        }
    }
}

impl From<&str> for InPinFunction {
    fn from(value: &str) -> Self {
        match value {
            PIN_IN_CLOSE => InPinFunction::OnClose,
            PIN_IN_OPEN => InPinFunction::OnOpen,
            _ => InPinFunction::None,
        }
    }
}

impl InPinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            InPinFunction::None => PIN_NONE,
            InPinFunction::OnClose => PIN_IN_CLOSE,
            InPinFunction::OnOpen => PIN_IN_OPEN,
        }
    }
}

pub const PIN_IN_TOGGLE: &str = "When toggled";

#[derive(Clone, Copy, PartialEq)]
pub enum InTogglePinFunction {
    None,
    OnClose,
    OnOpen,
    OnToggled,
}

impl From<u8> for InTogglePinFunction {
    fn from(value: u8) -> Self {
        match value {
            1 => InTogglePinFunction::OnClose,
            2 => InTogglePinFunction::OnOpen,
            3 => InTogglePinFunction::OnToggled,
            _ => InTogglePinFunction::None,
        }
    }
}

impl From<&str> for InTogglePinFunction {
    fn from(value: &str) -> Self {
        match value {
            PIN_IN_CLOSE => InTogglePinFunction::OnClose,
            PIN_IN_OPEN => InTogglePinFunction::OnOpen,
            PIN_IN_TOGGLE => InTogglePinFunction::OnToggled,
            _ => InTogglePinFunction::None,
        }
    }
}

impl InTogglePinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            InTogglePinFunction::None => PIN_NONE,
            InTogglePinFunction::OnClose => PIN_IN_CLOSE,
            InTogglePinFunction::OnOpen => PIN_IN_OPEN,
            InTogglePinFunction::OnToggled => PIN_IN_TOGGLE,
        }
    }
}

pub const PIN_OUT_CLOSE: &str = "Active: close";
pub const PIN_OUT_OPEN: &str = "Active: open";

#[derive(Clone, Copy, PartialEq)]
pub enum OutPinFunction {
    None,
    Closed,
    Opened,
}

impl From<u8> for OutPinFunction {
    fn from(value: u8) -> Self {
        match value {
            1 => OutPinFunction::Closed,
            2 => OutPinFunction::Opened,
            _ => OutPinFunction::None,
        }
    }
}

impl From<&str> for OutPinFunction {
    fn from(value: &str) -> Self {
        match value {
            PIN_OUT_CLOSE => OutPinFunction::Closed,
            PIN_OUT_OPEN => OutPinFunction::Opened,
            _ => OutPinFunction::None,
        }
    }
}

impl OutPinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutPinFunction::None => PIN_NONE,
            OutPinFunction::Closed => PIN_OUT_CLOSE,
            OutPinFunction::Opened => PIN_OUT_OPEN,
        }
    }
}

pub struct DrainControl {
    pin_function: InPinFunction,
    pin_state: PinState,
    is_flowing: bool,
    pub flow_rate_offset: f32, // flow rate [l/min] at 0kg ballast
    pub flow_rate_slope: f32,  // flow rate dif [l/min*s per l]
}

impl Default for DrainControl {
    fn default() -> Self {
        DrainControl {
            pin_function: InPinFunction::None,
            pin_state: PinState::High,
            is_flowing: false,
            flow_rate_offset: 30.0, // l/min
            flow_rate_slope: 0.0,   // l/min*kg*s
        }
    }
}

impl DrainControl {
    pub fn set_state(&mut self, cm: &mut CoreModel, pin_state: PinState) {
        self.pin_state = pin_state;
        self.adjust(cm);
    }

    pub fn tick_1s(&mut self, cm: &mut CoreModel) {
        if self.is_flowing {
            let flow_rate =
                self.flow_rate_offset + cm.glider_data.water_ballast.to_kg() * self.flow_rate_slope;
            cm.glider_data.water_ballast -= (flow_rate / 60.0).kg();
            if cm.glider_data.water_ballast.to_kg() <= 0.0 {
                cm.glider_data.water_ballast = 0.0.kg();
            }
            self.adjust(cm);
        }
    }

    pub fn is_flowing(&self) -> bool {
        self.is_flowing
    }

    pub fn pin_function(&self) -> InPinFunction {
        self.pin_function
    }

    pub fn set_pin_function(&mut self, pin_function: InPinFunction, cm: &mut CoreModel) {
        self.pin_function = pin_function;
        self.adjust(cm);
    }

    fn adjust(&mut self, cm: &mut CoreModel) {
        self.is_flowing = if cm.glider_data.water_ballast.to_kg() <= 0.0 {
            false
        } else {
            match self.pin_function {
                InPinFunction::OnClose => match self.pin_state {
                    PinState::High => false,
                    PinState::Low => true,
                },
                InPinFunction::OnOpen => match self.pin_state {
                    PinState::High => true,
                    PinState::Low => false,
                },
                InPinFunction::None => false,
            }
        };
        if self.is_flowing {
            if cm.config.info_active == TypeOfInfo::None {
                cm.config.info_active = TypeOfInfo::WaterBallast;
            }
        } else if cm.config.info_active == TypeOfInfo::WaterBallast {
            cm.config.info_active = TypeOfInfo::None
        }
    }
}

pub struct FlashControl {
    pub pin_function: OutPinFunction,
}

impl Default for FlashControl {
    fn default() -> Self {
        FlashControl {
            pin_function: OutPinFunction::None,
        }
    }
}

impl FlashControl {
    pub fn tick_1s(&mut self, cm: &mut CoreModel) -> Option<PinState> {
        match self.pin_function {
            OutPinFunction::None => None,
            OutPinFunction::Closed => {
                if cm.sensor.airspeed.ias().to_km_h() > 40.0 {
                    Some(PinState::High)
                } else {
                    Some(PinState::Low)
                }
            }
            OutPinFunction::Opened => {
                if cm.sensor.airspeed.ias().to_km_h() > 40.0 {
                    Some(PinState::Low)
                } else {
                    Some(PinState::High)
                }
            }
        }
    }

    pub fn pin_function(&self) -> OutPinFunction {
        self.pin_function
    }

    pub fn set_pin_function(&mut self, pin_function: OutPinFunction) {
        self.pin_function = pin_function;
    }
}

pub struct SpeedToFlyControl {
    pin_function: InTogglePinFunction,
    vario_mode: VarioMode,
}

impl Default for SpeedToFlyControl {
    fn default() -> Self {
        SpeedToFlyControl {
            pin_function: InTogglePinFunction::None,
            vario_mode: VarioMode::Vario,
        }
    }
}

impl SpeedToFlyControl {
    pub fn set_state(&mut self, pin_state: PinState) {
        self.vario_mode = match pin_state {
            PinState::High => match self.pin_function {
                InTogglePinFunction::None => return,
                InTogglePinFunction::OnClose => VarioMode::Vario,
                InTogglePinFunction::OnOpen => VarioMode::SpeedToFly,
                InTogglePinFunction::OnToggled => return,
            },
            PinState::Low => match self.pin_function {
                InTogglePinFunction::None => return,
                InTogglePinFunction::OnClose => VarioMode::SpeedToFly,
                InTogglePinFunction::OnOpen => VarioMode::Vario,
                InTogglePinFunction::OnToggled => !self.vario_mode,
            },
        }
    }

    pub fn pin_function(&self) -> InTogglePinFunction {
        self.pin_function
    }

    pub fn set_pin_function(&mut self, pin_function: InTogglePinFunction) {
        self.pin_function = pin_function;
    }

    pub fn vario_mode(&self) -> VarioMode {
        self.vario_mode
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GearPins {
    OnePinMode,
    TwoPinMode,
}

pub const ONE_PIN_MODE: &str = "One Pin Mode";
pub const TWO_PIN_MODE: &str = "Two Pin Mode";

impl GearPins {
    pub fn as_str(&self) -> &'static str {
        match self {
            GearPins::OnePinMode => ONE_PIN_MODE,
            GearPins::TwoPinMode => TWO_PIN_MODE,
        }
    }
}

impl From<u8> for GearPins {
    fn from(value: u8) -> Self {
        match value {
            1 => GearPins::TwoPinMode,
            _ => GearPins::OnePinMode,
        }
    }
}

impl From<&str> for GearPins {
    fn from(value: &str) -> Self {
        match value {
            ONE_PIN_MODE => GearPins::OnePinMode,
            _ => GearPins::TwoPinMode,
        }
    }
}

pub struct GearAlarmControl {
    gear_pins: GearPins,
    pin_gear_or_both_function: InPinFunction,
    pin_airbrakes_function: InPinFunction,
    gear_state: bool,
    airbrakes_state: bool,
}

impl Default for GearAlarmControl {
    fn default() -> Self {
        GearAlarmControl {
            gear_pins: GearPins::TwoPinMode,
            pin_gear_or_both_function: InPinFunction::None,
            pin_airbrakes_function: InPinFunction::None,
            gear_state: false,
            airbrakes_state: false,
        }
    }
}

impl GearAlarmControl {
    // sets the pin state and returns alarm active true/false
    pub fn set_gear_pin_state(&mut self, cm: &mut CoreModel, state: PinState) -> bool {
        self.gear_state = match self.pin_gear_or_both_function {
            InPinFunction::None => false,
            InPinFunction::OnClose => match state {
                PinState::High => false,
                PinState::Low => true,
            },
            InPinFunction::OnOpen => match state {
                PinState::High => true,
                PinState::Low => false,
            },
        };
        self.alarm_is_active(cm)
    }

    pub fn gear_pin_function(&self) -> InPinFunction {
        self.pin_gear_or_both_function
    }

    pub fn set_gear_pin_function(&mut self, function: InPinFunction) {
        self.pin_gear_or_both_function = function;
    }

    pub fn set_airbrakes_pin_state(&mut self, cm: &mut CoreModel, state: PinState) -> bool {
        self.airbrakes_state = match self.pin_airbrakes_function {
            InPinFunction::None => false,
            InPinFunction::OnClose => match state {
                PinState::High => false,
                PinState::Low => true,
            },
            InPinFunction::OnOpen => match state {
                PinState::High => true,
                PinState::Low => false,
            },
        };
        self.alarm_is_active(cm)
    }

    pub fn airbrakes_pin_function(&self) -> InPinFunction {
        self.pin_airbrakes_function
    }

    pub fn set_airbrakes_pin_function(&mut self, function: InPinFunction) {
        self.pin_airbrakes_function = function;
    }

    pub fn gear_pin_mode(&self) -> GearPins {
        self.gear_pins
    }

    pub fn set_gear_pin_mode(&mut self, mode: GearPins) {
        self.gear_pins = mode;
    }

    fn alarm_is_active(&self, cm: &mut CoreModel) -> bool {
        let alarm = match self.gear_pins {
            GearPins::OnePinMode => self.gear_state,
            GearPins::TwoPinMode => self.gear_state && self.airbrakes_state,
        };
        if alarm {
            cm.config.info_active = TypeOfInfo::GearAlarm;
        } else if cm.config.info_active == TypeOfInfo::GearAlarm {
            cm.config.info_active = TypeOfInfo::None;
        }
        alarm
    }
}
