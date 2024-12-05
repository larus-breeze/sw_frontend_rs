use super::{
    helpers::sprites::*,
    helpers::themes::Palette,
    CENTER,
    RADIUS,
    VARIO_SIZES,
};
use crate::{
    basic_config::*,
    model::{CoreModel, FlyMode, SystemState, VarioMode},
    system_of_units::FloatToSpeed,
    tformat,
    utils::{Colors, FONT_BIG},
    CoreError, DrawImage,
};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use VARIO_SIZES as SZS;

// Limits of the wind arrow
const WIND_MIN: f32 = 10.0; // 10 km/h
const WIND_MAX: f32 = 30.0; // 30 km/h

pub fn draw_thermal_climb<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    display.draw_img(
        SPIRAL_IMG,
        SZS.pic_info_1_pos,
        Some(cm.color(Palette::PicInfo1)),
    )?;
    display.draw_img(M_S_IMG, SZS.info_1_pos, Some(cm.color(Palette::Scale)))?;
    let acr = num::clamp(cm.calculated.thermal_climb_rate.to_m_s(), -9.9, 99.9);
    let txt = tformat!(10, "{:.1}", acr).unwrap();
    FONT_BIG.render_aligned(
        txt.as_str(),
        SZS.info_1_pos,
        VerticalPosition::Top,
        HorizontalAlignment::Right,
        FontColor::Transparent(cm.color(Palette::Scale)),
        display,
    )?;
    Ok(())
}

pub struct Vario {}

