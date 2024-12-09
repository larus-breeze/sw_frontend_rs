use super::{
    DISPLAY_HEIGHT, DISPLAY_WIDTH,
    helpers::themes::Palette,
    helpers::images::images::*,
};
use crate::{
    model::CoreModel,
    tformat,
    utils::Colors,
    CoreError, DrawImage,
    view::helpers::themes::FONT_BIG
};

#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
};
use num::clamp;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

const AH_TOP: i32 = 0;
const AH_BOTTOM: i32 = DISPLAY_WIDTH as i32;
const AH_CENTER_Y: i32 = AH_TOP + DISPLAY_WIDTH as i32 / 2;
const AH_CENTER_X: i32 = DISPLAY_WIDTH as i32 / 2;
const AH_CENTER: Point = Point::new(AH_CENTER_X, AH_CENTER_Y);
const SCALE_INC: i32 = DISPLAY_WIDTH as i32 / 10;

#[cfg(feature = "air_avionics_ad57")]
mod c {
    pub const T_WIDTH: i32 = 88;
    pub const RM_LEN: i32 = 25;
    pub const RM_WIDTH: f32 = 7.0;
    pub const STROKE_WIDTH: i32 = 2;
    pub const BOX_HEIGHT: i32 = 30;
    pub const TC_POS_Y: i32 = super::DISPLAY_HEIGHT as i32 - 16;
    pub const TC_NEEDLE_Y: i32 = super::DISPLAY_WIDTH as i32;
    pub const TC_NEEDLE_DELTA: i32 = 18;
    pub const PITCH_SCALE_LEN: i32 = 18;
}

#[cfg(feature = "larus_frontend_v1")]
mod c {
    pub const T_WIDTH: i32 = 100;
    pub const RM_LEN: i32 = 30;
    pub const RM_WIDTH: f32 = 8.0;
    pub const STROKE_WIDTH: i32 = 2;
    pub const BOX_HEIGHT: i32 = 32;
    pub const TC_POS_Y: i32 = super::DISPLAY_HEIGHT as i32 - 30;
    pub const TC_NEEDLE_Y: i32 = super::DISPLAY_WIDTH as i32;
    pub const TC_NEEDLE_DELTA: i32 = 18;
    pub const PITCH_SCALE_LEN: i32 = 20;
}

#[cfg(feature = "larus_frontend_v2")]
mod c {
    pub const T_WIDTH: i32 = 160;
    pub const RM_LEN: i32 = 45;
    pub const RM_WIDTH: f32 = 6.0;
    pub const STROKE_WIDTH: i32 = 3;
    pub const BOX_HEIGHT: i32 = 50;
    pub const TC_POS_Y: i32 = super::DISPLAY_HEIGHT as i32 - 105;
    pub const TC_NEEDLE_Y: i32 = super::DISPLAY_WIDTH as i32 - 50;
    pub const TC_NEEDLE_DELTA: i32 = -30;
    pub const PITCH_SCALE_LEN: i32 = 20;
}

pub struct Horizon {}

