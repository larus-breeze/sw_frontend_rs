use super::{DrawColored, Rotate, SIMPLE_INDICATOR_PCOORDS};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
};
#[allow(unused_imports)]
use micromath::F32Ext;

use crate::utils::{Colors, CoreError};

pub struct SimpleIndicator {
    zero_pos: f32,
    rotation: f32,
    radius: i32,
    center: Point,
}

impl SimpleIndicator {
    pub const fn at_base(radius: i32, center: Point) -> Self {
        Self {
            zero_pos: 0.0,
            rotation: 0.0,
            radius,
            center,
        }
    }

    #[allow(unused)]
    pub const fn at_tip(radius: i32, center: Point) -> Self {
        Self {
            zero_pos: 0.0,
            rotation: 0.0,
            radius: radius * 823 / 1000,
            center,
        }
    }
}

impl DrawColored for SimpleIndicator {
    fn draw_colored<D>(&self, color: Colors, display: &mut D) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError>,
    {
        let scale = self.radius as f32;
        let rotation = self.zero_pos + self.rotation;
        let center = self.center;

        // scale and rotate arrow points
        let px = [
            SIMPLE_INDICATOR_PCOORDS[0].to_xy(scale, rotation) + center,
            SIMPLE_INDICATOR_PCOORDS[1].to_xy(scale, rotation) + center,
            SIMPLE_INDICATOR_PCOORDS[2].to_xy(scale, rotation) + center,
        ];

        let style = PrimitiveStyle::with_fill(color);
        Triangle::new(px[0], px[1], px[2])
            .into_styled(style)
            .draw(display)?;

        Ok(())
    }
}

impl Rotate for SimpleIndicator {
    fn rotate(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }
    fn zero_pos(&mut self, zero_pos: f32) -> &mut Self {
        self.zero_pos = zero_pos;
        self
    }
}