impl Vario {
    pub fn new(_cm: &CoreModel) -> Vario {
        Vario {}
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        // draaw wallpaper
        display.clear(cm.color(Palette::Background))?;
        display.draw_img(
            WP_VARIO_IMG,
            Point::new(0, 0),
            Some(cm.color(Palette::Scale)),
        )?;
        display.draw_img(M_S_IMG, SZS.unit_pos, Some(cm.color(Palette::Background)))?;

        for (pos_x, pos_y, txt) in WP_VARIO_SCALE {
            let pos = Point::new(pos_x, pos_y);
            FONT_BIG.render(
                txt,
                pos,
                VerticalPosition::Baseline,
                FontColor::Transparent(cm.color(Palette::Background)),
                display,
            )?;
        }

        // draw battery symbol
        if cm.device.supply_voltage > cm.device.voltage_limit_good {
            display.draw_img(BAT_FULL_IMG, SZS.bat_pos, Some(cm.color(Palette::SignalGo)))?;
        } else if cm.device.supply_voltage < cm.device.voltage_limit_bad {
            display.draw_img(
                BAT_EMPTY_IMG,
                SZS.bat_pos,
                Some(cm.color(Palette::SignalStop)),
            )?;
        } else {
            display.draw_img(
                BAT_HALF_IMG,
                SZS.bat_pos,
                Some(cm.color(Palette::SignalWarning)),
            )?;
        }

        // draw sat symbol
        let color = match cm.control.system_state {
            SystemState::NoCom => cm.color(Palette::SignalStop),
            SystemState::CanOk => cm.color(Palette::SignalWarning),
            SystemState::CanAndGpsOk => cm.color(Palette::SignalGo),
        };
        display.draw_img(SAT_IMG, SZS.sat_pos, Some(color))?;

        // draw mc_ready marker
        scale_marker(
            display,
            CENTER,
            cm.config.mc_cready.to_m_s(),
            RADIUS as i32 + 1,
            SZS.mc_len as i32,
            SZS.mc_width,
            cm.color(Palette::Needle2),
        )?;

        // draw wind arrow
        let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
        let (mut angle, mut av_angle, fill_color, stroke_color) = match cm.control.fly_mode {
            FlyMode::Circling => {
                // draw north symbol
                display.draw_img(
                    NORTH_IMG,
                    SZS.north_pos,
                    Some(cm.color(Palette::Background)),
                )?;
                // return absolut wind vector
                (
                    cm.sensor.wind_vector.angle(),
                    cm.sensor.average_wind.angle(),
                    cm.color(Palette::Sprite2Fill),
                    cm.color(Palette::Sprite2Stroke),
                )
            }
            FlyMode::StraightFlight => {
                // draw glider symbol
                display.draw_img(GLIDER_IMG, SZS.glider_pos, Some(cm.color(Palette::Scale)))?;
                (
                    // return relativ wind vector
                    cm.sensor.wind_vector.angle() - cm.sensor.gps_track,
                    cm.sensor.average_wind.angle() - cm.sensor.gps_track,
                    cm.color(Palette::Sprite1Fill),
                    cm.color(Palette::Sprite1Stroke),
                )
            }
        };

        // draw wind arrow
        let txt_angle = if cm.sensor.airspeed.ias() < 30.0.km_h() {
            angle = 180.0.deg(); // The sensor box should actually do this
            av_angle = 180.0.deg();
            cm.sensor.euler_yaw
        } else {
            cm.sensor.wind_vector.angle()
        };

        let len = match wind_speed {
            x if x < WIND_MIN => SZS.wind_len_min, // Light wind is set to a minimum size
            x if x > WIND_MAX => SZS.wind_len,     // Strong wind is set to a maximum size
            _ => {
                SZS.wind_len_min
                    + ((SZS.wind_len - SZS.wind_len_min) as f32 * (wind_speed - WIND_MIN)
                        / (WIND_MAX - WIND_MIN)) as i32
            }
        };
        let avg_wind_spped = cm.sensor.average_wind.speed().to_km_h();
        let delta_speed = wind_speed - avg_wind_spped;
        let (delta_txt, delta_color) = if delta_speed < 0.0 {
            (
                tformat!(5, "{:.0}", delta_speed).unwrap(),
                cm.color(Palette::WindMinus),
            )
        } else {
            (
                tformat!(5, "+{:.0}", delta_speed).unwrap(),
                cm.color(Palette::WindPlus),
            )
        };
        let tail_thick = (num::clamp(num::abs(delta_speed), 1.0, 10.0)) as u32;
        wind_arrow(
            display,
            CENTER,
            angle,
            av_angle,
            len,
            fill_color,
            stroke_color,
            tail_thick,
            delta_color,
        )?;

        // draw wind direction an speed text
        display.draw_img(KM_H_IMG, SZS.wind_pos, Some(cm.color(Palette::Scale)))?;
        let wind_deg = txt_angle.to_degrees();
        let s = tformat!(25, "{:.0}° {:.0}", wind_deg, wind_speed).unwrap();
        FONT_BIG.render_aligned(
            s.as_str(),
            SZS.wind_pos,
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(cm.color(Palette::Scale)),
            display,
        )?;

        FONT_BIG.render_aligned(
            delta_txt.as_str(),
            SZS.delta_pos,
            VerticalPosition::Top,
            HorizontalAlignment::Center,
            FontColor::Transparent(delta_color),
            display,
        )?;

        // draw software version during the first 10 seconds
        if false {
            let s = cm.config.sw_version.as_string();
            FONT_BIG.render_aligned(
                s.as_str(),
                SZS.version_pos,
                VerticalPosition::Top,
                HorizontalAlignment::Right,
                FontColor::Transparent(cm.color(Palette::Scale)),
                display,
            )?;
        }

        // dependend on vario_mode draw speed_to_fly or average_climb_rate
        match cm.control.vario_mode {
            VarioMode::Vario => {
                draw_thermal_climb(display, cm)?;
            }
            VarioMode::SpeedToFly => {
                let stf = num::clamp(-cm.calculated.speed_to_fly_dif.to_km_h() / 10.0, -5.0, 5.0);
                let angle_sweep = (VARIO_SIZES.angle_m_s * stf).deg();
                let col = cm.color(Palette::VarioSpeedToFly);
                Arc::with_center(CENTER, SZS.stf_diameter, 180.0.deg(), angle_sweep)
                    .into_styled(PrimitiveStyle::with_stroke(col, SZS.stf_width))
                    .draw(display)?;

                if cm.config.alt_stf_thermal_climb {
                    display.draw_img(
                        STRAIGHT_IMG,
                        SZS.pic_info_1_pos,
                        Some(cm.color(Palette::Scale)),
                    )?;
                    display.draw_img(KM_H_IMG, SZS.info_1_pos, Some(cm.color(Palette::Scale)))?;
                    let stf = num::clamp(cm.calculated.speed_to_fly_1s.to_km_h(), 0.0, 999.0);
                    let txt = tformat!(10, "{:.0}", stf).unwrap();
                    FONT_BIG.render_aligned(
                        txt.as_str(),
                        SZS.info_1_pos,
                        VerticalPosition::Top,
                        HorizontalAlignment::Right,
                        FontColor::Transparent(col),
                        display,
                    )?;
                } else {
                    draw_thermal_climb(display, cm)?;
                }
            }
        }

        // draw average climb rate marker
        inverted_scale_marker(
            display,
            CENTER,
            cm.calculated.av2_climb_rate.to_m_s(),
            (RADIUS - SZS.indicator_len) as i32,
            SZS.tcr_len as i32,
            SZS.tcr_width,
            cm.color(Palette::Needle3),
        )?;

        // draw average climb rate text
        let s = if cm.calculated.av2_climb_rate.to_m_s() < 0.0 {
            tformat!(5, "{:.1}", cm.calculated.av2_climb_rate.to_m_s()).unwrap()
        } else {
            tformat!(5, "+{:.1}", cm.calculated.av2_climb_rate.to_m_s()).unwrap()
        };
        display.draw_img(M_S_IMG, SZS.avg_climb_pos, Some(cm.color(Palette::Scale)))?;
        FONT_BIG.render_aligned(
            s.as_str(),
            SZS.avg_climb_pos,
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(cm.color(Palette::Scale)),
            display,
        )?;

        // draw climb rate indicator
        let angle = (SZS.angle_m_s * num::clamp(cm.sensor.climb_rate.to_m_s(), -5.1, 5.1)).deg();
        classic_indicator(
            display,
            CENTER,
            angle,
            SZS.indicator_width as i32,
            (RADIUS - SZS.indicator_len) as i32,
            RADIUS as i32,
            cm.color(Palette::Needle1),
        )?;
        Ok(())
    }
}
