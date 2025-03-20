use crate::view::{
    sprites::{pos, Arrow, DrawStyled, PolarCoordinate, Rotate, WindArrow},
    thermal_data::{ThermalData, DELTA_ALPHA, THERMAL_DATA_CNT},
};
use crate::{Colors, CoreError, CoreModel, DrawImage, FloatToSpeed, FlyMode, VarioSizes};

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{AngleUnit, Point},
    prelude::{Angle, Primitive},
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Triangle},
    Drawable,
};
use num::clamp;

#[allow(unused_imports)]
use micromath::F32Ext;
use num_enum::FromPrimitive;

#[derive(Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum CenterView {
    #[default]
    None,
    SingleArrowCircling,
    SingleArrowStraight,
    DoubleArrowCircling,
    DoubleArrowStraight,
    ThermalAssistant1,
    ThermalAssistant2,
    LastElemntNotInUse,
}

const CIRCLING_CENTER_VIEW: [CenterView; 4] = [
    CenterView::SingleArrowCircling,
    CenterView::DoubleArrowCircling,
    CenterView::ThermalAssistant1,
    CenterView::ThermalAssistant2,
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
                for (idx, view_item) in CIRCLING_CENTER_VIEW.iter().enumerate() {
                    if *self == *view_item {
                        return idx as i32;
                    };
                }
            }
            CenterType::Straight => {
                for (idx, view_item) in STRAIGHT_CENTER_VIEW.iter().enumerate() {
                    if *self == *view_item {
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
            CenterView::SingleArrowCircling => "Single Arrow",
            CenterView::SingleArrowStraight => "Single Arrow",
            CenterView::DoubleArrowCircling => "Double Arrow",
            CenterView::DoubleArrowStraight => "Double Arrow",
            CenterView::ThermalAssistant1 => "Dotted Assistant",
            CenterView::ThermalAssistant2 => "Spider Assistant",
            CenterView::None => "None",
            CenterView::LastElemntNotInUse => "",
        }
    }

    /// Draw viewable
    pub fn draw<D>(
        &self,
        display: &mut D,
        cm: &CoreModel,
        thermal_data: &mut ThermalData,
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
            CenterView::ThermalAssistant1 => draw_thermal_assitant1(display, cm, thermal_data),
            CenterView::ThermalAssistant2 => draw_thermal_assitant2(display, cm, thermal_data),
            CenterView::LastElemntNotInUse => Ok(()),
        }
    }
}

fn draw_thermal_assitant1<D>(
    display: &mut D,
    cm: &CoreModel,
    thermal_data: &mut ThermalData,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes;
    let mut pcoord = PolarCoordinate {
        alpha: 0.0,
        len: sizes.vario.ta_circle_radius as f32,
    };
    let delta = DELTA_ALPHA;

    let rotation = if cm.sensor.euler_roll.to_radians() > 0.0 {
        -cm.sensor.euler_yaw.to_radians() + pos::NINE_O_CLOCK
    } else {
        -cm.sensor.euler_yaw.to_radians() + pos::THREE_O_CLOCK
    };
    thermal_data.prepare();
    for _cnt in 0..THERMAL_DATA_CNT {
        let (fill_color, delta_climb) = thermal_data.get_dotted_item(pcoord.alpha, cm);
        let center = pcoord.to_xy(1.0, rotation) + sizes.display.center;
        let diameter = clamp(
            (delta_climb.abs() * 10.0) as u32,
            sizes.vario.ta_point_diameter / 3,
            sizes.vario.ta_point_diameter,
        );
        Circle::with_center(center, diameter)
            .into_styled(PrimitiveStyle::with_fill(fill_color))
            .draw(display)?;
        pcoord.rotate(delta);
    }

    let dy = (sizes.vario.small_gld_size.height / 2) as i32;
    let p_gld = if cm.sensor.euler_roll.to_radians() > 0.0 {
        let dx = (sizes.vario.ta_circle_radius + sizes.vario.small_gld_size.width / 2) as i32;
        sizes.display.center + Point::new(-dx, -dy)
    } else {
        let dx = (sizes.vario.ta_circle_radius - sizes.vario.small_gld_size.width / 2) as i32;
        sizes.display.center + Point::new(dx, -dy)
    };
    display.draw_img(
        cm.device_const.images.small_glider,
        p_gld,
        Some(cm.palette().scale),
    )?;
    Ok(())
}

