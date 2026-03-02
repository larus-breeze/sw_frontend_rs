use core::f32::consts::PI;
use micromath::F32Ext;

const BINS: usize = 24;
const TWO_PI: f32 = 2.0 * PI;

#[derive(Clone, Copy)]
pub struct CircleStats {
    bins: [f32; BINS],
    visited: [bool; BINS],
    last_heading: Option<f32>,
    accumulated_turn_abs: f32,
    active: bool,
    last_full_circle_max_min: Option<f32>,
}

impl Default for CircleStats {
    fn default() -> Self {
        Self {
            bins: [0.0; BINS],
            visited: [false; BINS],
            last_heading: None,
            accumulated_turn_abs: 0.0,
            active: false,
            last_full_circle_max_min: None,
        }
    }
}

impl CircleStats {
    pub fn reset_full(&mut self) {
        self.bins = [0.0; BINS];
        self.visited = [false; BINS];
        self.last_heading = None;
        self.accumulated_turn_abs = 0.0;
        self.active = false;
        self.last_full_circle_max_min = None;
    }

    fn reset_window(&mut self) {
        self.bins = [0.0; BINS];
        self.visited = [false; BINS];
        self.accumulated_turn_abs = 0.0;
    }

    fn wrap_pi(mut x: f32) -> f32 {
        while x > PI {
            x -= TWO_PI;
        }
        while x < -PI {
            x += TWO_PI;
        }
        x
    }

    fn idx(alpha: f32) -> usize {
        let a = alpha.rem_euclid(TWO_PI);
        ((a / TWO_PI) * BINS as f32) as usize
    }

    fn update_bin(&mut self, yaw_rad: f32, climb_delta: f32) {
        let i = Self::idx(yaw_rad).min(BINS - 1);
        self.bins[i] = climb_delta;
        self.visited[i] = true;
    }

    fn calc_delta(&self) -> Option<f32> {
        let mut first = true;
        let mut min_v = 0.0f32;
        let mut max_v = 0.0f32;
        let mut count = 0usize;

        for i in 0..BINS {
            if self.visited[i] {
                let v = self.bins[i];
                if first {
                    min_v = v;
                    max_v = v;
                    first = false;
                } else {
                    if v < min_v {
                        min_v = v;
                    }
                    if v > max_v {
                        max_v = v;
                    }
                }
                count += 1;
            }
        }

        if count >= (BINS / 2) {
            Some(max_v - min_v)
        } else {
            None
        }
    }

    pub fn update(&mut self, yaw_rad: f32, climb_delta: f32, is_circling: bool) -> Option<f32> {
        if !is_circling {
            self.reset_full();
            return self.last_full_circle_max_min;
        }

        if !self.active {
            self.active = true;
            self.reset_window();
            self.last_full_circle_max_min = None; // show "--" until first full circle is done
            self.last_heading = Some(yaw_rad);
            self.update_bin(yaw_rad, climb_delta);
            return self.last_full_circle_max_min;
        }

        if let Some(last) = self.last_heading {
            let dyaw = Self::wrap_pi(yaw_rad - last);
            self.accumulated_turn_abs += dyaw.abs();
        }
        self.last_heading = Some(yaw_rad);
        self.update_bin(yaw_rad, climb_delta);

        if self.accumulated_turn_abs >= TWO_PI {
            self.last_full_circle_max_min = self.calc_delta();
            self.reset_window();
            self.last_heading = Some(yaw_rad);
        }

        self.last_full_circle_max_min
    }
}
