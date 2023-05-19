#![no_std]
mod blackboard;
pub use blackboard::Blackboard;

mod view;
pub use view::colors::Colors;
pub use view::rgb565_colors::RGB565_COLORS;

pub(crate) mod fmt;

use embedded_graphics::{mono_font::
    {
        MonoFont, MonoTextStyle, MonoTextStyleBuilder, iso_8859_1::{FONT_10X20, FONT_8X13_BOLD}
    }, 
    prelude::*, 
    primitives::{Circle, Line, Triangle, PrimitiveStyle}, text::Text,
    geometry::{Angle, AngleUnit}
};
#[allow(unused_imports)]
use micromath::F32Ext;


#[cfg(feature = "defmt")]
use cortex_m::peripheral::DWT;

#[cfg(feature = "defmt")]
fn msec_tick() -> u32 {
    DWT::cycle_count() / 100_000
}

#[cfg(not(feature = "defmt"))]
fn msec_tick() -> u32 {
    0
}

#[cfg(not(any(feature = "display_size_240x240")))]
pub const DISPLAY_WIDTH: u32 = 240;
#[cfg(not(any(feature = "display_size_240x240")))]
pub const DISPLAY_HEIGHT: u32 = 240;

#[cfg(feature = "display_size_240x240")]
pub const DISPLAY_WIDTH: u32 = 240;
#[cfg(feature = "display_size_240x240")]
pub const DISPLAY_HEIGHT: u32 = 240;



const DIAMETER: u32 = DISPLAY_HEIGHT;
const RADIUS: u32 = DIAMETER / 2;
const CENTER: Point = Point::new(RADIUS as i32, RADIUS as i32);

const STROKE_LEN: u32 = 18;
const STROKE_WIDTH: u32 = 3;
const STROKE_TEXT_POS: u32 = 30;
const STROKE_FONT: MonoFont = FONT_10X20;
const STROKE_FONT_OFF_X: i32 = -5;
const STROKE_FONT_OFF_Y: i32 = 6;

const INDICATOR_LEN: u32 = 40;
const INDICATOR_WIDTH: u32 = 5;

const SPEED_TO_FLY_X: i32 = 200;
const SPEED_TO_FLY_Y: i32 = (DISPLAY_HEIGHT / 2) as i32;
const SPEED_TO_FLY_WIDTH: i32 = 20;

const WIND_X: i32 = 90;
const WIND_DELTA_Y: i32 = 15;
const WIND_LEN: u32 = 60;
const WIND_FONT: MonoFont = FONT_8X13_BOLD;

const VARIO_COLOR: Colors = Colors::White;
const AVERAGE_CLIMB_COLOR: Colors = Colors::Yellow;
const WIND_COLOR: Colors = Colors::Magenta;
const AVERAGE_WIND_COLOR: Colors = Colors::Blue;
const SPEED_TO_FLY_COLOR: Colors = Colors::Coral;
const MC_READY_COLOR: Colors = Colors::Red;
const STROKE_COLOR: Colors = Colors::Lightgrey;
const BACKGROUND: Colors = Colors::Black;

const WALLPAPER_SCALE: [(f32, &str); 11] = [
    (5.0, "5"),
    (4.0, "4"),
    (3.0, "3"),
    (2.0, "2"),
    (1.0, "1"),
    (0.0, "0"),
    (-1.0, "1"),
    (-2.0, "2"),
    (-3.0, "3"),
    (-4.0, "4"),
    (-5.0, "5"),
];

fn wind_coord(angle: Angle, radius: f32) -> Point {
    CENTER + Point::new(
        (angle.to_radians().sin() * radius) as i32,
        -(angle.to_radians().cos() * radius) as i32,
    )
}

fn vario_coord(angle: Angle, radius: f32) -> Point {
    CENTER + Point::new(
        -(angle.to_radians().cos() * radius) as i32,
        -(angle.to_radians().sin() * radius) as i32,
    )
}

fn vario_angle(val: f32) -> Angle {
    (25.0 * val).deg()
}

fn draw_wallpaper<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Colors>,
{
    fn draw_vario_stroke<D>(
        display: &mut D, 
        angle: Angle, 
        style: MonoTextStyle<Colors>,
        txt: &str) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Colors>,
    {
        let pos_text = vario_coord(angle, (RADIUS - STROKE_TEXT_POS) as f32) + 
            Point::new(STROKE_FONT_OFF_X, STROKE_FONT_OFF_Y);

        Text::new(txt, pos_text, style)
            .draw(display)?;
    
        let start = vario_coord(angle, (RADIUS) as f32);
        let end = vario_coord(angle, (RADIUS - STROKE_LEN) as f32);
        let style = PrimitiveStyle::with_stroke(STROKE_COLOR, STROKE_WIDTH);
        Line::new(start, end)
            .into_styled(style)
            .draw(display)
    }
    
    display.clear(BACKGROUND)?;

    Circle::new(Point::new(0, 0), DIAMETER)
        .into_styled(PrimitiveStyle::with_stroke(STROKE_COLOR, 1))
        .draw(display)?;

    let text_style = MonoTextStyleBuilder::new()
        .font(&STROKE_FONT)
        .text_color(STROKE_COLOR)
        .build();

    for (value, txt) in WALLPAPER_SCALE {
        draw_vario_stroke(display, vario_angle(value), text_style, &txt)?;
    }
    Ok(())
}

