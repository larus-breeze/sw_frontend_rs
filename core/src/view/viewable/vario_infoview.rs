use crate::{
    model::DataSource, tformat, Colors, CoreError, CoreModel, DrawImage, FloatToSpeed, Image,
    Palette, Speed,
};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point};
use num_enum::FromPrimitive;
use u8g2_fonts::{
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

#[allow(unused)]
use micromath::F32Ext;

/// This enum is also used to reload configurations saved in the EEPROM. Therefore, the sequence
/// must not be changed, as otherwise existing configurations would change. New viewables should
/// always be inserted before the last enum (LastElementNotInUse)
#[derive(Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum LineView {
    #[default]
    None,
    AverageClimbRate,
    FlightLevel,
    TrueCourse,
    UtcTime,
    WindAndDelta,
    DriftAngle,
    WindAndAvgWind,
    SpeedToFly,
    TrueAirSpeed,
    BatteryVoltage,
    GLoad,
    CircleDiameter,
    CircleMaxMin,
    Heading,
    BankAngle,
    SlipAngle,
    IndicatedAirSpeed,
    EquivalentAirspeed,
    PitchAngle,
    LastElemntNotInUse,
}

const TOP_LINE_VIEW: &[LineView] = &[
    LineView::None,
    LineView::AverageClimbRate,
    LineView::BankAngle,
    LineView::BatteryVoltage,
    LineView::CircleDiameter,
    LineView::CircleMaxMin,
    LineView::DriftAngle,
    LineView::EquivalentAirspeed,
    LineView::FlightLevel,
    LineView::GLoad,
    LineView::Heading,
    LineView::IndicatedAirSpeed,
    LineView::PitchAngle,
    LineView::SlipAngle,
    LineView::SpeedToFly,
    LineView::TrueAirSpeed,
    LineView::TrueCourse,
    LineView::UtcTime,
];

const BOTTOM_LINE_VIEW: &[LineView] = &[
    LineView::None,
    LineView::AverageClimbRate,
    LineView::BankAngle,
    LineView::BatteryVoltage,
    LineView::CircleDiameter,
    LineView::CircleMaxMin,
    LineView::DriftAngle,
    LineView::EquivalentAirspeed,
    LineView::FlightLevel,
    LineView::GLoad,
    LineView::Heading,
    LineView::IndicatedAirSpeed,
    LineView::PitchAngle,
    LineView::SlipAngle,
    LineView::SpeedToFly,
    LineView::TrueAirSpeed,
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
        LineView::None // should never happen
    }

    pub fn sorted_as_i32(&self, placement: Placement) -> i32 {
        match placement {
            Placement::Bottom => {
                for (idx, view_item) in BOTTOM_LINE_VIEW.iter().enumerate() {
                    if *self == *view_item {
                        return idx as i32;
                    };
                }
            }
            Placement::Top => {
                for (idx, view_item) in TOP_LINE_VIEW.iter().enumerate() {
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
            LineView::AverageClimbRate => "Avg Climb Rate",
            LineView::DriftAngle => "Drift Angle",
            LineView::FlightLevel => "Flight Level",
            LineView::Heading => "Heading",
            LineView::SpeedToFly => "Speed to Fly",
            LineView::TrueAirSpeed => "True Air Speed",
            LineView::TrueCourse => "True Course",
            LineView::UtcTime => "UTC Time",
            LineView::BatteryVoltage => "Battery Voltage",
            LineView::WindAndAvgWind => "Wind, avg Wind",
            LineView::WindAndDelta => "Wind and Delta",
            LineView::GLoad => "G-Load",
            LineView::CircleDiameter => "Circle Diameter",
            LineView::CircleMaxMin => "Circle Max-Min",
            LineView::BankAngle => "Bank Angle",
            LineView::SlipAngle => "Slip Angle",
            LineView::IndicatedAirSpeed => "Indicated Air Speed",
            LineView::EquivalentAirspeed => "Equivalent Air Speed",
            LineView::PitchAngle => "Pitch Angle",
            LineView::None => "None",
            LineView::LastElemntNotInUse => "",
        }
    }

    /// Draw viewable
    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        match self {
            LineView::None => Ok(()),
            LineView::AverageClimbRate => draw_average_climb_rate(display, cm, pos),
            LineView::DriftAngle => draw_drift_angle(display, cm, pos),
            LineView::FlightLevel => draw_flight_level(display, cm, pos),
            LineView::Heading => draw_heading(display, cm, pos),
            LineView::SpeedToFly => draw_speed_to_fly(display, cm, pos),
            LineView::TrueAirSpeed => draw_true_air_speed(display, cm, pos),
            LineView::TrueCourse => draw_true_course(display, cm, pos),
            LineView::UtcTime => draw_utc_time(display, cm, pos),
            LineView::BatteryVoltage => draw_battery_voltage(display, cm, pos),
            LineView::WindAndAvgWind => draw_wind_and_avg_wind(display, cm, pos),
            LineView::WindAndDelta => draw_wind_and_delta(display, cm, pos),
            LineView::GLoad => draw_g_load(display, cm, pos),
            LineView::CircleDiameter => draw_circle_diameter(display, cm, pos),
            LineView::CircleMaxMin => draw_circle_max_min(display, cm, pos),
            LineView::BankAngle => draw_bank_angle(display, cm, pos),
            LineView::SlipAngle => draw_slip_angle(display, cm, pos),
            LineView::IndicatedAirSpeed => draw_indicated_air_speed(display, cm, pos),
            LineView::EquivalentAirspeed => draw_equivalent_air_speed(display, cm, pos),
            LineView::PitchAngle => draw_pitch_angle(display, cm, pos),
            LineView::LastElemntNotInUse => Ok(()),
        }
    }
}

