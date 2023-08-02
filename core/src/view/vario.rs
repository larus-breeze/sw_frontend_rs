use super::{
    elements::{classic_indicator, inverted_scale_marker, scale_marker, wind_arrow},
    CENTER, DIAMETER, RADIUS,
};
use crate:: {
    model::{CoreModel, FlyMode, VarioMode},
    utils::Colors,
};
use crate::{utils::FONT_HELV_18, Concat, CoreError, DrawImage};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

struct VarioSizes {
    diameter_stf: u32,
    indicator_len: u32,
    indicator_width: u32,
    left_under_pos: Point,
    pic_left_under_pos: Point,
    mc_width: f32,
    mc_len: u32,
    tcr_width: f32,
    tcr_len: u32,
    unit_pos: Point,
    wind_pos: Point,
    wind_len: i32,
    wind_len_min: i32,
}

const VARIO_SIZES: VarioSizes = VarioSizes {
    diameter_stf: DIAMETER - 108,
    indicator_len: 50,
    indicator_width: 8,
    left_under_pos: Point::new(40, 258),
    pic_left_under_pos: Point::new(2, 222),
    mc_width: 0.14,
    mc_len: 22,
    tcr_width: 0.25,
    tcr_len: 22,
    unit_pos: Point::new(122, 255),
    wind_pos: Point::new(180, 85),
    wind_len: 105,
    wind_len_min: 50,
};

use VARIO_SIZES as SZS;

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

// Predefined (background) images
const GLIDER: &[u8] = include_bytes!("../../assets/glider227.lif");
const COMPASS: &[u8] = include_bytes!("../../assets/compass227.lif");
const WALLPAPER: &[u8] = include_bytes!("../../assets/vario_wallpaper227.lif");
const SPIRAL: &[u8] = include_bytes!("../../assets/spiral227.lif");
const STRAIGHT: &[u8] = include_bytes!("../../assets/straight227.lif");
const KM_H: &[u8] = include_bytes!("../../assets/km_h.lif");
const M_S: &[u8] = include_bytes!("../../assets/m_s.lif");

// These are precalculated coordinates to draw the numbers on the display
// see /assets/create_wallpaper_vario.py
const WALLPAPER_SCALE: [(i32, i32, &str); 11] = [
    (194, 238, "5"),
    (152, 255, "4"),
    (106, 253, "3"),
    (66, 232, "2"),
    (38, 196, "1"),
    (29, 152, "0"),
    (38, 107, "1"),
    (66, 71, "2"),
    (106, 50, "3"),
    (152, 48, "4"),
    (194, 65, "5"),
];

/// draw the vario display on Screen
pub fn draw<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    // draw wallpaper
    display.clear(COLS.background)?;
    display.draw_img(WALLPAPER, Point::new(0, 0))?;
    display.draw_img(M_S, SZS.unit_pos)?;

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
        FlyMode::Circling => display.draw_img(COMPASS, Point::new(0, 0))?,
        FlyMode::StraightFlight => display.draw_img(GLIDER, Point::new(0, 0))?,
    }

    // draw mc_ready marker
    scale_marker(
        display,
        CENTER,
        cm.calculated.mc_cready.to_m_s(),
        RADIUS as i32,
        SZS.mc_len as i32,
        SZS.mc_width,
        COLS.mc_cready,
    )?;

    // draw thermal climb rate marker
    inverted_scale_marker(
        display,
        CENTER,
        cm.calculated.thermal_climb_rate.to_m_s(),
        (RADIUS - SZS.indicator_len) as i32,
        SZS.tcr_len as i32,
        SZS.tcr_width,
        COLS.thermal_climb_rate,
    )?;

    // draw climb rate indicator
    let angle = (25.0 * num::clamp(cm.measured.climb_rate.to_m_s(), -5.1, 5.1)).deg();
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
    let wind_speed = cm.measured.wind_speed.to_km_h();
    let angle = cm.measured.wind_angle;
    let av_angle = cm.measured.average_wind_angle;
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
    display.draw_img(KM_H, SZS.wind_pos)?;
    let wind_deg = cm.measured.wind_angle.to_degrees();
    let wind_speed = cm.measured.wind_speed.to_km_h();
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
            display.draw_img(SPIRAL, SZS.pic_left_under_pos)?;
            let acr = num::clamp(cm.measured.average_climb_rate.to_m_s(), -9.9, 99.9);
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
            display.draw_img(STRAIGHT, SZS.pic_left_under_pos)?;
            display.draw_img(KM_H, SZS.left_under_pos)?;
            let stf = num::clamp(cm.calculated.speed_to_fly_dif.to_km_h(), -50.0, 50.0);
            let angle_sweep = (-2.5 * stf).deg();
            Arc::with_center(CENTER, SZS.diameter_stf, 180.0.deg(), angle_sweep)
                .into_styled(PrimitiveStyle::with_stroke(COLS.speed_to_fly, 6))
                .draw(display)?;
            let stf = num::clamp(cm.calculated.speed_to_fly.ias().to_km_h(), 0.0, 999.0);
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