fn draw_vario_indicator<D>(display: &mut D, value: f32, color: Colors) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Colors>,
{
    let angle = vario_angle(value);
    let start = vario_coord(angle, (RADIUS) as f32);
    let end = vario_coord(angle, (RADIUS - 4) as f32);
    let mut style = PrimitiveStyle::with_stroke(color, INDICATOR_WIDTH - 4);
    Line::new(start, end)
        .into_styled(style)
        .draw(display)?;
    let start = vario_coord(angle, (RADIUS - 2) as f32);
    let end = vario_coord(angle, (RADIUS - 8) as f32);
    style.stroke_width = INDICATOR_WIDTH - 2;
    Line::new(start, end)
        .into_styled(style)
        .draw(display)?;
    let start = vario_coord(angle, (RADIUS - 6) as f32);
    let end = vario_coord(angle, (RADIUS - INDICATOR_LEN) as f32);
    style.stroke_width = INDICATOR_WIDTH;
    Line::new(start, end)
        .into_styled(style)
        .draw(display)
}

fn draw_wind<D>(display: &mut D, wind_speed: f32, angle: Angle, color: Colors) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Colors>,
{
    let start = wind_coord(angle, 15.0 + ((WIND_LEN-15) as f32)*wind_speed/30.0);
    let end = wind_coord(angle, 15 as f32);
    let style = PrimitiveStyle::with_stroke(color, 5);
    Line::new(start, end)
        .into_styled(style)
        .draw(display)?;

    let head_1 = wind_coord((angle.to_radians() + 0.5).rad(), 20 as f32);    
    let head_2 = wind_coord((angle.to_radians() - 0.5).rad(), 20 as f32);
    let style = PrimitiveStyle::with_fill(color);
    Triangle::new(head_1, head_2, CENTER)
        .into_styled(style)
        .draw(display)?;

    Ok(())
}

fn draw_wind_text<D>(display: &mut D, text: &str, upper: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Colors>,
{
    let style = MonoTextStyleBuilder::new()
    .font(&WIND_FONT)
    .text_color(STROKE_COLOR)
    .build();
    let wind_y = if upper {
        CENTER.y + WIND_DELTA_Y + WIND_FONT.character_size.height as i32
    } else {
        CENTER.y - WIND_DELTA_Y
    };
    Text::new(text, Point::new(WIND_X, wind_y), style).draw(display)?;
    Ok(())
}

fn draw_speed_to_fly<D>(display: &mut D, value: f32, color: Colors) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Colors>,
{
    let head = -value as i32 * 2 + SPEED_TO_FLY_Y;
    Triangle::new(
        Point::new(SPEED_TO_FLY_X, SPEED_TO_FLY_Y), 
        Point::new(SPEED_TO_FLY_X + SPEED_TO_FLY_WIDTH, SPEED_TO_FLY_Y), 
        Point::new(SPEED_TO_FLY_X + SPEED_TO_FLY_WIDTH / 2, head))
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(display)?;
    Ok(())
}

fn draw_mc_ready<D>(display: &mut D, value: f32, color: Colors) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Colors>,
{
    let p1 = vario_coord(vario_angle(value + 0.1), RADIUS as f32);
    let p2 = vario_coord(vario_angle(value - 0.1), RADIUS as f32);
    let p3 = vario_coord(vario_angle(value), (RADIUS - 10)  as f32);
    Triangle::new(p1, p2, p3)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(display)?;
    Ok(())
}


pub struct VarioDisplay {
}

impl VarioDisplay {
    pub fn new() -> Self { Self{} }

    pub fn draw<D>(&mut self, display: &mut D, data: &Blackboard) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color=Colors>
    {
        let before_all = msec_tick();
        draw_wallpaper(display)?;
        trace!("Wallpaper {=u32} ms", msec_tick() - before_all);


        let before = msec_tick();
        draw_speed_to_fly(display, data.speed_to_fly_dif, SPEED_TO_FLY_COLOR)?;
        draw_mc_ready(display, data.mc_cready, MC_READY_COLOR)?;
        trace!("Speed, McCready {=u32} ms", msec_tick() - before);

        let before = msec_tick();
        draw_vario_indicator(display, data.average_climb_rate, AVERAGE_CLIMB_COLOR)?;
        draw_vario_indicator(display, data.climb_rate, VARIO_COLOR)?;
        trace!("Vario Indicators {=u32} ms", msec_tick() - before);

        // draw wind indicators and text
        let before = msec_tick();
        draw_wind(display, data.average_wind_speed, data.average_wind_angle, AVERAGE_WIND_COLOR)?;
        draw_wind(display, data.wind_speed, data.wind_angle, WIND_COLOR)?;
        trace!("Wind {=u32} ms", msec_tick() - before);

        let before = msec_tick();
        let text = "60° 25 km/h";
        let upper = data.average_wind_angle < 90.0.deg() || data.average_wind_angle > 270.0.deg();
        let r=draw_wind_text(display, &text, upper);
        trace!("Wind Text {=u32} ms", msec_tick() - before);

        trace!("All {=u32} ms", msec_tick() - before_all);
        r
    }
}
