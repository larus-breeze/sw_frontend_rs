use super::{
    elements::{classic_indicator, scale_marker, wind_arrow}, // inverted_scale_marker,
    CENTER,
    RADIUS,
    VARIO_SIZES,
};
use crate::{basic_config::*, utils::FONT_HELV_18, Concat, CoreError, DrawImage};
use crate::{
    model::{CoreModel, FlyMode, VarioMode},
    utils::Colors,
};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use VARIO_SIZES as SZS;

#[allow(dead_code)]
struct VarioColors {
    average_climb_rate: Colors,
    background: Colors,
    mc_cready: Colors,
    needle: Colors,
    scale: Colors,
    speed_to_fly: Colors,
    thermal_climb_rate: Colors,
    wind_fill: Colors,
    wind_stroke: Colors,
}

const VARIO_COLORS: VarioColors = VarioColors {
    average_climb_rate: Colors::Yellow,
    background: Colors::Black,
    mc_cready: Colors::Red,
    needle: Colors::White,
    scale: Colors::DarkGray,
    speed_to_fly: Colors::Coral,
    thermal_climb_rate: Colors::LimeGreen,
    wind_fill: Colors::Blue,
    wind_stroke: Colors::LightSkyBlue,
};

use VARIO_COLORS as COLS;

// Limits of the wind arrow
const WIND_MIN: f32 = 10.0; // 10 km/h
const WIND_MAX: f32 = 30.0; // 30 km/h

/// draw the vario display on Screen
pub fn draw<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    // draw wallpaper
    display.clear(COLS.background)?;
    display.draw_img(WALLPAPER_IMG, Point::new(0, 0))?;
    display.draw_img(M_S_IMG, SZS.unit_pos)?;

    for (pos_x, pos_y, txt) in WALLPAPER_SCALE {
        let pos = Point::new(pos_x, pos_y);
        FONT_HELV_18.render(
            txt,
            pos,
            VerticalPosition::Baseline,
            FontColor::Transparent(COLS.scale),
            display,
        )?;
    }

    // dependend on fly_mode draw glider or north symbol
    match cm.control.fly_mode {
        FlyMode::Circling => display.draw_img(NORTH_IMG, Point::new(0, 0))?,
        FlyMode::StraightFlight | FlyMode::Transition => {
            display.draw_img(GLIDER_IMG, Point::new(0, 0))?
        }
    }

    // draw mc_ready marker
    scale_marker(
        display,
        CENTER,
        cm.config.mc_cready.to_m_s(),
        RADIUS as i32,
        SZS.mc_len as i32,
        SZS.mc_width,
        COLS.mc_cready,
    )?;

    // draw thermal climb rate marker
    /*inverted_scale_marker(
        display,
        CENTER,
        cm.calculated.thermal_climb_rate.to_m_s(),
        (RADIUS - SZS.indicator_len) as i32,
        SZS.tcr_len as i32,
        SZS.tcr_width,
        COLS.thermal_climb_rate,
    )?;*/

    // draw climb rate indicator
    let angle = (SZS.angle_m_s * num::clamp(cm.sensor.climb_rate.to_m_s(), -5.1, 5.1)).deg();
    classic_indicator(
        display,
        CENTER,
        angle,
        SZS.indicator_width as i32,
        (RADIUS - SZS.indicator_len) as i32,
        RADIUS as i32,
        COLS.needle,
    )?;

    // draw wind arrow
    let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
    let angle = cm.sensor.wind_vector.angle();
    let av_angle = cm.sensor.average_wind.angle();
    let len = match wind_speed {
        x if x < WIND_MIN => SZS.wind_len_min, // Light wind is set to a minimum size
        x if x > WIND_MAX => SZS.wind_len,     // Strong wind is set to a maximum size
        _ => {
            SZS.wind_len_min
                + ((SZS.wind_len - SZS.wind_len_min) as f32 * (wind_speed - WIND_MIN)
                    / (WIND_MAX - WIND_MIN)) as i32
        }
    };
    wind_arrow(
        display,
        CENTER,
        angle,
        av_angle,
        len,
        COLS.wind_fill,
        COLS.wind_stroke,
    )?;

    // draw wind direction an speed text
    display.draw_img(KM_H_IMG, SZS.wind_pos)?;
    let wind_deg = cm.sensor.wind_vector.angle().to_degrees();
    let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
    let s = Concat::<25>::from_f32(wind_deg, 0).push_str("° ");
    let s = s.push_f32(wind_speed, 0);
    FONT_HELV_18.render_aligned(
        s.as_str(),
        SZS.wind_pos,
        VerticalPosition::Top,
        HorizontalAlignment::Right,
        FontColor::Transparent(COLS.needle),
        display,
    )?;

    // dependend on vario_mode draw speed_to_fly or average_climb_rate
    match cm.control.vario_mode {
        VarioMode::Vario => {
            display.draw_img(SPIRAL_IMG, SZS.pic_left_under_pos)?;
            display.draw_img(M_S_IMG, SZS.left_under_pos)?;
            let acr = num::clamp(cm.sensor.average_climb_rate.to_m_s(), -9.9, 99.9);
            let txt = Concat::<10>::from_f32(acr, 1);
            FONT_HELV_18.render_aligned(
                txt.as_str(),
                SZS.left_under_pos,
                VerticalPosition::Top,
                HorizontalAlignment::Right,
                FontColor::Transparent(COLS.average_climb_rate),
                display,
            )?;
        }
        VarioMode::SpeedToFly => {
            display.draw_img(STRAIGHT_IMG, SZS.pic_left_under_pos)?;
            display.draw_img(KM_H_IMG, SZS.left_under_pos)?;
            let stf = num::clamp(-cm.calculated.speed_to_fly_dif.to_km_h() / 10.0, -5.0, 5.0);
            let angle_sweep = (VARIO_SIZES.angle_m_s * stf).deg();
            Arc::with_center(CENTER, SZS.diameter_stf, 180.0.deg(), angle_sweep)
                .into_styled(PrimitiveStyle::with_stroke(COLS.speed_to_fly, 6))
                .draw(display)?;
            let stf = num::clamp(cm.calculated.speed_to_fly_1s.to_km_h(), 0.0, 999.0);
            let txt = Concat::<10>::from_f32(stf, 0);
            FONT_HELV_18.render_aligned(
                txt.as_str(),
                SZS.left_under_pos,
                VerticalPosition::Top,
                HorizontalAlignment::Right,
                FontColor::Transparent(COLS.speed_to_fly),
                display,
            )?;
        }
    }

    Ok(())
}
