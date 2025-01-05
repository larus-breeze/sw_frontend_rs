use embedded_graphics::{
    geometry::Angle,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Triangle},
};
#[allow(unused_imports)]
use micromath::F32Ext;

use crate::utils::{Colors, CoreError};

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

/// Draw a scale marker
///
#[allow(clippy::too_many_arguments)]
pub fn scale_marker<D>(
    display: &mut D,
    center: Point,
    value: f32,
    radius: i32,
    len: i32,
    width: f32,
    angle_m_s: f32,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    fn scale_coord(center: Point, value: f32, radius: i32, angle_m_s: f32) -> Point {
        let angle = (angle_m_s * value).deg();
        center
            + Point::new(
                -(angle.to_radians().cos() * (radius + 3) as f32) as i32,
                -(angle.to_radians().sin() * (radius + 3) as f32) as i32,
            )
    }

    let p1 = scale_coord(center, value + width, radius, angle_m_s);
    let p2 = scale_coord(center, value - width, radius, angle_m_s);
    let p3 = scale_coord(center, value, radius - len, angle_m_s);
    Triangle::new(p1, p2, p3)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(display)?;
    Ok(())
}

/// Draw a inverted scale marker
///
///
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn inverted_scale_marker<D>(
    display: &mut D,
    center: Point,
    value: f32,
    radius: i32,
    len: i32,
    width: f32,
    angle_m_s: f32,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    fn scale_coord(center: Point, value: f32, radius: i32, angle_m_s: f32) -> Point {
        let angle = (angle_m_s * value).deg();
        center
            + Point::new(
                -(angle.to_radians().cos() * (radius + 3) as f32) as i32,
                -(angle.to_radians().sin() * (radius + 3) as f32) as i32,
            )
    }

    let p1 = scale_coord(center, value + width, radius - 2, angle_m_s);
    let p2 = scale_coord(center, value - width, radius - 2, angle_m_s);
    let p3 = scale_coord(center, value, radius + len, angle_m_s);
    Triangle::new(p1, p2, p3)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(display)?;
    Ok(())
}
