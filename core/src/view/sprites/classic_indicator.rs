use super::{DrawColored, Rotate, CLASSIC_INDICATOR_PCOORDS};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
};
#[allow(unused_imports)]
use micromath::F32Ext;

use crate::utils::{Colors, CoreError};

pub struct ClassicIndicator {
    zero_pos: f32,
    rotation: f32,
    len: i32,
    center: Point,
}

impl ClassicIndicator {
    pub const fn new(radius: i32, center: Point) -> Self {
        Self {
            zero_pos: 0.0,
            rotation: 0.0,
            len: radius,
            center,
        }
    }
}

impl DrawColored for ClassicIndicator {
    fn draw_colored<D>(&self, color: Colors, display: &mut D) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError>,
    {
        let scale = self.len as f32;
        let rotation = self.zero_pos + self.rotation;
        let center = self.center;

        // scale and rotate arrow points
        let px = [
            CLASSIC_INDICATOR_PCOORDS[0].to_xy(scale, rotation) + center,
            CLASSIC_INDICATOR_PCOORDS[1].to_xy(scale, rotation) + center,
            CLASSIC_INDICATOR_PCOORDS[2].to_xy(scale, rotation) + center,
            CLASSIC_INDICATOR_PCOORDS[3].to_xy(scale, rotation) + center,
            CLASSIC_INDICATOR_PCOORDS[4].to_xy(scale, rotation) + center,
        ];

        let style = PrimitiveStyle::with_fill(color);
        Triangle::new(px[0], px[1], px[4])
            .into_styled(style)
            .draw(display)?;
        Triangle::new(px[1], px[2], px[3])
            .into_styled(style)
            .draw(display)?;
        Triangle::new(px[1], px[3], px[4])
            .into_styled(style)
            .draw(display)?;

        Ok(())
    }
}

impl Rotate for ClassicIndicator {
    fn rotate(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }
    fn zero_pos(&mut self, zero_pos: f32) -> &mut Self {
        self.zero_pos = zero_pos;
        self
    }
}
