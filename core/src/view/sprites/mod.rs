pub(crate) mod arrow;

pub(crate) use arrow::Arrow;

use embedded_graphics::{
    prelude::{Point, DrawTarget},
    primitives::PrimitiveStyle
};
use crate::{utils::Colors, CoreError};

#[allow(unused_imports)]
use micromath::F32Ext;

pub struct PolarCoordinate {
    pub len: f32,
    pub alpha: f32,
}

impl PolarCoordinate {
    pub fn to_xy(&self, scale: f32, rotation: f32) -> Point {
        let a = self.alpha + rotation;
        let l = self.len * scale;
        let x = a.sin() * l;
        let y = -a.cos() * l;
        Point::new(x as i32, y as i32)
    }
}

pub trait Rotate {
    fn rotate(&mut self, rotation: f32) -> &mut Self;
    #[allow(unused)]
    fn zero_pos(&mut self, zero_pos: f32) -> &mut Self;
}

pub trait DrawStyled {
    fn draw_styled<D>(&self, style: PrimitiveStyle<Colors>, display: &mut D) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError>;
}

pub mod pos {
    use core::f32::consts::PI;

    #[allow(unused)]
    pub const TWELVE_O_CLOCK: f32 = 0.0;
    #[allow(unused)]
    pub const THREE_O_CLOCK: f32 = 0.5*PI;
    #[allow(unused)]
    pub const SIX_O_CLOCK: f32 = PI;
    #[allow(unused)]
    pub const NINE_O_CLOCK: f32 = 1.5*PI;
}