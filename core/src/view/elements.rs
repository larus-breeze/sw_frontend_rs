use crate::CoreError;
use embedded_graphics::{
    geometry::Angle,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Triangle},
};
#[allow(unused_imports)]
use micromath::F32Ext;

use super::colors::Colors;

/// Draw an indicator
///
/// Draws a classic indicator with given thickness and a tip on the display. Angle 0 is aligned horizontally to the left.
/// Increasing angles lead to a rotation of the pointer in clockwise direction
pub fn classic_indicator<D>(
    display: &mut D,
    center: Point,
    angle: Angle,
    thickness: i32,
    start: i32,
    end: i32,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    let r3 = end - thickness;
    let sin_a = angle.to_radians().sin();
    let cos_a = angle.to_radians().cos();
    let p_start = center
        + Point::new(
            -(cos_a * start as f32) as i32,
            -(sin_a * start as f32) as i32,
        );
    let p_r3 = center + Point::new(-(cos_a * r3 as f32) as i32, -(sin_a * r3 as f32) as i32);
    let p_end = center + Point::new(-(cos_a * end as f32) as i32, -(sin_a * end as f32) as i32);
    let p_thick = Point::new(
        (sin_a * (thickness / 2) as f32) as i32,
        -(cos_a * (thickness / 2) as f32) as i32,
    );

    let style = PrimitiveStyle::with_stroke(color, thickness as u32);
    Line::new(p_start, p_r3).into_styled(style).draw(display)?;
    let style = PrimitiveStyle::with_fill(color);
    Triangle::new(p_end, p_r3 + p_thick, p_r3 - p_thick)
        .into_styled(style)
        .draw(display)
}

/// Draw a wind arrow
///
/// The wind arrow is pointed at the front and open at the back. It has a length and a width. An angle of 0 means that the
/// arrow points upwards.
pub fn wind_arrow<D>(
    display: &mut D,
    center: Point,
    angle: Angle,
    len: i32,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    let l1 = len as f32 * 0.666;
    let l2 = l1 / 2.0;
    let t2 = len as f32 * 0.25;
    let sin_a = angle.to_radians().sin();
    let cos_a = angle.to_radians().cos();
    let p_end = center + Point::new((-sin_a * l1) as i32, (cos_a * l1) as i32);

    let style = PrimitiveStyle::with_fill(color);
    let p3 = Point::new(
        (-t2 * cos_a + l2 * sin_a) as i32,
        (-t2 * sin_a - l2 * cos_a) as i32,
    );
    Triangle::new(p_end, center, center + p3)
        .into_styled(style)
        .draw(display)?;
    let p3 = Point::new(
        (t2 * cos_a + l2 * sin_a) as i32,
        (t2 * sin_a - l2 * cos_a) as i32,
    );
    Triangle::new(p_end, center, center + p3)
        .into_styled(style)
        .draw(display)
}

/// Draw a scale marker
///
///
pub fn scale_marker<D>(
    display: &mut D,
    center: Point,
    value: f32,
    radius: i32,
    len: i32,
    width: f32,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    fn scale_coord(center: Point, value: f32, radius: i32) -> Point {
        let angle = (25.0 * value).deg();
        center
            + Point::new(
                -(angle.to_radians().cos() * (radius + 3) as f32) as i32,
                -(angle.to_radians().sin() * (radius + 3) as f32) as i32,
            )
    }

    let p1 = scale_coord(center, value + width, radius);
    let p2 = scale_coord(center, value - width, radius);
    let p3 = scale_coord(center, value, radius - len);
    Triangle::new(p1, p2, p3)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(display)?;
    Ok(())
}
