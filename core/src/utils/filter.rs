use core::ops::{Add, Mul, Sub};

pub struct Pt1<T>
where
    T: Mul<f32, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    value: T,
    factor: f32,
}

impl<T> Pt1<T>
where
    T: Mul<f32, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy,
{
    pub fn new(value: T, tick_rate: u32, time_const: f32) -> Self {
        let factor = Pt1::<T>::calc_factor(tick_rate, time_const);
        Pt1 { value, factor }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn set_time_const(&mut self, tick_rate: u32, time_const: f32) {
        self.factor = Pt1::<T>::calc_factor(tick_rate, time_const);
    }

    pub fn tick(&mut self, new_value: T) {
        self.value = self.value + (new_value - self.value) * self.factor;
    }

    pub fn value(&self) -> T {
        self.value
    }

    #[inline]
    fn calc_factor(tick_rate: u32, time_const: f32) -> f32 {
        1.0 / (time_const * tick_rate as f32)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;

    use super::*;

    #[test]
    fn test_pt1() {
        let mut pt1 = Pt1::new(0.0_f32, 10, 1.0);

        for _ in 0..10 {
            pt1.tick(1.0);
        }
        assert_float_eq!(pt1.value(), 0.651);

        pt1.set_value(0.0);
        assert_float_eq!(pt1.value(), 0.0);

        pt1.set_time_const(10, 2.0);
        for _ in 0..20 {
            pt1.tick(1.0);
        }
        assert_float_eq!(pt1.value(), 0.641);
    }
}
