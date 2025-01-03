use crate::{Colors, CoreError, CoreModel, DrawImage, FloatToSpeed, FlyMode};
use embedded_graphics::{draw_target::DrawTarget, prelude::{Angle, Primitive}, primitives::{Polyline, PrimitiveStyle, Triangle}};
use super::sprites::*;

use embedded_graphics::geometry::{AngleUnit, Point};

#[derive(Clone, Copy, PartialEq)]
pub enum CenterView {
    None,
    SingleArrowCircling,
    SingleArrowStraight,
    DoubleArrowCircling,
    DoubleArrowStraight,
    LastElemntNotInUse,
}

impl core::convert::From<u32> for CenterView {
    fn from(value: u32) -> Self {
        let idx = if value >= Self::LastElemntNotInUse as u32 - 1 {
            Self::LastElemntNotInUse as u8 - 1
        } else {
            value as u8
        };
        // Transmute is ok, as idx is guaranteed to be in the valid range
        unsafe { core::mem::transmute::<u8, CenterView>(idx) }
    }
}

const CIRCLING_CENTER_VIEW: [CenterView; 2] = [
    CenterView::SingleArrowCircling,
    CenterView::DoubleArrowCircling,
];

const STRAIGHT_CENTER_VIEW: [CenterView; 2] = [
    CenterView::SingleArrowStraight,
    CenterView::DoubleArrowStraight,
];

// Limits of the wind arrow
const WIND_MIN: f32 = 10.0; // 10 km/h
const WIND_MAX: f32 = 30.0; // 30 km/h

pub enum CenterType {
    Circling,
    Straight,
}

impl CenterView {
    pub const fn max(center_type: CenterType) -> usize {
        match center_type {
            CenterType::Circling => CIRCLING_CENTER_VIEW.len() - 1,
            CenterType::Straight => STRAIGHT_CENTER_VIEW.len() - 1,
        }
    }

    // This method is used by the editor to obtain the correct viewables in the correct order
    pub fn from_sorted(value: usize, center_type: CenterType) -> CenterView {
        match center_type {
            CenterType::Circling => {
                if value < CIRCLING_CENTER_VIEW.len() {
                    return CIRCLING_CENTER_VIEW[value];
                }
            }
            CenterType::Straight => {
                if value < STRAIGHT_CENTER_VIEW.len() {
                    return STRAIGHT_CENTER_VIEW[value];
                }
            }
        }
        CenterView::None // should never happen
    }

    pub fn sorted_as_i32(&self, center_type: CenterType) -> i32 {
        match center_type {
            CenterType::Circling => {
                for idx in 0..CIRCLING_CENTER_VIEW.len() {
                    if *self == CIRCLING_CENTER_VIEW[idx] {
                        return idx as i32;
                    };
                }
            }
            CenterType::Straight => {
                for idx in 0..STRAIGHT_CENTER_VIEW.len() {
                    if *self == STRAIGHT_CENTER_VIEW[idx] {
                        return idx as i32;
                    };
                }
            }
        }
        0 // should never happen
    }

    /// Get the name of a viewable
    pub fn name(&self) -> &'static str {
        match self {
            CenterView::SingleArrowCircling => "Arrow Circling",
            CenterView::SingleArrowStraight => "Arrow Straight",
            CenterView::DoubleArrowCircling => "Double Circling",
            CenterView::DoubleArrowStraight => "Double Straight",
            CenterView::None => "None",
            CenterView::LastElemntNotInUse => "",
        }
    }

    /// Draw viewable
    pub fn draw<D>(
        &self,
        display: &mut D,
        cm: &CoreModel,
    ) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        match self {
            CenterView::None => Ok(()),
            CenterView::SingleArrowCircling => draw_single_arrow(display, cm),
            CenterView::SingleArrowStraight => draw_single_arrow(display, cm),
            CenterView::DoubleArrowCircling => draw_double_arrow(display, cm),
            CenterView::DoubleArrowStraight => draw_double_arrow(display, cm),
            CenterView::LastElemntNotInUse => Ok(()),
        }
    }
}