fn draw_centered_line<D>(
    display: &mut D,
    pos: Point,
    img1: Option<Image>,
    content: &str,
    img2: Option<Image>,
    font: &FontRenderer,
    color: &Palette,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let mut txt_x = pos.x;
    if let Some(img) = img1 {
        txt_x += img.width() as i32 / 2;
    }
    if let Some(img) = img2 {
        txt_x -= img.width() as i32 / 2;
    }
    let result = font.render_aligned(
        content,
        Point::new(txt_x, pos.y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(color.vario.value),
        display,
    )?;
    if let Some(rectangle) = result {
        if let Some(img) = img1 {
            let pic_x = txt_x - (rectangle.size.width / 2 + img.width()) as i32;
            let pic_y = pos.y - img.height() as i32 / 2;
            img.draw(display, Point::new(pic_x, pic_y), Some(color.vario.icon))?;
        }
        if let Some(img) = img2 {
            let pic_x = txt_x + rectangle.size.width as i32 / 2;
            let pic_y = pos.y - img.height() as i32 / 2;
            img.draw(display, Point::new(pic_x, pic_y), Some(color.vario.unit))?;
        }
    }

    Ok(())
}

fn draw_average_climb_rate<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = match cm.control.avg_climb_rate_src {
        DataSource::Frontend => cm
            .config
            .unit_vertical_speed
            .value_str(cm.calculated.av2_climb_rate),
        DataSource::Sensorbox => cm
            .config
            .unit_vertical_speed
            .value_str(cm.sensor.average_climb_rate),
    };
    let img1 = Some(Image::new(cm.device_const.images.avg_climb_rate));
    let img2 = Some(cm.config.unit_vertical_speed.image(cm));
    draw_centered_line(
        display,
        pos,
        img1,
        s.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_drift_angle<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
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
        tformat!(12, "+{:.0}°", drift_angle).unwrap()
    } else {
        tformat!(12, "{:.0}°", drift_angle).unwrap()
    };

    let img1 = Some(Image::new(cm.device_const.images.drift_angle));
    let img2 = None;
    draw_centered_line(
        display,
        pos,
        img1,
        s.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_flight_level<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let mut altitude = cm.sensor.pressure_altitude.qne_altitude().to_ft() / 100.0;
    if altitude < 0.0 {
        // Patch to avoid -0.01 => "FL0-0"
        altitude = 0.0
    }
    let fl = tformat!(10, "{:03.0}", altitude).unwrap();

    let img1 = Some(Image::new(cm.device_const.images.flight_level));
    let img2 = None;
    draw_centered_line(
        display,
        pos,
        img1,
        fl.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_speed_to_fly<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let stf = cm
        .config
        .unit_horizontal_speed
        .value_str(cm.calculated.speed_to_fly_1s);
    let img1 = Some(Image::new(cm.device_const.images.speed_to_fly));
    let img2 = Some(cm.config.unit_horizontal_speed.image(cm));
    draw_centered_line(
        display,
        pos,
        img1,
        stf.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_true_air_speed<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let tas = cm
        .config
        .unit_horizontal_speed
        .value_str(cm.sensor.airspeed.tas());
    let img1 = Some(Image::new(cm.device_const.images.tas));
    let img2 = Some(cm.config.unit_horizontal_speed.image(cm));
    draw_centered_line(
        display,
        pos,
        img1,
        tas.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_heading<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let heading = cm.sensor.euler_yaw.to_degrees();
    let s = tformat!(8, "{:.0}°", heading).unwrap();

    draw_centered_line(
        display,
        pos,
        Some(Image::new(cm.device_const.images.yaw)),
        s.as_str(),
        None,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_true_course<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let tc = cm.sensor.gps_track.to_degrees();
    let s = tformat!(8, "{:.0}°", tc).unwrap();

    let img1 = Some(Image::new(cm.device_const.images.true_course));
    let img2 = None;
    draw_centered_line(
        display,
        pos,
        img1,
        s.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_utc_time<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = cm.sensor.gps_date_time.to_time_string();
    cm.device_const.big_font.render_aligned(
        s.as_str(),
        pos,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().vario.value),
        display,
    )?;
    Ok(())
}

fn draw_wind_and_avg_wind<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
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

    let unit_img = cm.config.unit_horizontal_speed.image(cm);
    let wind_deg = angle.to_degrees();
    let wind_speed = cm.sensor.wind_vector.speed();
    let ws_str = cm.config.unit_horizontal_speed.value_str(wind_speed);
    let wind_x = pos.x - unit_img.width() as i32 / 2;
    let wind_y = pos.y - (total_height as i32) / 2;
    let s = tformat!(25, "{:.0}° {}", wind_deg, ws_str.as_str()).unwrap();
    let result = cm.device_const.big_font.render_aligned(
        s.as_str(),
        Point::new(wind_x, wind_y),
        VerticalPosition::Top,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().vario.value),
        display,
    )?;

    if let Some(rectangle) = result {
        let pic_x = wind_x + 2 + (rectangle.size.width / 2) as i32;
        unit_img.draw(
            display,
            Point::new(pic_x, wind_y),
            Some(cm.palette().vario.unit),
        )?;
    }

    let avg_wind_spped = cm.sensor.average_wind.speed();
    let avg_str = cm.config.unit_horizontal_speed.value_str(avg_wind_spped);
    let avg_wind_angle = cm.sensor.average_wind.angle().to_degrees();
    let avg_txt = tformat!(25, "{:.0}° {}", avg_wind_angle, avg_str.as_str()).unwrap();
    let avg_color = cm.palette().vario.wind_diff;

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

fn draw_wind_and_delta<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
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

    let unit_img = cm.config.unit_horizontal_speed.image(cm);
    let wind_deg = angle.to_degrees();
    let wind_speed = cm.sensor.wind_vector.speed();
    let wind_str = cm.config.unit_horizontal_speed.value_str(wind_speed);
    let wind_x = pos.x - unit_img.width() as i32 / 2;
    let wind_y = pos.y - (total_height as i32) / 2;
    let s = tformat!(25, "{:.0}° {}", wind_deg, wind_str.as_str()).unwrap();
    let result = cm.device_const.big_font.render_aligned(
        s.as_str(),
        Point::new(wind_x, wind_y),
        VerticalPosition::Top,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().vario.value),
        display,
    )?;

    if let Some(rectangle) = result {
        let pic_x = wind_x + 2 + (rectangle.size.width / 2) as i32;
        unit_img.draw(
            display,
            Point::new(pic_x, wind_y),
            Some(cm.palette().vario.unit),
        )?;
    }

    let avg_wind_spped = cm.sensor.average_wind.speed();
    let delta_speed = wind_speed - avg_wind_spped;
    let delta_txt = cm.config.unit_horizontal_speed.value_str(delta_speed);
    let delta_y = pos.y + (total_height as i32) / 2;
    let delta_color = cm.palette().vario.wind_diff;

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

#[derive(Clone, Copy, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Info3View {
    #[default]
    None,
    Climbing,
    SpeedToFly,
}

const INFO3_LIST: &[Info3View] = &[Info3View::None, Info3View::Climbing, Info3View::SpeedToFly];

impl Info3View {
    pub fn name(&self) -> &'static str {
        match self {
            Info3View::None => "None",
            Info3View::Climbing => "Climbing",
            Info3View::SpeedToFly => "Speed to fly",
        }
    }

    pub fn max() -> i32 {
        INFO3_LIST.len() as i32 - 1
    }

    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        match self {
            Info3View::None => (),
            Info3View::Climbing => draw_info3_climbing(display, cm)?,
            Info3View::SpeedToFly => draw_info3_stf(display, cm)?,
        }
        Ok(())
    }
}

