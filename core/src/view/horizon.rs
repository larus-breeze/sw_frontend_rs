use crate::{
    model::CoreModel, utils::Colors, view::sprites::SimpleIndicator, CoreError, DrawImage,
    view::viewable::circle_area::draw_info,
};

#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Circle},
};
use num::clamp;

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
        if cm.sensor.horizon_available == false {
            display.clear(cm.palette().horizon_sky)?;
            return draw_info(display, cm, "Horizon", "not available")
        }

        display.clear(cm.palette().horizon_sky)?;
        let sizes = &cm.device_const.sizes;

        // draw horizon
        //
        let m_roll = -clamp(cm.sensor.euler_roll.to_radians(), -1.55, 1.55).tan();
        let m_pitch = clamp(cm.sensor.euler_pitch.to_radians(), -1.55, 1.55).tan();

        // the y coordinate is not height / 2 because on rectangle display only quadratic content is used
        let ah_center_x = sizes.display.width as i32 / 2;
        let ah_center_y = ah_center_x;
        let ah_pitch_center_y = (m_pitch * ah_center_x as f32) as i32 + ah_center_y;

        // to achieve a good performance, we have to use draw_line_unchecked()
        if m_roll == 0.0 {
            let corner_1 = Point::new(0, ah_pitch_center_y);
            let corner_2 = Point::new(sizes.display.width as i32, sizes.display.height as i32);
            Rectangle::with_corners(corner_1, corner_2)
                .into_styled(PrimitiveStyle::with_fill(cm.palette().horizon_earth))
                .draw(display)?;
        } else if m_roll > 0.0 {
            let start_y = ah_pitch_center_y - (m_roll * ah_center_x as f32) as i32;
            let mut y = clamp(start_y, 0, sizes.display.height as i32 - 1) as usize;
            let m2_roll = 1.0 / m_roll;
            while y < sizes.display.height as usize {
                let x = (m2_roll * (y as i32 - start_y) as f32) as i32;
                let len = clamp(x, 0, sizes.display.width as i32) as usize;
                // We know, that we are within the display limits, so unsafe is ok
                unsafe {
                    display.draw_line_unchecked(
                        y * sizes.display.width as usize,
                        len,
                        cm.palette().horizon_earth,
                    );
                }
                y += 1;
            }
        } else {
            let start_y = ah_pitch_center_y + (m_roll * ah_center_x as f32) as i32;
            let mut y = clamp(start_y, 0, sizes.display.height as i32 - 1) as usize;
            let m2_roll = 1.0 / m_roll;
            while y < sizes.display.height as usize {
                let x = clamp(
                    sizes.display.width as i32 - 1 + (m2_roll * (y as i32 - start_y) as f32) as i32,
                    0,
                    (sizes.display.width) as i32,
                );
                let len = clamp(
                    sizes.display.width as i32 - x,
                    0,
                    sizes.display.width as i32,
                ) as usize;
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
            cm.device_const.images.wp_horizon,
            Point::new(0, 0),
            None,
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

        // draw level tube
        //
        let cmh = &cm.device_const.sizes.horizon;
        let sin_slip = cm.sensor.slip_angle.to_radians().sin();
        let cos_slip = cm.sensor.slip_angle.to_radians().cos();
        let d_x = (cmh.lt_swing_len as f32 * sin_slip) as i32;
        let x = (sizes.display.width / 2) as i32 + d_x; 
        let d_y = (cmh.lt_swing_len as f32 * cos_slip) as i32;
        let y = cmh.lt_mid + d_y;
        let center = Point::new(x, y);

        let style = PrimitiveStyle::with_fill(Colors::Black);
        Circle::with_center(center, cmh.lt_rad)
            .into_styled(style)
            .draw(display)?;

        Ok(())
    }
}