impl Horizon {
    pub fn new() -> Horizon {
        Horizon {}
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        display.clear(cm.color(Palette::Background))?;

        // draw horizon
        //
        let m = clamp(cm.sensor.euler_roll.to_radians(), -1.55, 1.55).tan();
        let m2 = clamp(cm.sensor.euler_pitch.to_radians(), -1.55, 1.55).tan();
        let dy2 = (m2 * AH_CENTER_X as f32) as i32;

        let heaven = PrimitiveStyle::with_fill(cm.color(Palette::HorizonSky));
        Rectangle::new(
            Point::new(0, AH_TOP),
            Size::new(DISPLAY_WIDTH, DISPLAY_WIDTH),
        )
        .into_styled(heaven)
        .draw(display)?;

        let earth = PrimitiveStyle::with_stroke(cm.color(Palette::HorizonEarth), 1);
        for x in 0..DISPLAY_WIDTH as i32 {
            let dy1 = (m * ((DISPLAY_WIDTH / 2) as i32 - x) as f32) as i32;
            let y2 = AH_CENTER_Y + clamp(dy1 + dy2, -AH_CENTER_X, AH_CENTER_X);

            let p2 = Point::new(x, y2);
            let p3 = Point::new(x, AH_BOTTOM);
            Line::new(p2, p3).into_styled(earth).draw(display)?;
        }

        // draw background image / scale
        //
        display.draw_img(
            WP_HORIZON_IMG,
            Point::new(0, 0),
            Some(cm.color(Palette::Scale)),
        )?;

        // draw roll marker
        //
        let angle = 90.0_f32.deg() - cm.sensor.euler_roll;
        let radius = AH_CENTER_X - 4;

        fn scale_coord(center: Point, angle: Angle, radius: i32) -> Point {
            center
                + Point::new(
                    -(angle.to_radians().cos() * (radius + 3) as f32) as i32,
                    -(angle.to_radians().sin() * (radius + 3) as f32) as i32,
                )
        }

        let p1 = scale_coord(AH_CENTER, angle + c::RM_WIDTH.deg(), radius - c::RM_LEN);
        let p2 = scale_coord(AH_CENTER, angle - c::RM_WIDTH.deg(), radius - c::RM_LEN);
        let p3 = scale_coord(AH_CENTER, angle, radius);
        Triangle::new(p1, p2, p3)
            .into_styled(PrimitiveStyle::with_fill(Colors::Red))
            .draw(display)?;

        // draw pitch scale
        //
        let sin_alpha = angle.to_radians().sin();
        let cos_alpha = angle.to_radians().cos();
        let dx = (sin_alpha * c::PITCH_SCALE_LEN as f32) as i32;
        let dy = (cos_alpha * c::PITCH_SCALE_LEN as f32) as i32;
        let dcx = cos_alpha * (DISPLAY_WIDTH / 9) as f32;
        let dcy = sin_alpha * (DISPLAY_WIDTH / 9) as f32;

        let style = PrimitiveStyle::with_stroke(cm.color(Palette::Scale), 2);
        for mul in 1_i32..4 {
            let mul_dcx = (mul as f32 * dcx) as i32;
            let mul_dcy = (mul as f32 * dcy) as i32;
            let p1 = Point::new(AH_CENTER_X - dx - mul_dcx, AH_CENTER_Y - mul_dcy + dy);
            let p2 = Point::new(AH_CENTER_X + dx - mul_dcx, AH_CENTER_Y - mul_dcy - dy);
            Line::new(p1, p2).into_styled(style).draw(display)?;
            let p1 = Point::new(AH_CENTER_X - dx + mul_dcx, AH_CENTER_Y + mul_dcy + dy);
            let p2 = Point::new(AH_CENTER_X + dx + mul_dcx, AH_CENTER_Y + mul_dcy - dy);
            Line::new(p1, p2).into_styled(style).draw(display)?;
        }

        // draw true heading
        //
        let th = cm.sensor.euler_yaw.to_degrees();
        let th_txt = tformat!(10, "TH {:.0}°", th).unwrap();
        boxed_text(
            display,
            th_txt.as_str(),
            Point::new(AH_CENTER_X, AH_BOTTOM - c::BOX_HEIGHT / 2),
            c::T_WIDTH,
            c::STROKE_WIDTH,
            cm.color(Palette::Scale),
            cm.color(Palette::Needle4),
            cm.color(Palette::Background),
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
        let t_col = cm.color(Palette::Needle5);
        let x = ((diff * SCALE_INC as f32) / 10.0) as i32 + AH_CENTER_X;
        let p1 = Point::new(x, c::TC_NEEDLE_Y);
        let p2 = Point::new(x + 10, c::TC_NEEDLE_Y + c::TC_NEEDLE_DELTA);
        let p3 = Point::new(x - 10, c::TC_NEEDLE_Y + c::TC_NEEDLE_DELTA);
        Triangle::new(p1, p2, p3)
            .into_styled(PrimitiveStyle::with_fill(t_col))
            .draw(display)?;

        let x = clamp(x, c::T_WIDTH / 2, DISPLAY_WIDTH as i32 - c::T_WIDTH / 2);
        boxed_text(
            display,
            tc_txt.as_str(),
            Point::new(x, c::TC_POS_Y),
            c::T_WIDTH,
            c::STROKE_WIDTH,
            cm.color(Palette::Scale),
            cm.color(Palette::Scale),
            cm.color(Palette::Background),
        )?;

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn boxed_text<D>(
    display: &mut D,
    content: &str,
    position: Point,
    width: i32,
    stroke_width: i32,
    frame_color: Colors,
    text_color: Colors,
    background_color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    let top_left = Point::new(position.x - width / 2, position.y - c::BOX_HEIGHT / 2);
    let size = Size::new(width as u32, c::BOX_HEIGHT as u32);
    let mut style = PrimitiveStyle::with_stroke(frame_color, stroke_width as u32);
    style.fill_color = Some(background_color);
    Rectangle::new(top_left, size)
        .into_styled(style)
        .draw(display)?;

    FONT_BIG.render_aligned(
        content,
        position,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(text_color),
        display,
    )?;
    Ok(())
}
