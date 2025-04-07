use crate::{CoreModel, FloatToMass, PinState, model::{OverlayActive, TypeOfInfo}};

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
    pub flow_rate_slope: f32, // flow rate dif [l/min*s per l]
}

impl Default for DrainControl {
    fn default() -> Self {
        DrainControl { 
            pin_function: InPinFunction::None, 
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
                }
                InPinFunction::OnOpen => match self.pin_state {
                    PinState::High => true,
                    PinState::Low => false,
                }
                InPinFunction::None => false,
            }
        };
        if self.is_flowing {
            if cm.config.overlay_active == OverlayActive::None {
                cm.config.overlay_active = OverlayActive::Info(TypeOfInfo::WaterBallast);
            }
        } else {
            match cm.config.overlay_active {
                OverlayActive::Info(_) => cm.config.overlay_active = OverlayActive::None,
                _ => (),
            }
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
            OutPinFunction::Closed => if cm.sensor.airspeed.ias().to_km_h() > 40.0 {
                Some(PinState::Low)
            } else {
                Some(PinState::High)
            }
            OutPinFunction::Opened => if cm.sensor.airspeed.ias().to_km_h() > 40.0 {
                Some(PinState::High)
            } else {
                Some(PinState::Low)
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
