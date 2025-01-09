use super::{DrawStyled, Rotate, ARROW_PCOORDS};
use embedded_graphics::{
    prelude::*,
    primitives::{Polyline, PrimitiveStyle, Triangle},
};
#[allow(unused_imports)]
use micromath::F32Ext;

use crate::utils::{Colors, CoreError};

pub struct Arrow {
    zero_pos: f32,
    rotation: f32,
    len: i32,
    center: Point,
}

impl Arrow{
    pub const fn new(len: i32, center: Point) -> Self {
        Self {
            zero_pos: 0.0,
            rotation: 0.0,
            len,
            center
        }
    }
}

impl DrawStyled for Arrow {
    fn draw_styled<D>(&self, style: PrimitiveStyle<Colors>, display: &mut D) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError>,
    {
        if style.is_transparent() {
            return Ok(());
        }

        let scale = self.len as f32;
        let rotation = self.zero_pos + self.rotation;
        let center = self.center;

        // scale and rotate arrow points
        let p1 = ARROW_PCOORDS[0].to_xy(scale, rotation) + center;
        let px = [
            p1,
            ARROW_PCOORDS[1].to_xy(scale, rotation) + center,
            ARROW_PCOORDS[2].to_xy(scale, rotation) + center,
            ARROW_PCOORDS[3].to_xy(scale, rotation) + center,
            ARROW_PCOORDS[4].to_xy(scale, rotation) + center,
            ARROW_PCOORDS[5].to_xy(scale, rotation) + center,
            ARROW_PCOORDS[6].to_xy(scale, rotation) + center,
            p1,
        ];

        if let Some(fill_color) = style.fill_color {
            let style = PrimitiveStyle::with_fill(fill_color);
            Triangle::new(px[0], px[1], px[6]).into_styled(style).draw(display)?;
            Triangle::new(px[2], px[3], px[5]).into_styled(style).draw(display)?;
            Triangle::new(px[3], px[4], px[5]).into_styled(style).draw(display)?;
        }

        if let Some(stroke_color) = style.stroke_color {
            let style = PrimitiveStyle::with_stroke(stroke_color, style.stroke_width);
            Polyline::new(&px).into_styled(style).draw(display)?;
        }

        Ok(())
    }
}



impl Rotate for Arrow {
    fn rotate(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }
    fn zero_pos(&mut self, zero_pos: f32) -> &mut Self{
        self.zero_pos = zero_pos;
        self
    }
}
