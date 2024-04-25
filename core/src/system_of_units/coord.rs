#[allow(unused_imports)]
use micromath::F32Ext;
#[allow(unused_imports)]
use num::traits::float::FloatCore;
use tfmt::{uDisplayFormatted, uWrite, Formatter, Padding, Convert};
use core::f64::consts::PI;

#[allow(dead_code)]
const RAD_PER_DEGREE: f64 = PI / 180.0;
#[allow(dead_code)]
const DEGREE_PER_RAD: f64 = 180.0 / PI;

#[derive(Copy, Clone, Default)]
pub struct Coord(pub f64);

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Coord {
    /// Create an instance of type Coord from a f64 number in RAD per s
    #[inline]
    pub fn from_rad(value: f64) -> Self {
        Coord(value)
    }

    /// Create an instance of type Coord from a f64 number in DEG per s
    #[inline]
    pub fn from_deg(value: f64) -> Self {
        Coord(value * RAD_PER_DEGREE)
    }

    /// Extract a f64 number in the unit RAD per s
    #[inline]
    pub fn to_rad(self) -> f64 {
        self.0
    }

    /// Extract a f64 number in the unit DEG per s
    #[inline]
    pub fn to_deg(self) -> f64 {
        self.0 * DEGREE_PER_RAD
    }
}

/// Trait to convert data to the struct [Coord]
#[allow(non_snake_case)]
pub trait F64ToCoord {
    /// Create an instance of type [AnglularVelocity] from a number RAD per s
    fn rad(self) -> Coord;
    /// Create an instance of type [AnglularVelocity] from a number DEG per s
    fn deg(self) -> Coord;
}

impl F64ToCoord for f64 {
    #[inline]
    fn rad(self) -> Coord {
        Coord::from_rad(self)
    }
    #[inline]
    fn deg(self) -> Coord {
        Coord::from_deg(self)
    }
}

pub struct Latitude(pub Coord);

impl uDisplayFormatted for Latitude {
    fn fmt_formatted<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        _prefix: bool,
        cmd: char,
        _padding: Padding,
        _pad_char: char,
        _behind: usize
    ) -> Result<(), W::Error>
       where W: uWrite + ?Sized {
        if cmd == 'n' {
            let degs = self.0.to_deg();
            let (degs, sign) = if degs.is_sign_positive() {
                (degs, b'N')
            } else {
                (-degs, b'S')
            };
            let mins = degs.fract() * 60.0;
            let mut conv = Convert::<15>::new(b'0');
            conv.write_u8(sign).unwrap();
            conv.write_u8(b',').unwrap();
            conv.f64_pad(mins, 8, 5).unwrap();
            conv.u32(degs as u32).unwrap();
            fmt.write_str(conv.as_str())
        } else {
            fmt.write_str("FormatError")
        }
    }
}
pub struct Longitude(pub Coord);

impl uDisplayFormatted for Longitude {
    fn fmt_formatted<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        _prefix: bool,
        cmd: char,
        _padding: Padding,
        _pad_char: char,
        _behind: usize
    ) -> Result<(), W::Error>
       where W: uWrite + ?Sized {
        if cmd == 'n' {
            let degs = self.0.to_deg();
            let (degs, sign) = if degs.is_sign_positive() {
                (degs, b'E')
            } else {
                (-degs, b'W')
            };
            let mins = degs.fract() * 60.0;
            let mut conv = Convert::<15>::new(b'0');
            conv.write_u8(sign).unwrap();
            conv.write_u8(b',').unwrap();
            conv.f64_pad(mins, 8, 5).unwrap();
            conv.u32(degs as u32).unwrap();
            fmt.write_str(conv.as_str())
        } else {
            fmt.write_str("FormatError")
        }
    }
}
