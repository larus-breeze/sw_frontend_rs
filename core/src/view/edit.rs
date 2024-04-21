use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    controller::Editable,
    model::{CoreModel, VarioModeControl},
    tformat,
    utils::{Colors, FONT_BIG, FONT_SMALL},
    view::SCREEN_CENTER,
    CoreError, DrawImage, POLARS,
};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use heapless::String;
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

    FONT_SMALL.render_aligned(
        name_str,
        SCREEN_CENTER + Point::new(0, -13),
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(DESCRIPTION_COLOR),
        display,
    )?;

    FONT_BIG.render_aligned(
        val_str.as_str(),
        SCREEN_CENTER + Point::new(0, 25),
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(VALUE_COLOR),
        display,
    )?;
    Ok(())
}

fn get_edit_strs(cm: &CoreModel) -> (&str, String<20>) {
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

    let val_str = match cm.control.edit_var {
        Editable::ClimbRate => tformat!(20, "{:.1} m/s", cm.sensor.climb_rate.to_m_s()),
        Editable::Glider => tformat!(20, "{}", POLARS[cm.config.glider_idx as usize].name),
        Editable::McCready => tformat!(20, "{:.1} m/s", cm.config.mc_cready.to_m_s()),
        Editable::PilotWeight => tformat!(20, "{:.0} kg", cm.glider_data.pilot_weight.to_kg()),
        Editable::VarioModeControl => match cm.control.vario_mode_control {
            VarioModeControl::Auto => tformat!(20, "Auto"),
            VarioModeControl::Vario => tformat!(20, "Vario"),
            VarioModeControl::SpeedToFly => tformat!(20, "SpeedToFly"),
        },
        Editable::Speed => tformat!(20, "{:.0} km/h", cm.sensor.airspeed.ias().to_km_h()),
        Editable::Volume => tformat!(20, "{}", cm.config.volume),
        Editable::WaterBallast => tformat!(20, "{:.0} kg", cm.glider_data.water_ballast.to_kg()),
        Editable::WindDirection => {
            tformat!(20, "{:.0} °", cm.sensor.wind_vector.angle().to_degrees())
        }
        Editable::WindSpeed => tformat!(20, "{:.0} km/h", cm.sensor.wind_vector.speed().to_km_h()),
    };
    (name_str, val_str.unwrap())
}
