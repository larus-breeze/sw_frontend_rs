use crate::PinState;

#[derive(Clone, Copy)]
pub struct Device {
    pub supply_voltage: f32,
    pub illumination_voltage: f32,
    pub temperature_pcb: f32,
    pub in1: PinState,
    pub in2: PinState,
    pub in3: PinState,
    pub in4: PinState,
    pub out1: PinState,
    pub out2: PinState,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            supply_voltage: 13.0,
            illumination_voltage: 0.0,
            temperature_pcb: 0.0,
            in1: PinState::High,
            in2: PinState::High,
            in3: PinState::High,
            in4: PinState::High,
            out1: PinState::High,
            out2: PinState::High,
        }
    }
}
