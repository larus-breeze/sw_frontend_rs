use super::{DrawStyled, Rotate, WIND_ARROW_PCOORDS};
use embedded_graphics::{
    prelude::*,
    primitives::{Polyline, PrimitiveStyle, Triangle, Arc},
};
use core::f32::consts::PI;

#[allow(unused_imports)]
use micromath::F32Ext;

use crate::utils::{Colors, CoreError};

pub struct WindArrow {
    zero_pos: f32,
    rotation: f32,
    len: i32,
    center: Point,
    tail_color: Colors,
    tail_thick: u32,
    tail_angle: f32,
}

impl WindArrow{
    pub const fn new(len: i32, center: Point) -> Self {
        Self {
            zero_pos: 0.0,
            rotation: 0.0,
            len,
            center,
            tail_angle: 0.0,
            tail_color: Colors::Black,
            tail_thick: 0,
        }
    }

    pub fn add_tail(&mut self, angle: Angle, thick: u32, color: Colors) -> &mut Self {
        let mut angle = angle.to_radians();
        while angle > PI {
            angle -= 2.0 * PI
        }
        while angle < -PI {
            angle += 2.0 * PI
        }
        self.tail_angle = angle;
        self.tail_thick = thick;
        self.tail_color = color;
        self
    }
}

impl DrawStyled for WindArrow {
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
        let p1 = WIND_ARROW_PCOORDS[0].to_xy(scale, rotation) + center;
        let px = [
            p1,
            WIND_ARROW_PCOORDS[1].to_xy(scale, rotation) + center,
            WIND_ARROW_PCOORDS[2].to_xy(scale, rotation) + center,
            WIND_ARROW_PCOORDS[3].to_xy(scale, rotation) + center,
            p1,
        ];

        if let Some(fill_color) = style.fill_color {
            let style = PrimitiveStyle::with_fill(fill_color);
            Triangle::new(px[0], px[1], px[2]).into_styled(style).draw(display)?;
            Triangle::new(px[0], px[2], px[3]).into_styled(style).draw(display)?;
        }

        if let Some(stroke_color) = style.stroke_color {
            let style = PrimitiveStyle::with_stroke(stroke_color, style.stroke_width);
            Polyline::new(&px).into_styled(style).draw(display)?;
        }

        if self.tail_angle != 0.0 {
            let tip = WIND_ARROW_PCOORDS[0].get_scaled_rotated(scale, rotation);
            let style = PrimitiveStyle::with_stroke(self.tail_color, self.tail_thick);
            Arc::with_center(
                    center, 
                    (2.0*tip.len) as u32 , 
                    tip.alpha.rad() - 90.0.deg(), 
                    self.tail_angle.rad())
                .into_styled(style)
                .draw(display)?;
        }

        Ok(())
    }
}



impl Rotate for WindArrow {
    fn rotate(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }
    fn zero_pos(&mut self, zero_pos: f32) -> &mut Self{
        self.zero_pos = zero_pos;
        self
    }
}
