use super::{
    helpers::sprites::*,
    helpers::images::images::*,
    helpers::themes::Palette,
    CENTER, DIAMETER, RADIUS,
};
use crate::{
    model::{CoreModel, FlyMode, SystemState, VarioMode},
    system_of_units::FloatToSpeed,
    tformat,
    utils::Colors,
    CoreError, DrawImage,
    view::helpers::themes::FONT_BIG, 
};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

struct VarioSizes {
    stf_diameter: u32,
    stf_width: u32,
    indicator_len: u32,
    indicator_width: u32,
    info_1_pos: Point,
    pic_info_1_pos: Point,
    mc_width: f32,
    mc_len: u32,
    tcr_width: f32,
    tcr_len: u32,
    glider_pos: Point,
    north_pos: Point,
    bat_pos: Point,
    sat_pos: Point,
    unit_pos: Point,
    wind_pos: Point,
    delta_pos: Point,
    avg_climb_pos: Point,
    version_pos: Point,
    wind_len: i32,
    wind_len_min: i32,
    angle_m_s: f32,
    wp_vario_scale: [(i32, i32, &'static str); 11],
}

#[cfg(feature = "air_avionics_ad57")]
const DIMS: VarioSizes = VarioSizes {
    stf_diameter: DIAMETER - 80,
    stf_width: 5,
    indicator_len: 37,
    indicator_width: 12,
    info_1_pos: Point::new(40, 258),
    pic_info_1_pos: Point::new(2, 222),
    mc_width: 0.14,
    mc_len: 22,
    tcr_width: 0.25,
    tcr_len: 22,
    glider_pos: Point::new(67, 118),
    north_pos: Point::new(127, 8),
    bat_pos: Point::new(205, 100),
    sat_pos: Point::new(10, 15),
    unit_pos: Point::new(122, 255),
    wind_pos: Point::new(175, 195),
    delta_pos: Point::new(150, 220),
    avg_climb_pos: Point::new(150, 65),
    version_pos: Point::new(200, 200),
    wind_len: 105,
    wind_len_min: 50,
    angle_m_s: 25.0,
    wp_vario_scale:[
        (202, 248, "5"),
        (156, 267, "4"),
        (105, 265, "3"),
        (61, 241, "2"),
        (30, 202, "1"),
        (20, 153, "0"),
        (30, 103, "1"),
        (61, 64, "2"),
        (105, 40, "3"),
        (156, 38, "4"),
        (202, 57, "5"),
    ],
};

#[cfg(feature = "larus_frontend_v1")]
const DIMS: VarioSizes = VarioSizes {
    stf_diameter: DIAMETER - 80,
    stf_width: 10,
    indicator_len: 45,
    indicator_width: 14,
    info_1_pos: Point::new(47, 290),
    pic_info_1_pos: Point::new(9, 254),
    mc_width: 0.16,
    mc_len: 25,
    tcr_width: 0.25,
    tcr_len: 25,
    glider_pos: Point::new(85, 136),
    north_pos: Point::new(135, 12),
    bat_pos: Point::new(215, 90),
    sat_pos: Point::new(10, 10),
    unit_pos: Point::new(130, 292),
    wind_pos: Point::new(195, 217),
    delta_pos: Point::new(147, 245),
    avg_climb_pos: Point::new(170, 70),
    version_pos: Point::new(200, 217),
    wind_len: 105,
    wind_len_min: 50,
    angle_m_s: 24.0,
    wp_vario_scale: [
        (217, 282, "5"),
        (166, 299, "4"),
        (112, 293, "3"),
        (65, 266, "2"),
        (34, 223, "1"),
        (23, 170, "0"),
        (34, 117, "1"),
        (65, 74, "2"),
        (112, 47, "3"),
        (166, 41, "4"),
        (217, 58, "5"),
    ],
};

#[cfg(feature = "larus_frontend_v2")]
const DIMS: VarioSizes = VarioSizes {
    stf_diameter: DIAMETER - 130,
    stf_width: 10,
    indicator_len: 71,
    indicator_width: 18,
    info_1_pos: Point::new(420, 200),
    pic_info_1_pos: Point::new(390, 125),
    mc_width: 0.14,
    mc_len: 33,
    tcr_width: 0.25,
    tcr_len: 33,
    glider_pos: Point::new(129, 205),
    north_pos: Point::new(216, 14),
    bat_pos: Point::new(375, 300),
    sat_pos: Point::new(410, 300),
    unit_pos: Point::new(208, 432),
    wind_pos: Point::new(280, 320),
    delta_pos: Point::new(247, 363),
    avg_climb_pos: Point::new(270, 105),
    version_pos: Point::new(300, 120),
    wind_len: 120,
    wind_len_min: 80,
    angle_m_s: 25.0,
    wp_vario_scale: [
        (338, 413, "5"),
        (261, 445, "4"),
        (178, 441, "3"),
        (104, 402, "2"),
        (54, 336, "1"),
        (36, 255, "0"),
        (54, 174, "1"),
        (104, 108, "2"),
        (178, 69, "3"),
        (261, 65, "4"),
        (338, 97, "5"),
    ],
};

// Limits of the wind arrow
const WIND_MIN: f32 = 10.0; // 10 km/h
const WIND_MAX: f32 = 30.0; // 30 km/h

pub fn draw_thermal_climb<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    display.draw_img(
        SPIRAL_IMG,
        DIMS.pic_info_1_pos,
        Some(cm.color(Palette::PicInfo1)),
    )?;
    display.draw_img(M_S_IMG, DIMS.info_1_pos, Some(cm.color(Palette::Scale)))?;
    let acr = num::clamp(cm.calculated.thermal_climb_rate.to_m_s(), -9.9, 99.9);
    let txt = tformat!(10, "{:.1}", acr).unwrap();
    FONT_BIG.render_aligned(
        txt.as_str(),
        DIMS.info_1_pos,
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
        display.draw_img(M_S_IMG, DIMS.unit_pos, Some(cm.color(Palette::Background)))?;

        for (pos_x, pos_y, txt) in DIMS.wp_vario_scale {
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
            display.draw_img(BAT_FULL_IMG, DIMS.bat_pos, Some(cm.color(Palette::SignalGo)))?;
        } else if cm.device.supply_voltage < cm.device.voltage_limit_bad {
            display.draw_img(
                BAT_EMPTY_IMG,
                DIMS.bat_pos,
                Some(cm.color(Palette::SignalStop)),
            )?;
        } else {
            display.draw_img(
                BAT_HALF_IMG,
                DIMS.bat_pos,
                Some(cm.color(Palette::SignalWarning)),
            )?;
        }

        // draw sat symbol
        let color = match cm.control.system_state {
            SystemState::NoCom => cm.color(Palette::SignalStop),
            SystemState::CanOk => cm.color(Palette::SignalWarning),
            SystemState::CanAndGpsOk => cm.color(Palette::SignalGo),
        };
        display.draw_img(SAT_IMG, DIMS.sat_pos, Some(color))?;

        // draw mc_ready marker
        scale_marker(
            display,
            CENTER,
            cm.config.mc_cready.to_m_s(),
            RADIUS as i32 + 1,
            DIMS.mc_len as i32,
            DIMS.mc_width,
            DIMS.angle_m_s,
            cm.color(Palette::Needle2),
        )?;

        // draw wind arrow
        let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
        let (mut angle, mut av_angle, fill_color, stroke_color) = match cm.control.fly_mode {
            FlyMode::Circling => {
                // draw north symbol
                display.draw_img(
                    NORTH_IMG,
                    DIMS.north_pos,
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
                display.draw_img(GLIDER_IMG, DIMS.glider_pos, Some(cm.color(Palette::Scale)))?;
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
            x if x < WIND_MIN => DIMS.wind_len_min, // Light wind is set to a minimum size
            x if x > WIND_MAX => DIMS.wind_len,     // Strong wind is set to a maximum size
            _ => {
                DIMS.wind_len_min
                    + ((DIMS.wind_len - DIMS.wind_len_min) as f32 * (wind_speed - WIND_MIN)
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
        display.draw_img(KM_H_IMG, DIMS.wind_pos, Some(cm.color(Palette::Scale)))?;
        let wind_deg = txt_angle.to_degrees();
        let s = tformat!(25, "{:.0}° {:.0}", wind_deg, wind_speed).unwrap();
        FONT_BIG.render_aligned(
            s.as_str(),
            DIMS.wind_pos,
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(cm.color(Palette::Scale)),
            display,
        )?;

        FONT_BIG.render_aligned(
            delta_txt.as_str(),
            DIMS.delta_pos,
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
                DIMS.version_pos,
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
                let angle_sweep = (DIMS.angle_m_s * stf).deg();
                let col = cm.color(Palette::VarioSpeedToFly);
                Arc::with_center(CENTER, DIMS.stf_diameter, 180.0.deg(), angle_sweep)
                    .into_styled(PrimitiveStyle::with_stroke(col, DIMS.stf_width))
                    .draw(display)?;

                if cm.config.alt_stf_thermal_climb {
                    display.draw_img(
                        STRAIGHT_IMG,
                        DIMS.pic_info_1_pos,
                        Some(cm.color(Palette::Scale)),
                    )?;
                    display.draw_img(KM_H_IMG, DIMS.info_1_pos, Some(cm.color(Palette::Scale)))?;
                    let stf = num::clamp(cm.calculated.speed_to_fly_1s.to_km_h(), 0.0, 999.0);
                    let txt = tformat!(10, "{:.0}", stf).unwrap();
                    FONT_BIG.render_aligned(
                        txt.as_str(),
                        DIMS.info_1_pos,
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
            (RADIUS - DIMS.indicator_len) as i32,
            DIMS.tcr_len as i32,
            DIMS.tcr_width,
            DIMS.angle_m_s,
            cm.color(Palette::Needle3),
        )?;

        // draw average climb rate text
        let s = if cm.calculated.av2_climb_rate.to_m_s() < 0.0 {
            tformat!(5, "{:.1}", cm.calculated.av2_climb_rate.to_m_s()).unwrap()
        } else {
            tformat!(5, "+{:.1}", cm.calculated.av2_climb_rate.to_m_s()).unwrap()
        };
        display.draw_img(M_S_IMG, DIMS.avg_climb_pos, Some(cm.color(Palette::Scale)))?;
        FONT_BIG.render_aligned(
            s.as_str(),
            DIMS.avg_climb_pos,
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(cm.color(Palette::Scale)),
            display,
        )?;

        // draw climb rate indicator
        let angle = (DIMS.angle_m_s * num::clamp(cm.sensor.climb_rate.to_m_s(), -5.1, 5.1)).deg();
        classic_indicator(
            display,
            CENTER,
            angle,
            DIMS.indicator_width as i32,
            (RADIUS - DIMS.indicator_len) as i32,
            RADIUS as i32,
            cm.color(Palette::Needle1),
        )?;
        Ok(())
    }
}
