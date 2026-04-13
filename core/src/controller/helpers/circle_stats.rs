use core::f32::consts::PI;
use micromath::F32Ext;

const BINS: usize = 24;
const TWO_PI: f32 = 2.0 * PI;

#[derive(Clone, Copy)]
pub struct CircleStats {
    max_min_bins: [f32; BINS],
    max_min_counts: [u8; BINS],
    visited: [bool; BINS],
    diameter_window: [f32; BINS],
    diameter_sum: f32,
    diameter_idx: usize,
    diameter_count: usize,
}

impl Default for CircleStats {
    fn default() -> Self {
        Self {
            max_min_bins: [0.0; BINS],
            max_min_counts: [0; BINS],
            visited: [false; BINS],
            diameter_window: [0.0; BINS],
            diameter_sum: 0.0,
            diameter_idx: 0,
            diameter_count: 0,
        }
    }
}

impl CircleStats {
    pub fn reset_full(&mut self) {
        self.max_min_bins = [0.0; BINS];
        self.max_min_counts = [0; BINS];
        self.visited = [false; BINS];
        self.diameter_window = [0.0; BINS];
        self.diameter_sum = 0.0;
        self.diameter_idx = 0;
        self.diameter_count = 0;
    }

    fn idx(alpha: f32) -> usize {
        let a = alpha.rem_euclid(TWO_PI);
        ((a / TWO_PI) * BINS as f32) as usize
    }

    fn update_bin_avg(&mut self, yaw_rad: f32, climb_delta: f32) {
        let i = Self::idx(yaw_rad).min(BINS - 1);
        self.visited[i] = true;

        let cnt = self.max_min_counts[i];
        if cnt == 0 {
            self.max_min_bins[i] = climb_delta;
            self.max_min_counts[i] = 1;
            return;
        }

        let n = cnt as f32;
        // Cap effective history to 24 to keep this responsive as a rolling average.
        let eff_n = n.min(BINS as f32);
        self.max_min_bins[i] += (climb_delta - self.max_min_bins[i]) / eff_n;
        if self.max_min_counts[i] < BINS as u8 {
            self.max_min_counts[i] += 1;
        }
    }

    fn calc_delta_live(&self) -> Option<f32> {
        let mut found = false;
        let mut min_v = 0.0;
        let mut max_v = 0.0;

        for i in 0..BINS {
            if !self.visited[i] {
                continue;
            }

            let v = self.max_min_bins[i];
            if !found {
                min_v = v;
                max_v = v;
                found = true;
            } else {
                if v < min_v {
                    min_v = v;
                }
                if v > max_v {
                    max_v = v;
                }
            }
        }

        if found {
            Some(max_v - min_v)
        } else {
            None
        }
    }

    pub fn update_max_min(&mut self, yaw_rad: f32, climb_delta: f32, is_circling: bool) -> Option<f32> {
        if !is_circling {
            self.max_min_bins = [0.0; BINS];
            self.max_min_counts = [0; BINS];
            self.visited = [false; BINS];
            return None;
        }

        self.update_bin_avg(yaw_rad, climb_delta);
        self.calc_delta_live()
    }

    pub fn update_diameter(&mut self, diameter_m: Option<f32>, is_circling: bool) -> Option<f32> {
        if !is_circling {
            self.diameter_window = [0.0; BINS];
            self.diameter_sum = 0.0;
            self.diameter_idx = 0;
            self.diameter_count = 0;
            return None;
        }

        let value = diameter_m?;

        if self.diameter_count < BINS {
            self.diameter_window[self.diameter_idx] = value;
            self.diameter_sum += value;
            self.diameter_count += 1;
        } else {
            self.diameter_sum -= self.diameter_window[self.diameter_idx];
            self.diameter_window[self.diameter_idx] = value;
            self.diameter_sum += value;
        }

        self.diameter_idx = (self.diameter_idx + 1) % BINS;

        if self.diameter_count < BINS {
            None
        } else {
            Some(self.diameter_sum / BINS as f32)
        }
    }
}