fn draw_and_calc_wind_basics<D>(
    display: &mut D,
    cm: &CoreModel
) -> Result<(Angle, Angle), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;
    let (mut angle, mut av_angle) = match cm.control.fly_mode {
        FlyMode::Circling => {
            // draw north symbol
            display.draw_img(
                &cm.device_const.images.north,
                sizes.north_pos,
                Some(cm.palette().background),
            )?;
            // return absolut wind vector
            (
                cm.sensor.wind_vector.angle(),
                cm.sensor.average_wind.angle(),
            )
        }
        FlyMode::StraightFlight => {
            // draw glider symbol
            display.draw_img(
                &cm.device_const.images.glider,
                sizes.glider_pos,
                Some(cm.palette().scale),
            )?;
            (
                // return relativ wind vector
                cm.sensor.wind_vector.angle() - cm.sensor.gps_track,
                cm.sensor.average_wind.angle() - cm.sensor.gps_track,
            )
        }
    };

    if cm.sensor.airspeed.ias() < 30.0.km_h() {
        angle = 180.0.deg(); // The sensor box should actually do this
        av_angle = 180.0.deg();
    }
    Ok((angle, av_angle))
}

fn draw_single_arrow<D>(
    display: &mut D,
    cm: &CoreModel,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;
    let d_sizes = &cm.device_const.sizes.display;

    // draw wind arrow
    let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
    let (angle, av_angle) = draw_and_calc_wind_basics(display, cm)?;
    
    let len = match wind_speed {
        x if x < WIND_MIN => sizes.wind_len_min, // Light wind is set to a minimum size
        x if x > WIND_MAX => sizes.wind_len,     // Strong wind is set to a maximum size
        _ => {
            sizes.wind_len_min
                + ((sizes.wind_len - sizes.wind_len_min) as f32 * (wind_speed - WIND_MIN)
                    / (WIND_MAX - WIND_MIN)) as i32
        }
    };
    let avg_wind_spped = cm.sensor.average_wind.speed().to_km_h();
    let delta_speed = wind_speed - avg_wind_spped;
    let delta_color = if delta_speed < 0.0 {
        cm.palette().vario_wind_minus
    } else {
        cm.palette().vario_wind_plus
    };
    let tail_thick = (num::clamp(num::abs(delta_speed), 1.0, 10.0)) as u32;
    wind_arrow(
        display,
        d_sizes.center,
        angle,
        av_angle,
        len,
        cm.palette().sprite1_fill,
        cm.palette().sprite1_stroke,
        tail_thick,
        delta_color,
    )?;
    Ok(())
}

fn draw_double_arrow<D>(
    display: &mut D,
    cm: &CoreModel,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let _sizes = &cm.device_const.sizes.vario;
    let d_sizes = &cm.device_const.sizes.display;

    // draw wind arrow
    let (angle, _av_angle) = draw_and_calc_wind_basics(display, cm)?;
    arrow(display, d_sizes.center, angle, 150, cm.palette().sprite1_fill, cm.palette().sprite1_stroke)?;
//    arrow(display, d_sizes.center, angle, 100, cm.palette().sprite2_fill, cm.palette().sprite2_stroke)?;
    Ok(())
}

#[allow(unused_imports)]
use micromath::F32Ext;
use embedded_graphics::Drawable;
pub fn arrow<D>(
    display: &mut D,
    ctr: Point,
    _angle: Angle,
    len: i32,
    fill_color: Colors,
    stroke_color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    let l_2 = len / 2;
    let ah = len / 5;
    let w = len / 14;
    let p1 = Point::new(0, -l_2);
    let p2 = Point::new(ah, ah - l_2);
    let p3 = Point::new(w, p2.y);
    let p4 = Point::new(p3.x, l_2);
    let p5 = Point::new(-w, p4.y);
    let p6 = Point::new(p5.x, p3.y);
    let p7 = Point::new(-ah, p3.y);

    //let sin_a = angle.to_radians().sin();
    //let cos_a = angle.to_radians().cos();

    let p1 = p1 + ctr;
    let p2 = p2 + ctr;
    let p3 = p3 + ctr;
    let p4 = p4 + ctr;
    let p5 = p5 + ctr;
    let p6 = p6 + ctr;
    let p7 = p7 + ctr;

    let style = PrimitiveStyle::with_fill(fill_color);
    Triangle::new(p1, p2, p7).into_styled(style).draw(display)?;
    Triangle::new(p3, p4, p6).into_styled(style).draw(display)?;
    Triangle::new(p4, p5, p6).into_styled(style).draw(display)?;

    let points = [p1, p2, p3, p4, p5, p6, p7, p1];
    let style = PrimitiveStyle::with_stroke(stroke_color, 1);
    Polyline::new(&points).into_styled(style).draw(display)?;

    Ok(())
}