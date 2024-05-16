#[derive(Clone, Copy)]
pub struct Device {
    pub supply_voltage: f32,
    pub illumination_voltage: f32,
    pub temperature_pcb: f32,

    pub voltage_limit_bad: f32,
    pub voltage_limit_good: f32,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            supply_voltage: 13.0,
            illumination_voltage: 0.0,
            temperature_pcb: 0.0,

            voltage_limit_bad: 10.5,
            voltage_limit_good: 12.0,
        }
    }
}