fn draw_info3_climbing<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;

    display.draw_img(
        cm.device_const.images.spiral,
        sizes.pic_info3_pos,
        Some(cm.palette().vario.icon),
    )?;
    let img = cm.config.unit_vertical_speed.image(cm);
    img.draw(display, sizes.info3_pos, Some(cm.palette().vario.unit))?;

    let txt = cm
        .config
        .unit_vertical_speed
        .value_str(cm.calculated.thermal_climb_rate);
    cm.device_const.big_font.render_aligned(
        txt.as_str(),
        sizes.info3_pos,
        VerticalPosition::Top,
        HorizontalAlignment::Right,
        FontColor::Transparent(cm.palette().vario.scale),
        display,
    )?;
    Ok(())
}

fn draw_info3_stf<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;
    display.draw_img(
        cm.device_const.images.straight,
        sizes.pic_info3_pos,
        Some(cm.palette().vario.icon),
    )?;
    let img = cm.config.unit_horizontal_speed.image(cm);
    img.draw(display, sizes.info3_pos, Some(cm.palette().vario.unit))?;
    let txt = cm
        .config
        .unit_horizontal_speed
        .value_str(cm.calculated.speed_to_fly_1s);
    cm.device_const.big_font.render_aligned(
        txt.as_str(),
        sizes.info3_pos,
        VerticalPosition::Top,
        HorizontalAlignment::Right,
        FontColor::Transparent(cm.palette().vario.scale),
        display,
    )?;
    Ok(())
}

