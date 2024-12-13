use crate::{
    CoreModel, DrawImage, CoreError, Colors, tformat, FloatToSpeed,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

#[derive(Clone, Copy)]
pub enum Viewable {
    None,
    AverageClimbRate,
    FlightLevel,
    TrueCourse,
    UtcTime,
    WindAndDelta,
    LastElemntNotInUse,
}

impl core::convert::From<u32> for Viewable {
    fn from(value: u32) -> Self {
        let idx = if value > Self::max() {
            Self::max() as u8
        } else {
            value as u8
        };
        // Transmute is ok, as idx is guaranteed to be in the valid range
        unsafe {
            core::mem::transmute::<u8, Viewable>(idx)
        }
    }
}

impl Viewable {
    pub const fn max() -> u32 {
        Viewable::LastElemntNotInUse as u32 - 1
    }

    pub fn name(&self) -> &'static str {
        match self {
            Viewable::AverageClimbRate => "Avg Climb Rate",
            Viewable::FlightLevel => "Flight Level",
            Viewable::TrueCourse => "True Course",
            Viewable::UtcTime => "UTC Time",
            Viewable::WindAndDelta => "Wind and Delta",
            Viewable::None => "None",
            Viewable::LastElemntNotInUse => "",
        }
    }

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
            Viewable::None => Ok(()),
            Viewable::AverageClimbRate => draw_average_climb_rate(display, cm, pos, color),
            Viewable::FlightLevel => draw_flight_level(display, cm, pos, color),
            Viewable::TrueCourse => draw_true_course(display, cm, pos, color),
            Viewable::UtcTime => draw_utc_time(display, cm, pos, color),
            Viewable::WindAndDelta => draw_wind_and_delta(display, cm, pos, color),
            Viewable::LastElemntNotInUse => Ok(()),
        }
    }
}



pub fn draw_average_climb_rate<D>(
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

pub fn draw_flight_level<D>(
    display: &mut D, 
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let altitude = cm.sensor.pressure_altitude.qne_altitude().to_ft() / 100.0;
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

pub fn draw_true_course<D>(
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

pub fn draw_utc_time<D>(
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

pub fn draw_wind_and_delta<D>(
    display: &mut D, 
    cm: &CoreModel,
    pos: Point,
    color: Colors,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let total_height = 
        (cm.device_const.big_font.get_default_line_height() + cm.device_const.small_font.get_default_line_height()) * 80 / 100;
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