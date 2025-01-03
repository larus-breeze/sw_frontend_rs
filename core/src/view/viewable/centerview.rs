use super::sprites::*;
use crate::{Colors, CoreError, CoreModel, DrawImage, FloatToSpeed, FlyMode, VarioSizes};
use embedded_graphics::{draw_target::DrawTarget, prelude::Angle};

use embedded_graphics::geometry::AngleUnit;

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
    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
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
    cm: &CoreModel,
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

fn calc_len(wind_speed: f32, sizes: &VarioSizes) -> i32 {
    match wind_speed {
        x if x < WIND_MIN => sizes.wind_len_min, // Light wind is set to a minimum size
        x if x > WIND_MAX => sizes.wind_len,     // Strong wind is set to a maximum size
        _ => {
            sizes.wind_len_min
                + ((sizes.wind_len - sizes.wind_len_min) as f32 * (wind_speed - WIND_MIN)
                    / (WIND_MAX - WIND_MIN)) as i32
        }
    }
}

fn draw_single_arrow<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;
    let d_sizes = &cm.device_const.sizes.display;

    // draw wind arrow
    let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
    let (angle, av_angle) = draw_and_calc_wind_basics(display, cm)?;

    let len = calc_len(wind_speed, sizes);
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

fn draw_double_arrow<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;
    let d_sizes = &cm.device_const.sizes.display;

    // draw wind arrow
    let (angle, av_angle) = draw_and_calc_wind_basics(display, cm)?;

    let len = calc_len(cm.sensor.average_wind.speed().to_km_h(), sizes);
    arrow(
        display,
        d_sizes.center,
        av_angle,
        len,
        cm.palette().sprite2_fill,
        cm.palette().sprite2_fill,
    )?;

    let len = calc_len(cm.sensor.wind_vector.speed().to_km_h(), sizes);
    arrow(
        display,
        d_sizes.center,
        angle,
        len,
        cm.palette().sprite1_fill,
        cm.palette().sprite2_stroke,
    )?;

    Ok(())
}
