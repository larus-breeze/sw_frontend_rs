use super::{
    colors::Colors,
    elements::{classic_indicator, scale_marker, wind_arrow},
    CENTER, DIAMETER, MARGIN, RADIUS,
};
use crate::core_model::{CoreModel, FlyMode, VarioMode};
use crate::{
    utils::{FONT_HELV_14, FONT_HELV_18},
    Concat, CoreError, DrawImage,
};

use embedded_graphics::{
    geometry::{Angle, AngleUnit},
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

const INDICATOR_LEN: u32 = 40;
const INDICATOR_WIDTH: u32 = 8;
const POS_INTEGRATOR: Point = Point::new(40, 280);
const POS_WIND_KMH: Point = Point::new(220, 100);
const POS_WIND_DEG: Point = Point::new(125, 100);

const MC_WIDTH: f32 = 0.14;
const MC_LEN: u32 = 20;

const WIND_LEN: i32 = 105;
const WIND_MIN: f32 = 10.0; // 10 km/h
const WIND_MAX: f32 = 30.0; // 30 km/h

const VARIO_COLOR: Colors = Colors::White;
const AVERAGE_CLIMB_COLOR: Colors = Colors::Yellow;
const WIND_COLOR: Colors = Colors::Magenta;
const AVERAGE_WIND_COLOR: Colors = Colors::LightSkyBlue;
const SPEED_TO_FLY_COLOR: Colors = Colors::Coral;
const MC_READY_COLOR: Colors = Colors::Red;
const STROKE_COLOR: Colors = Colors::DarkGray;
const BACKGROUND: Colors = Colors::Black;

const GLIDER: &[u8] = include_bytes!("../../assets/glider227.lif");
const COMPASS: &[u8] = include_bytes!("../../assets/compass227.lif");
const WALLPAPER: &[u8] = include_bytes!("../../assets/vario_wallpaper227.lif");
const SPIRAL: &[u8] = include_bytes!("../../assets/spiral227.lif");
const STRAIGHT: &[u8] = include_bytes!("../../assets/straight227.lif");

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

// Draw a wind arrow on the display. This routine is called twice.
fn draw_wind<D>(
    display: &mut D,
    wind_speed: f32,
    angle: Angle,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError>,
{
    let len = match wind_speed {
        x if x < WIND_MIN => WIND_LEN / 3, // Light wind is set to a minimum size
        x if x > WIND_MAX => WIND_LEN,     // Strong wind is set to a maximum size
        _ => (wind_speed * ((WIND_LEN as f32 * 0.666) / (WIND_MAX - WIND_MIN))) as i32,
    };
    wind_arrow(display, CENTER, angle, len, color)
}

/// Draw the vario display on Screen
pub fn draw<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    // Draw Wallpaper
    display.clear(BACKGROUND)?;
    display.draw_img(WALLPAPER, Point::new(0, 0))?;
    for (pos_x, pos_y, txt) in WALLPAPER_SCALE {
        let pos = Point::new(pos_x, pos_y);
        let _ = FONT_HELV_18.render(
            txt,
            pos,
            VerticalPosition::Baseline,
            FontColor::Transparent(STROKE_COLOR),
            display,
        );
    }

    // Dependend on fly_mode draw glider or compass
    match cm.modes.fly_mode {
        FlyMode::Circling => display.draw_img(COMPASS, Point::new(0, 0))?,
        FlyMode::StraightFlight => display.draw_img(GLIDER, Point::new(0, 0))?,
    }

    // Dependend on vario_mode draw speed_to_fly or average_climb_rate
    match cm.modes.vario_mode {
        VarioMode::Vario => {
            display.draw_img(SPIRAL, Point::new(2, 222))?;
            let acr = num::clamp(cm.measured.average_climb_rate.to_m_s(), -9.9, 99.9);
            let txt = Concat::<10>::from_f32(acr, 1);
            let _ = FONT_HELV_18.render_aligned(
                txt.as_str(),
                POS_INTEGRATOR,
                VerticalPosition::Baseline,
                HorizontalAlignment::Right,
                FontColor::Transparent(AVERAGE_CLIMB_COLOR),
                display,
            );
        }
        VarioMode::SpeedToFly => {
            display.draw_img(STRAIGHT, Point::new(2, 222))?;
            let angle_sweep = (2.5 * cm.calculated.speed_to_fly_dif.to_m_s()).deg();
            Arc::new(
                Point::new(MARGIN + 7, MARGIN + 6),
                DIAMETER - 14,
                180.0.deg(),
                angle_sweep,
            )
            .into_styled(PrimitiveStyle::with_stroke(SPEED_TO_FLY_COLOR, 12))
            .draw(display)?;
            let stf = num::clamp(cm.measured.speed_to_fly.to_km_h(), 0.0, 999.0);
            let txt = Concat::<10>::from_f32(stf, 0);
            let _ = FONT_HELV_18.render_aligned(
                txt.as_str(),
                POS_INTEGRATOR,
                VerticalPosition::Baseline,
                HorizontalAlignment::Right,
                FontColor::Transparent(SPEED_TO_FLY_COLOR),
                display,
            );
        }
    }

    // draw mc_ready marker and vario indicator
    scale_marker(
        display,
        CENTER,
        cm.calculated.mc_cready.to_m_s(),
        RADIUS as i32,
        MC_LEN as i32,
        MC_WIDTH,
        MC_READY_COLOR,
    )?;
    let angle = (25.0 * num::clamp(cm.measured.climb_rate.to_m_s(), -5.1, 5.1)).deg();
    classic_indicator(
        display,
        CENTER,
        angle,
        INDICATOR_WIDTH as i32,
        (RADIUS - INDICATOR_LEN) as i32,
        RADIUS as i32,
        VARIO_COLOR,
    )?;

    // draw wind indicators and text
    draw_wind(
        display,
        cm.measured.average_wind_speed.to_km_h(),
        cm.measured.average_wind_angle,
        AVERAGE_WIND_COLOR,
    )?;
    draw_wind(
        display,
        cm.measured.wind_speed.to_km_h(),
        cm.measured.wind_angle,
        WIND_COLOR,
    )?;

    let wind_deg = cm.measured.wind_angle.to_degrees();
    let s = Concat::<20>::from_f32(wind_deg, 0).push_str("°");
    let _ = FONT_HELV_14.render_aligned(
        s.as_str(),
        POS_WIND_DEG,
        VerticalPosition::Baseline,
        HorizontalAlignment::Right,
        FontColor::Transparent(VARIO_COLOR),
        display,
    );

    let wind_speed = cm.measured.wind_speed.to_km_h();
    let s = Concat::<20>::from_f32(wind_speed, 0).push_str(" km/h");
    let _ = FONT_HELV_14.render_aligned(
        s.as_str(),
        POS_WIND_KMH,
        VerticalPosition::Baseline,
        HorizontalAlignment::Right,
        FontColor::Transparent(VARIO_COLOR),
        display,
    );

    Ok(())
}
