use crate::{
    model::CoreModel, 
    utils::Colors,
    view::sprites::SimpleIndicator, 
    CoreError, DrawImage, tformat};

#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Line, Rectangle, Triangle},
    geometry::AngleUnit,
};
use num::clamp;
use u8g2_fonts::{
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

use super::sprites::{DrawColored, Rotate};

#[derive(PartialEq)]
pub struct Horizon {}

impl Horizon {
    pub fn new() -> Horizon {
        Horizon {}
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        display.clear(cm.palette().horizon_sky)?;
        let sizes = &cm.device_const.sizes;

        // draw horizon
        //
        let m_roll = -clamp(cm.sensor.euler_roll.to_radians(), -1.55, 1.55).tan();
        let m_pitch = clamp(cm.sensor.euler_pitch.to_radians(), -1.55, 1.55).tan();

        // the y coordinate is not height / 2 because on rectangle display only quadratic content is used
        let ah_center_x = sizes.display.width as i32 /2;
        let ah_center_y = ah_center_x;
        let ah_pitch_center_y = (m_pitch * ah_center_x as f32) as i32 + ah_center_y;

        if m_roll == 0.0 {
            let corner_1 = Point::new(0, ah_pitch_center_y);
            let corner_2 = Point::new(sizes.display.width as i32, sizes.display.height as i32); 
            Rectangle::with_corners(corner_1, corner_2)
                .into_styled(PrimitiveStyle::with_fill(cm.palette().horizon_earth))
                .draw(display)?;
        } else if m_roll > 0.0 {
            let start_y = ah_pitch_center_y - (m_roll * sizes.display.center.x as f32) as i32;
            let mut y = clamp(start_y, 0, sizes.display.height as i32 - 1) as usize;
            let m2_roll = 1.0 / m_roll; 
            while y < sizes.display.height as usize {
                let x = (m2_roll * (y as i32- start_y) as f32) as i32;
                let len = clamp(x, 0, sizes.display.width as i32) as usize;
                // We know, that we are within the display limits, so unsafe is ok
                unsafe {
                    display.draw_line_unchecked(y * sizes.display.width as usize, len, cm.palette().horizon_earth);

                }
                y += 1;
            }
        } else {
            let start_y = ah_pitch_center_y + (m_roll * sizes.display.center.x as f32) as i32;
            let mut y = clamp(start_y, 0, sizes.display.height as i32 - 1) as usize;
            let m2_roll = 1.0 / m_roll;
            while y < sizes.display.height as usize {
                let x = clamp(
                    sizes.display.width as i32 - 1 + (m2_roll * (y as i32 - start_y) as f32) as i32, 
                    0, 
                    (sizes.display.width) as i32);
                let len = clamp(
                    sizes.display.width as i32 - x, 
                    0, 
                    sizes.display.width as i32) as usize;
                let p_idx = y * sizes.display.width as usize + x as usize;
                // We know, that we are within the display limits, so unsafe is ok
                unsafe {
                    display.draw_line_unchecked(p_idx, len, cm.palette().horizon_earth);
                }
                y += 1;
            }
        }

        // draw background image / scale
        //
        display.draw_img(
            &cm.device_const.images.wp_horizon,
            Point::new(0, 0),
            Some(cm.palette().scale),
        )?;

        let roll_angle = -cm.sensor.euler_roll.to_radians();

        // draw roll marker
        //
        SimpleIndicator::at_tip(ah_center_y - 3, Point::new(ah_center_x, ah_center_y))
            .rotate(roll_angle)
            .draw_colored(cm.palette().needle2, display)?;

        // draw pitch scale
        //
        let sin_alpha = (roll_angle + 90.0.deg().to_radians()).sin();
        let cos_alpha = (roll_angle + 90.0.deg().to_radians()).cos();
        let dx = (sin_alpha * sizes.horizon.pitch_scale_len as f32) as i32;
        let dy = (cos_alpha * sizes.horizon.pitch_scale_len as f32) as i32;
        let dcx = cos_alpha * (sizes.display.width / 9) as f32;
        let dcy = sin_alpha * (sizes.display.width / 9) as f32;

        let style = PrimitiveStyle::with_stroke(cm.palette().scale, 2);
        for mul in 1_i32..4 {
            let mul_dcx = (mul as f32 * dcx) as i32;
            let mul_dcy = (mul as f32 * dcy) as i32;
            let p1 = Point::new(ah_center_x - dx - mul_dcx, ah_center_y - mul_dcy + dy);
            let p2 = Point::new(ah_center_x + dx - mul_dcx, ah_center_y - mul_dcy - dy);
            Line::new(p1, p2).into_styled(style).draw(display)?;
            let p1 = Point::new(ah_center_x - dx + mul_dcx, ah_center_y + mul_dcy + dy);
            let p2 = Point::new(ah_center_x + dx + mul_dcx, ah_center_y + mul_dcy - dy);
            Line::new(p1, p2).into_styled(style).draw(display)?;
        }

        // draw true heading
        //
        let th = cm.sensor.euler_yaw.to_degrees();
        let th_txt = tformat!(10, "TH {:.0}°", th).unwrap();
        boxed_text(
            display,
            th_txt.as_str(),
            Point::new(ah_center_x, sizes.display.width as i32 - sizes.horizon.box_height / 2),
            &cm.device_const.big_font,
            sizes.horizon.t_width,
            sizes.horizon.box_height,
            sizes.horizon.stroke_width,
            cm.palette().scale,
            cm.palette().needle4,
            cm.palette().background,
        )?;

        // draw true course
        //
        let tc = cm.sensor.gps_track.to_degrees();
        let tc_txt = tformat!(10, "TC {:.0}°", tc).unwrap();
        let mut diff = tc - th;
        if diff > 180.0 {
            diff -= 360.0;
        }
        if diff < -180.0 {
            diff += 360.0;
        }
        let t_col = cm.palette().needle5;
        let scale_inc = sizes.display.width as i32 / 10;
        let x = ((diff * scale_inc as f32) / 10.0) as i32 + ah_center_x;
        let p1 = Point::new(x, sizes.horizon.tc_needle_y);
        let p2 = Point::new(x + 10, sizes.horizon.tc_needle_y + sizes.horizon.tc_needle_delta);
        let p3 = Point::new(x - 10, sizes.horizon.tc_needle_y + sizes.horizon.tc_needle_delta);
        Triangle::new(p1, p2, p3)
            .into_styled(PrimitiveStyle::with_fill(t_col))
            .draw(display)?;

        let x = clamp(
            x,
            sizes.horizon.t_width / 2,
            sizes.display.width as i32 - sizes.horizon.t_width / 2,
        );
        boxed_text(
            display,
            tc_txt.as_str(),
            Point::new(x, sizes.horizon.tc_pos_y),
            &cm.device_const.big_font,
            sizes.horizon.t_width,
            sizes.horizon.box_height,
            sizes.horizon.stroke_width,
            cm.palette().scale,
            cm.palette().scale,
            cm.palette().background,
        )?;

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn boxed_text<D>(
    display: &mut D,
    content: &str,
    position: Point,
    font: &FontRenderer,
    width: i32,
    height: i32,
    stroke_width: i32,
    frame_color: Colors,
    text_color: Colors,
    background_color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    let top_left = Point::new(position.x - width / 2, position.y - height / 2);
    let size = Size::new(width as u32, height as u32);
    let mut style = PrimitiveStyle::with_stroke(frame_color, stroke_width as u32);
    style.fill_color = Some(background_color);
    Rectangle::new(top_left, size)
        .into_styled(style)
        .draw(display)?;

    font.render_aligned(
        content,
        position,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(text_color),
        display,
    )?;
    Ok(())
}