fn draw_indicated_air_speed<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let ias = cm
        .config
        .unit_horizontal_speed
        .value_str(cm.sensor.airspeed.ias());
    let img1 = Some(Image::new(cm.device_const.images.ias));
    let img2 = Some(cm.config.unit_horizontal_speed.image(cm));
    draw_centered_line(
        display,
        pos,
        img1,
        ias.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_equivalent_air_speed<D>(
    display: &mut D,
    cm: &CoreModel,
    pos: Point,
) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let g_load = (cm.sensor.g_force.to_m_s2() / 9.81).max(0.0);
    let ve_str = if g_load > 0.01 {
        let ve_speed = Speed::from_km_h(cm.sensor.airspeed.ias().to_km_h() / g_load.sqrt());
        cm.config.unit_horizontal_speed.value_str(ve_speed)
    } else {
        heapless::String::<3>::try_from("--").unwrap()
    };
    let img1 = Some(Image::new(cm.device_const.images.ve));
    let img2 = Some(cm.config.unit_horizontal_speed.image(cm));
    draw_centered_line(
        display,
        pos,
        img1,
        ve_str.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_bank_angle<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let roll_deg = cm.sensor.euler_roll.to_degrees();
    let s = tformat!(8, "{:.0}°", roll_deg.abs()).unwrap();
    let img_bytes = if roll_deg < 0.0 {
        cm.device_const.images.roll_left
    } else {
        cm.device_const.images.roll_right
    };
    let img = Image::new(img_bytes);
    let txt_x = pos.x + img.width() as i32 / 2;

    let result = cm.device_const.big_font.render_aligned(
        s.as_str(),
        Point::new(txt_x, pos.y),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(cm.palette().vario.value),
        display,
    )?;
    if let Some(rectangle) = result {
        let pic_x = txt_x - (rectangle.size.width / 2 + img.width()) as i32;
        let pic_y = pos.y - img.height() as i32 / 2;
        let pic_pos = Point::new(pic_x, pic_y);
        img.draw(display, pic_pos, Some(cm.palette().vario.icon))?;
    }

    Ok(())
}

fn draw_slip_angle<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = tformat!(8, "{:.0}°", cm.sensor.slip_angle.to_degrees()).unwrap();
    draw_centered_line(
        display,
        pos,
        Some(Image::new(cm.device_const.images.slip)),
        s.as_str(),
        None,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_pitch_angle<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = tformat!(8, "{:.0}°", cm.sensor.euler_pitch.to_degrees()).unwrap();
    draw_centered_line(
        display,
        pos,
        Some(Image::new(cm.device_const.images.pitch)),
        s.as_str(),
        None,
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_g_load<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let g = cm.sensor.g_force.to_m_s2() / 9.81;
    let s = tformat!(10, "{:.2}", g).unwrap();
    draw_centered_line(
        display,
        pos,
        Some(Image::new(cm.device_const.images.g_load)),
        s.as_str(),
        Some(Image::new(cm.device_const.images.g)),
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_circle_diameter<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = if cm.calculated.circle_diameter_valid {
        cm.config
            .unit_height
            .value_str(cm.calculated.circle_diameter)
    } else {
        heapless::String::<5>::try_from("--").unwrap()
    };
    draw_centered_line(
        display,
        pos,
        Some(Image::new(cm.device_const.images.circle_diameter)),
        s.as_str(),
        Some(cm.config.unit_height.image(cm)),
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_circle_max_min<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let s = if cm.calculated.circle_max_min_valid {
        cm.config
            .unit_vertical_speed
            .value_str(cm.calculated.circle_max_min_last)
    } else {
        heapless::String::<5>::try_from("--").unwrap()
    };
    draw_centered_line(
        display,
        pos,
        Some(Image::new(cm.device_const.images.circle_delta)),
        s.as_str(),
        Some(cm.config.unit_vertical_speed.image(cm)),
        &cm.device_const.big_font,
        cm.palette(),
    )
}

fn draw_battery_voltage<D>(display: &mut D, cm: &CoreModel, pos: Point) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let voltage = cm.device.supply_voltage;
    let s = if voltage > 0.1 {
        tformat!(6, "{:.1}", voltage).unwrap()
    } else {
        heapless::String::<6>::try_from("---").unwrap()
    };

    let img1 = if voltage > 0.1 {
        Some(Image::new(cm.device_const.images.battery))
    } else {
        None
    };
    let img2 = Some(Image::new(cm.device_const.images.v));

    draw_centered_line(
        display,
        pos,
        img1,
        s.as_str(),
        img2,
        &cm.device_const.big_font,
        cm.palette(),
    )
}
