use crate::{CoreModel, FloatToMass, PinState};

pub const PIN_NONE: &str = "Not connected";
pub const PIN_CLOSE: &str = "When closed";
pub const PIN_OPEN: &str = "When opened";

#[derive(Clone, Copy, PartialEq)]
pub enum PinFunction {
    None,
    OnClose,
    OnOpen,
}

impl From<u8> for PinFunction {
    fn from(value: u8) -> Self {
        match value {
            1 => PinFunction::OnClose,  
            2 => PinFunction::OnOpen,  
            _ => PinFunction::None,  
        }
    }
}

impl From<&str> for PinFunction {
    fn from(value: &str) -> Self {
        match value {
            PIN_CLOSE => PinFunction::OnClose,  
            PIN_OPEN => PinFunction::OnOpen,  
            _ => PinFunction::None,  
        }
    }
}

impl PinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            PinFunction::None => PIN_NONE,
            PinFunction::OnClose => PIN_CLOSE,
            PinFunction::OnOpen => PIN_OPEN,
        }
    }
}

pub struct DrainControl {
    pin_function: PinFunction,
    pin_state: PinState,
    is_flowing: bool,
    pub flow_rate_offset: f32, // flow rate [l/min] at 0kg ballast
    pub flow_rate_slope: f32, // flow rate dif [l/min*s per l]
}

impl Default for DrainControl {
    fn default() -> Self {
        DrainControl { 
            pin_function: PinFunction::None, 
            pin_state: PinState::High, 
            is_flowing: false, 
            flow_rate_offset: 30.0, // l/min
            flow_rate_slope: 0.0,  // l/min*kg*s
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
            let flow_rate = self.flow_rate_offset + 
                cm.glider_data.water_ballast.to_kg() * self.flow_rate_slope;
            cm.glider_data.water_ballast -= (flow_rate / 60.0).kg();
            if cm.glider_data.water_ballast.to_kg() < 0.0 {
                cm.glider_data.water_ballast = 0.0.kg();
            }
            self.adjust(cm);
        }
    }

    pub fn pin_function(&self) -> PinFunction {
        self.pin_function
    }

    pub fn set_pin_function(&mut self, pin_function: PinFunction, cm: &CoreModel) {
        self.pin_function = pin_function;
        self.adjust(cm);
    }

    fn adjust(&mut self, cm: &CoreModel) {
        self.is_flowing = if cm.glider_data.water_ballast.to_kg() == 0.0 {
            false
        } else {
            match self.pin_function {
                PinFunction::OnClose => match self.pin_state {
                    PinState::High => false,
                    PinState::Low => true,
                }
                PinFunction::OnOpen => match self.pin_state {
                    PinState::High => true,
                    PinState::Low => false,
                }
                PinFunction::None => false,
            }
        }
    }
}

