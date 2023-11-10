use heapless::String;

#[allow(unused_imports)]
use micromath::F32Ext;

const DECIMAL_SIGN: u8 = 10;
const LOOKUP: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

/// A simple tool to put together text outputs in a formatted way.
#[derive(Default)]
pub struct Concat<const CAP: usize> {
    str: String<CAP>,
}

impl<const CAP: usize> Concat<CAP> {
    pub fn new() -> Self {
        Concat {
            str: String::<CAP>::default(),
        }
    }

    pub fn push_str(mut self, s: &str) -> Self {
        let _ = self.str.push_str(s);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let r = Concat::new();
        r.push_str(s)
    }

    pub fn push_u32(mut self, u: u32) -> Self {
        let mut idx: usize = 0;
        let mut u = u;
        let mut buf = [0u8; CAP];

        // Convert to numbers (invers)
        loop {
            buf[idx] = (u % 10) as u8;
            u /= 10;
            idx += 1;
            if u == 0 {
                break;
            }
        }

        // Add to string
        for r_idx in (0..idx).rev() {
            let _ = self.str.push(LOOKUP[buf[r_idx] as usize]);
        }
        self
    }

    pub fn push_u16(self, u: u16) -> Self {
        self.push_u32(u as u32)
    }

    pub fn push_u8(self, u: u8) -> Self {
        self.push_u32(u as u32)
    }

    pub fn from_u32(u: u32) -> Self {
        let r = Self::new();
        r.push_u32(u)
    }

    pub fn from_u16(u: u16) -> Self {
        let r = Self::new();
        r.push_u16(u)
    }

    pub fn from_u8(u: u8) -> Self {
        let r = Self::new();
        r.push_u8(u)
    }

    pub fn push_i32(mut self, i: i32) -> Self {
        if i < 0 {
            let _ = self.str.push('-');
            self.push_u32(i.unsigned_abs())
        } else {
            self.push_u32(i as u32)
        }
    }

    pub fn push_i16(self, i: i16) -> Self {
        self.push_i32(i as i32)
    }

    pub fn push_i8(self, i: i8) -> Self {
        self.push_i32(i as i32)
    }

    pub fn from_i32(i: i32) -> Self {
        let r = Self::new();
        r.push_i32(i)
    }

    pub fn from_i16(i: i16) -> Self {
        let r = Self::new();
        r.push_i16(i)
    }

    pub fn from_i8(i: i8) -> Self {
        let r = Self::new();
        r.push_i8(i)
    }

    pub fn push_f32(mut self, f: f32, decimal_places: usize) -> Self {
        // Shift decimal placese in front of decimal point
        let mut f = f;
        for _ in 0..decimal_places {
            f *= 10.0;
        }

        // Set sign if necessary
        if f < -0.4999 {
            let _ = self.str.push('-');
        }

        // Create the inverted pattern
        let mut u = (f.abs() + 0.5) as u32;
        let mut idx: usize = 0;
        let mut buf = [0u8; CAP];
        let stop_idx = if decimal_places == 0 {
            0
        } else {
            decimal_places + 1
        };

        loop {
            buf[idx] = (u % 10) as u8;
            u /= 10;
            idx += 1;
            if idx == decimal_places {
                buf[idx] = DECIMAL_SIGN;
                idx += 1;
            }
            if (u == 0) && (idx > stop_idx) {
                break;
            }
        }

        // Add pattern to string buffer
        for r_idx in (0..idx).rev() {
            let _ = self.str.push(LOOKUP[buf[r_idx] as usize]);
        }
        self
    }

    pub fn from_f32(f: f32, decimal_places: usize) -> Self {
        let r = Self::new();
        r.push_f32(f, decimal_places)
    }

    pub fn as_str(&self) -> &str {
        self.str.as_str()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.str.as_bytes()
    }

    pub fn as_string(&self) -> String<CAP> {
        self.str.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u_to_str() {
        assert_eq!(Concat::<20>::from_u32(0).as_str(), "0");
        assert_eq!(Concat::<20>::from_u32(123).as_str(), "123");
        assert_eq!(Concat::<20>::from_u32(4294967295).as_str(), "4294967295");
    }

    #[test]
    fn test_i_to_str() {
        assert_eq!(Concat::<20>::from_i32(0).as_str(), "0");
        assert_eq!(Concat::<20>::from_i32(123).as_str(), "123");
        assert_eq!(Concat::<20>::from_i32(2147483647).as_str(), "2147483647");
        assert_eq!(Concat::<20>::from_i32(-123).as_str(), "-123");
        assert_eq!(Concat::<20>::from_i32(-2147483647).as_str(), "-2147483647");
    }

    #[test]
    fn test_f32_to_str() {
        assert_eq!(Concat::<20>::from_f32(0.0, 0).as_str(), "0");
        assert_eq!(Concat::<20>::from_f32(883.0, 0).as_str(), "883");
        assert_eq!(Concat::<20>::from_f32(-0.01, 1).as_str(), "0.0");
        assert_eq!(Concat::<20>::from_f32(-0.1, 1).as_str(), "-0.1");
        assert_eq!(Concat::<20>::from_f32(-0.0127, 4).as_str(), "-0.0127");
        assert_eq!(Concat::<20>::from_f32(0.51, 0).as_str(), "1");
        assert_eq!(Concat::<20>::from_f32(-0.51, 0).as_str(), "-1");
        assert_eq!(Concat::<20>::from_f32(0.051, 1).as_str(), "0.1");
        assert_eq!(Concat::<20>::from_f32(-0.051, 1).as_str(), "-0.1");
        assert_eq!(Concat::<20>::from_f32(0.0281, 4).as_str(), "0.0281");
        assert_eq!(Concat::<20>::from_f32(100000.0, 3).as_str(), "100000.000");
        assert_eq!(Concat::<20>::from_f32(-123.45, 2).as_str(), "-123.45");
        assert_eq!(Concat::<20>::from_f32(-100000.0, 3).as_str(), "-100000.000");
    }
}
