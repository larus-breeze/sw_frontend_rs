#[derive(Clone, Copy)]
pub struct Device {
    pub supply_voltage: f32,
    pub illumination_voltage: f32,
    pub temperature_pcb: f32,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            supply_voltage: 13.0,
            illumination_voltage: 0.0,
            temperature_pcb: 0.0,
        }
    }
}
