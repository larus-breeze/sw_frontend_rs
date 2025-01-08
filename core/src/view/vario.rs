use super::{sprites::*, thermal_data::ThermalData};
use crate::{
    model::{CoreModel, FlyMode, SystemState, VarioMode},
    tformat,
    utils::Colors,
    CoreError, DrawImage,
};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use num::clamp;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub fn draw_thermal_climb<D>(display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    let sizes = &cm.device_const.sizes.vario;

    display.draw_img(
        &cm.device_const.images.spiral,
        sizes.pic_info3_pos,
        Some(cm.palette().vario_pic_info1),
    )?;
    display.draw_img(
        &cm.device_const.images.m_s,
        sizes.info3_pos,
        Some(cm.palette().scale),
    )?;
    let acr = num::clamp(cm.calculated.thermal_climb_rate.to_m_s(), -9.9, 99.9);
    let txt = tformat!(10, "{:.1}", acr).unwrap();
    cm.device_const.big_font.render_aligned(
        txt.as_str(),
        sizes.info3_pos,
        VerticalPosition::Top,
        HorizontalAlignment::Right,
        FontColor::Transparent(cm.palette().scale),
        display,
    )?;
    Ok(())
}

#[derive(PartialEq)]
pub struct Vario {
    thermal_data: ThermalData,
}

impl Vario {
    pub fn new() -> Vario {
        Vario { thermal_data: ThermalData::default(), }
    }

    pub fn draw<D>(&mut self, display: &mut D, cm: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let sizes = &cm.device_const.sizes.vario;
        let d_sizes = &cm.device_const.sizes.display;

        // draaw wallpaper
        display.clear(cm.palette().background)?;
        display.draw_img(
            &cm.device_const.images.wp_vario,
            Point::new(0, 0),
            Some(cm.palette().scale),
        )?;
        display.draw_img(
            &cm.device_const.images.m_s,
            sizes.unit_pos,
            Some(cm.palette().background),
        )?;

        // draw battery symbol
        if cm.device.supply_voltage > cm.device.voltage_limit_good {
            display.draw_img(
                &cm.device_const.images.bat_full,
                sizes.bat_pos,
                Some(cm.palette().signal_go),
            )?;
        } else if cm.device.supply_voltage < cm.device.voltage_limit_bad {
            display.draw_img(
                &cm.device_const.images.bat_empty,
                sizes.bat_pos,
                Some(cm.palette().signal_stop),
            )?;
        } else {
            display.draw_img(
                &cm.device_const.images.bat_half,
                sizes.bat_pos,
                Some(cm.palette().signal_warning),
            )?;
        }

        // draw sat symbol
        let color = match cm.control.system_state {
            SystemState::NoCom => cm.palette().signal_stop,
            SystemState::CanOk => cm.palette().signal_warning,
            SystemState::CanAndGpsOk => cm.palette().signal_go,
        };
        display.draw_img(&cm.device_const.images.sat, sizes.sat_pos, Some(color))?;

        // draw center view
        self.thermal_data.update(cm);
        match cm.control.fly_mode {
            FlyMode::Circling => cm.config.center_circling.draw(display, cm, &mut self.thermal_data),
            FlyMode::StraightFlight => cm.config.center_straignt.draw(display, cm, &mut self.thermal_data),
        }?;

        // draw info1 field or firmware version
        if cm.control.alive_ticks > 70 {
            cm.config
                .info1
                .draw(display, cm, sizes.info1_pos, cm.palette().scale)?;
        } else {
            // draw software version during the first N seconds
            let s = cm.device_const.misc.sw_version.as_string();
            cm.device_const.big_font.render_aligned(
                s.as_str(),
                sizes.info1_pos,
                VerticalPosition::Center,
                HorizontalAlignment::Center,
                FontColor::Transparent(cm.palette().scale),
                display,
            )?;
        }

        // draw info2 field
        cm.config
            .info2
            .draw(display, cm, sizes.info2_pos, cm.palette().scale)?;

        // draw info3 field
        match cm.control.vario_mode {
            VarioMode::Vario => {
                draw_thermal_climb(display, cm)?;
            }
            VarioMode::SpeedToFly => {
                let stf = num::clamp(-cm.calculated.speed_to_fly_dif.to_km_h() / 10.0, -5.0, 5.0);
                let angle_sweep = (sizes.angle_m_s * stf).deg();
                let col = cm.palette().vario_speed_to_fly;
                Arc::with_center(d_sizes.center, sizes.stf_diameter, 180.0.deg(), angle_sweep)
                    .into_styled(PrimitiveStyle::with_stroke(col, sizes.stf_width))
                    .draw(display)?;

                if cm.config.alt_stf_thermal_climb {
                    display.draw_img(
                        &cm.device_const.images.straight,
                        sizes.pic_info3_pos,
                        Some(cm.palette().scale),
                    )?;
                    display.draw_img(
                        &cm.device_const.images.km_h,
                        sizes.info3_pos,
                        Some(cm.palette().scale),
                    )?;
                    let stf = num::clamp(cm.calculated.speed_to_fly_1s.to_km_h(), 0.0, 999.0);
                    let txt = tformat!(10, "{:.0}", stf).unwrap();
                    cm.device_const.big_font.render_aligned(
                        txt.as_str(),
                        sizes.info3_pos,
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

        // draw mc_cready indicator
        ScaleMarker::new(d_sizes.radius as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate((cm.config.mc_cready.to_m_s() * sizes.angle_m_s).to_radians())
            .draw_colored(cm.palette().needle2, display)?;

    
        // draw average climb rate marker
        let av_climb_rate = clamp(cm.calculated.av2_climb_rate.to_m_s(), -5.0, 5.0);
        SimpleIndicator::at_base((d_sizes.radius - sizes.indicator_len) as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate((av_climb_rate * sizes.angle_m_s).to_radians())
            .draw_colored(cm.palette().needle3, display)?;

        // draw climb rate indicator
        let climb_rate = num::clamp(cm.sensor.climb_rate.to_m_s(), -5.1, 5.1);
        ClassicIndicator::new(d_sizes.radius as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate((climb_rate * sizes.angle_m_s).to_radians())
            .draw_colored(cm.palette().needle1, display)?;
        Ok(())
    }
}
