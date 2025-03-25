use crate::PinState;

#[derive(Clone, Copy)]
pub struct Device {
    pub supply_voltage: f32,
    pub illumination_voltage: f32,
    pub temperature_pcb: f32,
    pub io1: PinState,
    pub io2: PinState,
    pub io3: PinState,
    pub io4: PinState,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            supply_voltage: 13.0,
            illumination_voltage: 0.0,
            temperature_pcb: 0.0,
            io1: PinState::High,
            io2: PinState::High,
            io3: PinState::High,
            io4: PinState::High,
        }
    }
}
