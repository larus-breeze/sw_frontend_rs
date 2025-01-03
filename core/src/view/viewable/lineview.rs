use crate::{tformat, Colors, CoreError, CoreModel, DrawImage, FloatToSpeed};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

#[allow(unused)]
use micromath::F32Ext;

/// This enum is also used to reload configurations saved in the EEPROM. Therefore, the sequence
/// must not be changed, as otherwise existing configurations would change. New viewables should
/// always be inserted before the last enum (LastElementNotInUse)
#[derive(Clone, Copy, PartialEq)]
pub enum LineView {
    None,
    AverageClimbRate,
    FlightLevel,
    TrueCourse,
    UtcTime,
    WindAndDelta,
    DriftAngle,
    WindAndAvgWind,
    LastElemntNotInUse,
}

impl core::convert::From<u32> for LineView {
    fn from(value: u32) -> Self {
        let idx = if value >= Self::LastElemntNotInUse as u32 - 1 {
            Self::LastElemntNotInUse as u8 - 1
        } else {
            value as u8
        };
        // Transmute is ok, as idx is guaranteed to be in the valid range
        unsafe { core::mem::transmute::<u8, LineView>(idx) }
    }
}

const TOP_LINE_VIEW: [LineView; 6] = [
    LineView::None,
    LineView::AverageClimbRate,
    LineView::DriftAngle,
    LineView::FlightLevel,
    LineView::TrueCourse,
    LineView::UtcTime,
];

const BOTTOM_LINE_VIEW: [LineView; 8] = [
    LineView::None,
    LineView::AverageClimbRate,
    LineView::DriftAngle,
    LineView::FlightLevel,
    LineView::TrueCourse,
    LineView::UtcTime,
    LineView::WindAndAvgWind,
    LineView::WindAndDelta,
];

#[derive(Clone, Copy)]
pub enum Placement {
    Top,
    Bottom,
}

impl LineView {
    pub const fn max(placement: Placement) -> usize {
        match placement {
            Placement::Bottom => BOTTOM_LINE_VIEW.len() - 1,
            Placement::Top => TOP_LINE_VIEW.len() - 1,
        }
    }

    // This method is used by the editor to obtain the correct viewables in the correct order
    pub fn from_sorted(value: usize, placement: Placement) -> LineView {
        match placement {
            Placement::Bottom => {
                if value < BOTTOM_LINE_VIEW.len() {
                    return BOTTOM_LINE_VIEW[value];
                }
            }
            Placement::Top => {
                if value < TOP_LINE_VIEW.len() {
                    return TOP_LINE_VIEW[value];
                }
            }
        }
        return LineView::None; // should never happen
    }