fn draw_thermal_assitant2<D>(
    display: &mut D,
    cm: &CoreModel,
    thermal_data: &mut ThermalData,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes;
    let mut pcoord = PolarCoordinate {
        alpha: 0.0,
        len: sizes.vario.ta_circle_radius as f32,
    };
    let delta = DELTA_ALPHA;
    let center = sizes.display.center;

    let rotation = if cm.sensor.euler_roll.to_radians() > 0.0 {
        -cm.sensor.euler_yaw.to_radians() + pos::NINE_O_CLOCK
    } else {
        -cm.sensor.euler_yaw.to_radians() + pos::THREE_O_CLOCK
    };
    thermal_data.prepare();

    let mut p1: Option<Point> = None;
    let mut p_first: Option<Point> = None;
    let mut fill_color = Colors::Black;
    let mut delta_climb;
    for _cnt in 0..THERMAL_DATA_CNT {
        (fill_color, delta_climb) = thermal_data.get_spider_item(pcoord.alpha, cm);
        let scale = clamp((delta_climb + 3.0) * 0.15 + 0.4, 0.4, 1.3);
        let p2 = pcoord.to_xy(scale, rotation) + center;
        if let Some(p1) = p1 {
            Triangle::new(center, p1, p2)
                .into_styled(PrimitiveStyle::with_fill(fill_color))
                .draw(display)?;
            Line::new(p1, p2)
                .into_styled(PrimitiveStyle::with_stroke(cm.palette().scale, 1))
                .draw(display)?;
        } else {
            p_first = Some(p2);
        }
        p1 = Some(p2);
        pcoord.rotate(delta);
    }

    Triangle::new(center, p1.unwrap(), p_first.unwrap())
        .into_styled(PrimitiveStyle::with_fill(fill_color))
        .draw(display)?;
    Line::new(p1.unwrap(), p_first.unwrap())
        .into_styled(PrimitiveStyle::with_stroke(cm.palette().scale, 1))
        .draw(display)?;

    let dy = (sizes.vario.small_gld_size.height / 2) as i32;
    let p_gld = if cm.sensor.euler_roll.to_radians() > 0.0 {
        let dx = (sizes.vario.ta_circle_radius + sizes.vario.small_gld_size.width / 2) as i32;
        sizes.display.center + Point::new(-dx, -dy)
    } else {
        let dx = (sizes.vario.ta_circle_radius - sizes.vario.small_gld_size.width / 2) as i32;
        sizes.display.center + Point::new(dx, -dy)
    };
    display.draw_img(
        cm.device_const.images.small_glider,
        p_gld,
        Some(cm.palette().scale),
    )?;
    Ok(())
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
                cm.device_const.images.north,
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
            if cm.config.glider_symbol {
                display.draw_img(
                    cm.device_const.images.glider,
                    sizes.glider_pos,
                    Some(cm.palette().scale),
                )?;
            }
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
    let tail_thick = (num::clamp(num::abs(delta_speed), 2.0, 10.0)) as u32;
    let style = PrimitiveStyleBuilder::new()
        .fill_color(cm.palette().sprite1_fill)
        .stroke_color(cm.palette().sprite1_stroke)
        .stroke_width(2)
        .build();

    WindArrow::new(len, d_sizes.center)
        .zero_pos(pos::SIX_O_CLOCK)
        .rotate(angle.to_radians())
        .add_tail(angle - av_angle, tail_thick, delta_color)
        .draw_styled(style, display)?;

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
    let style = PrimitiveStyle::with_fill(cm.palette().sprite2_fill);
    Arrow::new(len, d_sizes.center)
        .zero_pos(pos::SIX_O_CLOCK)
        .rotate(av_angle.to_radians())
        .draw_styled(style, display)?;

    let len = calc_len(cm.sensor.wind_vector.speed().to_km_h(), sizes);
    let style = PrimitiveStyleBuilder::new()
        .fill_color(cm.palette().sprite1_fill)
        .stroke_color(cm.palette().sprite2_stroke)
        .stroke_width(1)
        .build();
    Arrow::new(len, d_sizes.center)
        .zero_pos(pos::SIX_O_CLOCK)
        .rotate(angle.to_radians())
        .draw_styled(style, display)?;

    Ok(())
}
