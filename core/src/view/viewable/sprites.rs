use embedded_graphics::{
    geometry::Angle,
    prelude::*,
    primitives::{Arc, Line, Polyline, PrimitiveStyle, Triangle},
};
#[allow(unused_imports)]
use micromath::F32Ext;

use crate::{utils::Colors, CoreError};

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
#[allow(clippy::too_many_arguments)]
pub fn wind_arrow<D>(
    display: &mut D,
    center: Point,
    angle: Angle,
    av_angle: Angle,
    len: i32,
    fill_color: Colors,
    stroke_color: Colors,
    tail_thick: u32,
    tail_color: Colors,
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

    let style = PrimitiveStyle::with_fill(fill_color);
    let p3 = Point::new(
        (-t2 * cos_a + l2 * sin_a) as i32,
        (-t2 * sin_a - l2 * cos_a) as i32,
    );
    Triangle::new(p_end, center, center + p3)
        .into_styled(style)
        .draw(display)?;
    let p4 = Point::new(
        (t2 * cos_a + l2 * sin_a) as i32,
        (t2 * sin_a - l2 * cos_a) as i32,
    );
    Triangle::new(p_end, center, center + p4)
        .into_styled(style)
        .draw(display)?;

    let style = PrimitiveStyle::with_stroke(stroke_color, 2);
    Line::new(p_end, center + p3)
        .into_styled(style)
        .draw(display)?;
    Line::new(center, center + p3)
        .into_styled(style)
        .draw(display)?;
    Line::new(p_end, center + p4)
        .into_styled(style)
        .draw(display)?;
    Line::new(center, center + p4)
        .into_styled(style)
        .draw(display)?;

    let (w1, w2) = if angle > av_angle {
        (angle, av_angle)
    } else {
        (av_angle, angle)
    };
    let dif = w1 - w2;
    let (w1, w2) = if dif > 180.0.deg() {
        (w1, w2 + 360.0.deg())
    } else {
        (w2, w1)
    };

    // Draw wind tail
    let style = PrimitiveStyle::with_stroke(tail_color, tail_thick);
    Arc::with_center(center, (2.0 * l1) as u32, 90.0.deg() + w1, w2 - w1)
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

const ARROW_VALS: [[f32; 2]; 7] = [
    [0.5, -0.0],
    [0.37267799624996495, 0.4636476090008061],
    [0.33706247360261143, 0.14888994760949725],
    [0.5024937810560445, 3.0419240010986313],
    [0.5024937810560445, -3.0419240010986313],
    [0.33706247360261143, -0.14888994760949725],
    [0.37267799624996495, -0.4636476090008061],
];

use embedded_graphics::Drawable;

/// Draw an Arror arround center with len
pub fn arrow<D>(
    display: &mut D,
    center: Point,
    angle: Angle,
    len: i32,
    fill_color: Colors,
    stroke_color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    fn to_x_y(l_a: [f32; 2], angle: Angle, len: f32) -> Point {
        let (l, a) = (l_a[0] * len, angle.to_radians() + l_a[1]);
        let x = -a.sin() * l;
        let y = a.cos() * l;
        Point::new(x as i32, y as i32)
    }

    let len = len as f32;
    let p1 = to_x_y(ARROW_VALS[0], angle, len) + center;
    let p2 = to_x_y(ARROW_VALS[1], angle, len) + center;
    let p3 = to_x_y(ARROW_VALS[2], angle, len) + center;
    let p4 = to_x_y(ARROW_VALS[3], angle, len) + center;
    let p5 = to_x_y(ARROW_VALS[4], angle, len) + center;
    let p6 = to_x_y(ARROW_VALS[5], angle, len) + center;
    let p7 = to_x_y(ARROW_VALS[6], angle, len) + center;

    let style = PrimitiveStyle::with_fill(fill_color);
    Triangle::new(p1, p2, p7).into_styled(style).draw(display)?;
    Triangle::new(p3, p4, p6).into_styled(style).draw(display)?;
    Triangle::new(p4, p5, p6).into_styled(style).draw(display)?;

    let points = [p1, p2, p3, p4, p5, p6, p7, p1];
    let style = PrimitiveStyle::with_stroke(stroke_color, 1);
    Polyline::new(&points).into_styled(style).draw(display)?;

    Ok(())
}
