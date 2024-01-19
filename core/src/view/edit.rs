use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    controller::Editable,
    model::{CoreModel, VarioModeControl},
    utils::{Colors, FONT_HELV_14, FONT_HELV_18},
    view::SCREEN_CENTER,
    Concat, CoreError, DrawImage, POLARS,
};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

const VALUE_COLOR: Colors = Colors::White;
const DESCRIPTION_COLOR: Colors = Colors::Yellow;
const BACKGROUND_COLOR: Colors = Colors::MidnightBlue;
const BORDER_COLOR: Colors = Colors::LightSteelBlue;

const WIDTH: u32 = DISPLAY_WIDTH * 90 / 100;
const HEIGHT: u32 = DISPLAY_HEIGHT * 32 / 100;

pub fn draw<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BORDER_COLOR)
        .stroke_width(2)
        .fill_color(BACKGROUND_COLOR)
        .build();

    Rectangle::with_center(SCREEN_CENTER, Size::new(WIDTH, HEIGHT))
        .into_styled(style)
        .draw(display)?;

    let (name_str, val_str) = get_edit_strs(cm);

    FONT_HELV_14.render_aligned(
        name_str,
        SCREEN_CENTER + Point::new(0, -13),
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(DESCRIPTION_COLOR),
        display,
    )?;

    FONT_HELV_18.render_aligned(
        val_str.as_str(),
        SCREEN_CENTER + Point::new(0, 25),
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(VALUE_COLOR),
        display,
    )?;
    Ok(())
}

fn get_edit_strs(cm: &CoreModel) -> (&str, Concat<20>) {
    let name_str = match cm.control.edit_var {
        Editable::ClimbRate => "Climb Rate",
        Editable::Glider => "Glider",
        Editable::McCready => "MC Cready",
        Editable::PilotWeight => "Pilot Weight",
        Editable::VarioModeControl => "Vario Control",
        Editable::Speed => "Airspeed (IAS)",
        Editable::Volume => "Volume",
        Editable::WaterBallast => "Water Ballast",
        Editable::WindDirection => "Wind Direction",
        Editable::WindSpeed => "Wind Speed",
    };

    let val_str = Concat::<20>::new();
    let val_str = match cm.control.edit_var {
        Editable::ClimbRate => val_str
            .push_f32(cm.sensor.climb_rate.to_m_s(), 1)
            .push_str(" m/s"),
        Editable::Glider => val_str.push_str(POLARS[cm.config.glider_idx as usize].name),
        Editable::McCready => val_str.push_f32(cm.config.mc_cready.to_m_s(), 1),
        Editable::PilotWeight => val_str.push_f32(cm.glider_data.pilot_weight.to_kg(), 0),
        Editable::VarioModeControl => match cm.control.vario_mode_control {
            VarioModeControl::Auto => val_str.push_str("Auto"),
            VarioModeControl::Vario => val_str.push_str("Vario"),
            VarioModeControl::SpeedToFly => val_str.push_str("SpeedToFly"),
        },
        Editable::Speed => val_str
            .push_f32(cm.sensor.airspeed.ias().to_km_h(), 0)
            .push_str(" km/h"),
        Editable::Volume => val_str.push_i8(cm.config.volume),
        Editable::WaterBallast => val_str
            .push_f32(cm.glider_data.water_ballast.to_kg(), 0)
            .push_str(" kg"),
        Editable::WindDirection => val_str
            .push_f32(cm.sensor.wind_vector.angle().to_degrees(), 0)
            .push_str(" °"),
        Editable::WindSpeed => val_str
            .push_f32(cm.sensor.wind_vector.speed().to_km_h(), 0)
            .push_str(" km/h"),
    };
    (name_str, val_str)
}
