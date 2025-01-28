use core::{default::Default, f32::consts::PI};

use crate::{Colors, CoreModel, VarioMode};

#[allow(unused_imports)]
use micromath::F32Ext;

pub const THERMAL_DATA_CNT: usize = 24;
pub const DELTA_ALPHA: f32 = 2.0 * PI / THERMAL_DATA_CNT as f32;

#[derive(PartialEq)]
pub struct ThermalData {
    climb_data: [f32; THERMAL_DATA_CNT],
    last_vario_mode: VarioMode,
    last_tick: u32,

    best_pos: usize,
}

impl ThermalData {
    pub fn update(&mut self, cm: &CoreModel) {
        if cm.control.alive_ticks == self.last_tick {
            return;
        }
        self.last_tick = cm.control.alive_ticks;

        if cm.control.vario_mode != self.last_vario_mode {
            if cm.control.vario_mode == VarioMode::Vario {
                for climb_data in &mut self.climb_data {
                    *climb_data = 0.0;
                }
            }
            self.last_vario_mode = cm.control.vario_mode;
        }
        if cm.control.vario_mode == VarioMode::SpeedToFly {
            return;
        }

        let idx = Self::get_idx(cm.sensor.euler_yaw.to_radians());
        self.climb_data[idx] = (cm.sensor.climb_rate - cm.calculated.av2_climb_rate).to_m_s();
    }

    fn get_idx(alpha: f32) -> usize {
        (alpha.rem_euclid(2.0 * PI) / DELTA_ALPHA) as usize
    }

    pub fn prepare(&mut self) {
        self.best_pos = 0;
        let mut value = self.climb_data[0];
        for idx in 1..THERMAL_DATA_CNT {
            if self.climb_data[idx] > value {
                value = self.climb_data[idx];
                self.best_pos = idx;
            }
        }
    }

    pub fn get_dotted_item(&mut self, alpha: f32, cm: &CoreModel) -> (Colors, f32) {
        let idx = Self::get_idx(alpha);
        let color = if idx == self.best_pos {
            cm.palette().therm_ass_best
        } else if self.climb_data[idx] > 0.0 {
            cm.palette().therm_ass_good
        } else {
            cm.palette().therm_ass_bad
        };
        let value = self.climb_data[idx];
        (color, value)
    }

    pub fn get_spider_item(&mut self, alpha: f32, cm: &CoreModel) -> (Colors, f32) {
        let idx = Self::get_idx(alpha);
        let color = if idx == self.best_pos {
            cm.palette().therm2_ass_best
        } else if self.climb_data[idx] > 0.0 {
            cm.palette().therm2_ass_good
        } else {
            cm.palette().therm2_ass_bad
        };
        let value = self.climb_data[idx];
        (color, value)
    }
}

impl Default for ThermalData {
    fn default() -> Self {
        ThermalData {
            climb_data: [0.0; THERMAL_DATA_CNT],
            last_vario_mode: VarioMode::SpeedToFly,
            last_tick: 0,
            best_pos: 0,
        }
    }
}
