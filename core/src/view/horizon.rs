use crate::{model::CoreModel, tformat, utils::Colors, CoreError, DrawImage};

#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
};
use num::clamp;
use u8g2_fonts::{
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

pub struct Horizon {}

impl Horizon {
    pub fn new() -> Horizon {
        Horizon {}
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        display.clear(cm.palette().background)?;
        let sizes = &cm.device_const.sizes.horizon;

        let ah_top = 0;
        let display_width = cm.device_const.sizes.display.width;
        let ah_bottom = display_width as i32;
        let ah_center_y = ah_top + display_width as i32 / 2;
        let ah_center_x = display_width as i32 / 2;
        let ah_center = Point::new(ah_center_x, ah_center_y);
        let scale_inc = display_width as i32 / 10;

        // draw horizon
        //
        let m = clamp(cm.sensor.euler_roll.to_radians(), -1.55, 1.55).tan();
        let m2 = clamp(cm.sensor.euler_pitch.to_radians(), -1.55, 1.55).tan();
        let dy2 = (m2 * ah_center_x as f32) as i32;

        let heaven = PrimitiveStyle::with_fill(cm.palette().horizon_sky);
        Rectangle::new(
            Point::new(0, ah_top),
            Size::new(display_width, display_width),
        )
        .into_styled(heaven)
        .draw(display)?;

        let earth = PrimitiveStyle::with_stroke(cm.palette().horizon_earth, 1);
        for x in 0..display_width as i32 {
            let dy1 = (m * ((display_width / 2) as i32 - x) as f32) as i32;
            let y2 = ah_center_y + clamp(dy1 + dy2, -ah_center_x, ah_center_x);

            let p2 = Point::new(x, y2);
            let p3 = Point::new(x, ah_bottom);
            Line::new(p2, p3).into_styled(earth).draw(display)?;
        }

        // draw background image / scale
        //
        display.draw_img(
            &cm.device_const.images.wp_horizon,
            Point::new(0, 0),
            Some(cm.palette().scale),
        )?;

        // draw roll marker
        //
        let angle = 90.0_f32.deg() - cm.sensor.euler_roll;
        let radius = ah_center_x - 4;

        fn scale_coord(center: Point, angle: Angle, radius: i32) -> Point {
            center
                + Point::new(
                    -(angle.to_radians().cos() * (radius + 3) as f32) as i32,
                    -(angle.to_radians().sin() * (radius + 3) as f32) as i32,
                )
        }

        let p1 = scale_coord(
            ah_center,
            angle + sizes.rm_width.deg(),
            radius - sizes.rm_len,
        );
        let p2 = scale_coord(
            ah_center,
            angle - sizes.rm_width.deg(),
            radius - sizes.rm_len,
        );
        let p3 = scale_coord(ah_center, angle, radius);
        Triangle::new(p1, p2, p3)
            .into_styled(PrimitiveStyle::with_fill(Colors::Red))
            .draw(display)?;

        // draw pitch scale
        //
        let sin_alpha = angle.to_radians().sin();
        let cos_alpha = angle.to_radians().cos();
        let dx = (sin_alpha * sizes.pitch_scale_len as f32) as i32;
        let dy = (cos_alpha * sizes.pitch_scale_len as f32) as i32;
        let dcx = cos_alpha * (display_width / 9) as f32;
        let dcy = sin_alpha * (display_width / 9) as f32;

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
            Point::new(ah_center_x, ah_bottom - sizes.box_height / 2),
            &cm.device_const.big_font,
            sizes.t_width,
            sizes.box_height,
            sizes.stroke_width,
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
        let x = ((diff * scale_inc as f32) / 10.0) as i32 + ah_center_x;
        let p1 = Point::new(x, sizes.tc_needle_y);
        let p2 = Point::new(x + 10, sizes.tc_needle_y + sizes.tc_needle_delta);
        let p3 = Point::new(x - 10, sizes.tc_needle_y + sizes.tc_needle_delta);
        Triangle::new(p1, p2, p3)
            .into_styled(PrimitiveStyle::with_fill(t_col))
            .draw(display)?;

        let x = clamp(
            x,
            sizes.t_width / 2,
            display_width as i32 - sizes.t_width / 2,
        );
        boxed_text(
            display,
            tc_txt.as_str(),
            Point::new(x, sizes.tc_pos_y),
            &cm.device_const.big_font,
            sizes.t_width,
            sizes.box_height,
            sizes.stroke_width,
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