    pub fn sorted_as_i32(&self, placement: Placement) -> i32 {
        match placement {
            Placement::Bottom => {
                for idx in 0..BOTTOM_LINE_VIEW.len() {
                    if *self == BOTTOM_LINE_VIEW[idx] {
                        return idx as i32;
                    };
                }
            }
            Placement::Top => {
                for idx in 0..TOP_LINE_VIEW.len() {
                    if *self == TOP_LINE_VIEW[idx] {
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
            LineView::AverageClimbRate => "Avg Climb Rate",
            LineView::DriftAngle => "Drift Angle",
            LineView::FlightLevel => "Flight Level",
            LineView::TrueCourse => "True Course",
            LineView::UtcTime => "UTC Time",
            LineView::WindAndAvgWind => "Wind, avg Wind",
            LineView::WindAndDelta => "Wind and Delta",
            LineView::None => "None",
            LineView::LastElemntNotInUse => "",
        }
    }

    /// Draw viewable
    pub fn draw<D>(
        &self,
        display: &mut D,
        cm: &CoreModel,
        pos: Point,
        color: Colors,
    ) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        match self {
            LineView::None => Ok(()),
            LineView::AverageClimbRate => draw_average_climb_rate(display, cm, pos, color),
            LineView::DriftAngle => draw_drift_angle(display, cm, pos, color),
            LineView::FlightLevel => draw_flight_level(display, cm, pos, color),
            LineView::TrueCourse => draw_true_course(display, cm, pos, color),
            LineView::UtcTime => draw_utc_time(display, cm, pos, color),
            LineView::WindAndAvgWind => draw_wind_and_avg_wind(display, cm, pos, color),
            LineView::WindAndDelta => draw_wind_and_delta(display, cm, pos, color),
            LineView::LastElemntNotInUse => Ok(()),
        }
    }
}

fn draw_average_climb_rate<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = if cm.calculated.av2_climb_rate.to_m_s() < 0.0 {
        tformat!(5, "{:.1}", cm.calculated.av2_climb_rate.to_m_s()).unwrap()
    } else {
        tformat!(5, "+{:.1}", cm.calculated.av2_climb_rate.to_m_s()).unwrap()
    };
    let txt_x = pos.x - cm.device_const.sizes.display.m_s.width as i32 / 2;
    let result = cm.device_const.big_font.render_aligned(
        s.as_str(),
        Point::new(txt_x, pos.y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;
    if let Some(rectangle) = result {
        let pic_x = txt_x + 2 + (rectangle.size.width / 2) as i32;
        let pic_y = pos.y - (cm.device_const.sizes.display.m_s.height as i32) / 2;
        display.draw_img(
            &cm.device_const.images.m_s,
            Point::new(pic_x, pic_y),
            Some(color),
        )?;
    }
    Ok(())
}

fn draw_drift_angle<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let track = cm.sensor.gps_track.to_degrees();
    let heading = cm.sensor.euler_yaw.to_degrees();
    let mut drift_angle = track - heading;
    if drift_angle.abs() > 360.0 {
        drift_angle = 0.0
    }
    while drift_angle > 180.0 {
        drift_angle -= 360.0 // t: 355 h 5 => 350 correct -10
    }
    while drift_angle < -180.0 {
        drift_angle += 360.0 // t: 5 h 355 => - 350 correct +10
    }
    let s = if drift_angle > 0.0 {
        tformat!(12, "DA +{:.0}°", drift_angle).unwrap()
    } else {
        tformat!(12, "DA {:.0}°", drift_angle).unwrap()
    };
    cm.device_const.big_font.render_aligned(
        s.as_str(),
        pos,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;
    Ok(())
}

fn draw_flight_level<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let mut altitude = cm.sensor.pressure_altitude.qne_altitude().to_ft() / 100.0;
    if altitude < 0.0 {
        // Patch to avoid -0.01 => "FL0-0"
        altitude = 0.0
    }
    let fl = tformat!(10, "FL{:03.0}", altitude).unwrap();

    cm.device_const.big_font.render_aligned(
        fl.as_str(),
        pos,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;
    Ok(())
}

fn draw_true_course<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let tc = cm.sensor.gps_track.to_degrees();
    let s = tformat!(8, "TC {:.0}°", tc).unwrap();
    cm.device_const.big_font.render_aligned(
        s.as_str(),
        pos,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;
    Ok(())
}

fn draw_utc_time<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = cm.sensor.gps_date_time.to_time_string();
    cm.device_const.big_font.render_aligned(
        s.as_str(),
        pos,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;
    Ok(())
}

fn draw_wind_and_avg_wind<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let total_height = (cm.device_const.big_font.get_default_line_height()
        + cm.device_const.small_font.get_default_line_height())
        * 80
        / 100;
    let angle = if cm.sensor.airspeed.ias() < 30.0.km_h() {
        cm.sensor.euler_yaw
    } else {
        cm.sensor.wind_vector.angle()
    };

    let wind_deg = angle.to_degrees();
    let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
    let wind_x = pos.x - cm.device_const.sizes.display.km_h.width as i32 / 2;
    let wind_y = pos.y - (total_height as i32) / 2;
    let s = tformat!(25, "{:.0}° {:.0}", wind_deg, wind_speed).unwrap();
    let result = cm.device_const.big_font.render_aligned(
        s.as_str(),
        Point::new(wind_x, wind_y),
        VerticalPosition::Top,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;

    if let Some(rectangle) = result {
        let pic_x = wind_x + 2 + (rectangle.size.width / 2) as i32;
        display.draw_img(
            &cm.device_const.images.km_h,
            Point::new(pic_x, wind_y),
            Some(color),
        )?;
    }

    let avg_wind_spped = cm.sensor.average_wind.speed().to_km_h();
    let avg_wind_angle = cm.sensor.average_wind.angle().to_degrees();
    let delta_speed = wind_speed - avg_wind_spped;
    let (avg_txt, avg_color) = if delta_speed < 0.0 {
        (
            tformat!(25, "{:.0}° {:.0}", avg_wind_angle, avg_wind_spped).unwrap(),
            cm.palette().vario_wind_minus,
        )
    } else {
        (
            tformat!(25, "{:.0}° {:.0}", avg_wind_angle, avg_wind_spped).unwrap(),
            cm.palette().vario_wind_plus,
        )
    };

    let avg_y = pos.y + (total_height as i32) / 2;
    cm.device_const.small_font.render_aligned(
        avg_txt.as_str(),
        Point::new(pos.x, avg_y),
        VerticalPosition::Bottom,
        HorizontalAlignment::Center,
        FontColor::Transparent(avg_color),
        display,
    )?;
    Ok(())
}

fn draw_wind_and_delta<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let total_height = (cm.device_const.big_font.get_default_line_height()
        + cm.device_const.small_font.get_default_line_height())
        * 80
        / 100;
    let angle = if cm.sensor.airspeed.ias() < 30.0.km_h() {
        cm.sensor.euler_yaw
    } else {
        cm.sensor.wind_vector.angle()
    };

    let wind_deg = angle.to_degrees();
    let wind_speed = cm.sensor.wind_vector.speed().to_km_h();
    let wind_x = pos.x - cm.device_const.sizes.display.km_h.width as i32 / 2;
    let wind_y = pos.y - (total_height as i32) / 2;
    let s = tformat!(25, "{:.0}° {:.0}", wind_deg, wind_speed).unwrap();
    let result = cm.device_const.big_font.render_aligned(
        s.as_str(),
        Point::new(wind_x, wind_y),
        VerticalPosition::Top,
        HorizontalAlignment::Center,
        FontColor::Transparent(color),
        display,
    )?;

    if let Some(rectangle) = result {
        let pic_x = wind_x + 2 + (rectangle.size.width / 2) as i32;
        display.draw_img(
            &cm.device_const.images.km_h,
            Point::new(pic_x, wind_y),
            Some(color),
        )?;
    }

    let avg_wind_spped = cm.sensor.average_wind.speed().to_km_h();
    let delta_speed = wind_speed - avg_wind_spped;
    let (delta_txt, delta_color) = if delta_speed < 0.0 {
        (
            tformat!(5, "{:.0}", delta_speed).unwrap(),
            cm.palette().vario_wind_minus,
        )
    } else {
        (
            tformat!(5, "+{:.0}", delta_speed).unwrap(),
            cm.palette().vario_wind_plus,
        )
    };

    let delta_y = pos.y + (total_height as i32) / 2;
    cm.device_const.small_font.render_aligned(
        delta_txt.as_str(),
        Point::new(pos.x, delta_y),
        VerticalPosition::Bottom,
        HorizontalAlignment::Center,
        FontColor::Transparent(delta_color),
        display,
    )?;
    Ok(())
}
