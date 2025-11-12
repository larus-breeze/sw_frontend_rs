use super::{sprites::*, thermal_data::ThermalData};
use crate::{
    CoreError, DrawImage, model::{CoreModel, DataSource, FlyMode, OverlayActive, SystemState, VarioMode}, utils::Colors
};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use num::clamp;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub fn draw_info1<D>(display: &mut D, cm: &CoreModel, cm_1s: &CoreModel, vario_mode: bool) -> Result<(), CoreError>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    // draw info1 field or firmware version
    let sizes = &cm.device_const.sizes.vario;
    if cm.control.alive_ticks > 70 {
        if vario_mode {
            cm.config
                .info1_vario
                .draw(display, cm_1s, sizes.info1_pos)?;
        } else {
            cm.config
                .info1_stf
                .draw(display, cm_1s, sizes.info1_pos)?;
        }
    } else {
        // draw software version during the first N seconds
        let s = cm.device_const.misc.sw_version.as_string();
        cm.device_const.big_font.render_aligned(
            s.as_str(),
            sizes.info1_pos,
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(cm.palette().vario.scale),
            display,
        )?;
    }
    Ok(())
}

#[derive(PartialEq)]
pub struct Vario {
    thermal_data: ThermalData,
}

impl Vario {
    pub fn new() -> Vario {
        Vario {
            thermal_data: ThermalData::default(),
        }
    }

    pub fn draw<D>(&mut self, display: &mut D, cm: &CoreModel, cm_1s: &CoreModel) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let sizes = &cm.device_const.sizes.vario;
        let d_sizes = &cm.device_const.sizes.display;

        // draaw wallpaper
        display.clear(cm.palette().vario.background)?;
        display.draw_img(
            cm.device_const.images.wp_vario,
            Point::new(0, 0),
            Some(cm.palette().vario.scale),
        )?;
        display.draw_img(
            cm.device_const.images.m_s,
            sizes.unit_pos,
            Some(cm.palette().vario.background),
        )?;

        // draw battery symbol
        if cm_1s.device.supply_voltage > cm.config.battery_good {
            display.draw_img(
                cm.device_const.images.bat_full,
                sizes.bat_pos,
                Some(cm.palette().signal.go),
            )?;
        } else if cm_1s.device.supply_voltage < cm.config.battery_low {
            display.draw_img(
                cm.device_const.images.bat_empty,
                sizes.bat_pos,
                Some(cm.palette().signal.stop),
            )?;
        } else {
            display.draw_img(
                cm.device_const.images.bat_half,
                sizes.bat_pos,
                Some(cm.palette().signal.warning),
            )?;
        }

        // draw sat symbol
        let color = match cm.control.system_state {
            SystemState::NoCom => cm.palette().signal.stop,
            SystemState::CanOk => cm.palette().signal.warning,
            SystemState::CanAndGpsOk => cm.palette().signal.go,
        };
        display.draw_img(cm.device_const.images.sat, sizes.sat_pos, Some(color))?;

        // draw attention if necessary
        if !cm.sensor.gnss_and_compass_ok {
            display.draw_img(cm.device_const.images.attention, sizes.attention_pos, None)?;
        }

        // to save computing power: Only draw the central elements when they are visible.
        if cm.config.overlay_active == OverlayActive::None {
            // draw center view
            self.thermal_data.update(cm);
            match cm.control.fly_mode {
                FlyMode::Circling => {
                    cm.config
                        .center_circling
                        .draw(display, cm, &mut self.thermal_data)
                }
                FlyMode::StraightFlight => {
                    cm.config
                        .center_straight
                        .draw(display, cm, &mut self.thermal_data)
                }
            }?;

            // draw info fields
            match cm.control.vario_mode {
                VarioMode::Vario => {
                    // draw info1 field
                    draw_info1(display, cm, cm_1s, true)?;

                    // draw info2 field
                    cm.config
                        .info2_vario
                        .draw(display, cm_1s, sizes.info2_pos)?;

                    // draw info3 field
                    cm.config
                        .info3_vario
                        .draw(display, cm_1s)?;

                }
                VarioMode::SpeedToFly => {
                    // draw info1 field
                    draw_info1(display, cm, cm_1s, false)?;

                    // draw info2 field
                    cm.config
                        .info2_stf
                        .draw(display, cm_1s, sizes.info2_pos)?;

                    // draw info3 field
                    cm.config
                        .info3_stf
                        .draw(display, cm_1s)?;

                    // draw scal arc
                    let stf = num::clamp(-cm.calculated.speed_to_fly_dif.to_km_h() / 10.0, -5.0, 5.0);
                    let angle_sweep = (sizes.angle_m_s * stf).deg();
                    let col = cm.palette().vario.stf_arc;
                    Arc::with_center(d_sizes.center, sizes.stf_diameter, 180.0.deg(), angle_sweep)
                        .into_styled(PrimitiveStyle::with_stroke(col, sizes.stf_width))
                        .draw(display)?;
                }
            }
        }

        // draw mc_cready indicator
        ScaleMarker::new(d_sizes.radius as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate((cm.config.mc_cready.to_m_s() * sizes.angle_m_s).to_radians())
            .draw_colored(cm.palette().vario.mc_cready, display)?;

        // draw average climb rate marker
        let avg_climb_rate = match cm.control.avg_climb_rate_src {
            DataSource::Frontend => cm.calculated.av2_climb_rate.to_m_s(),
            DataSource::Sensorbox => cm.sensor.average_climb_rate.to_m_s(),
        };
        let av_climb_rate = clamp(avg_climb_rate, -5.0, 5.0);
        SimpleIndicator::at_base(
            (d_sizes.radius - sizes.indicator_len) as i32,
            d_sizes.center,
        )
        .zero_pos(pos::NINE_O_CLOCK)
        .rotate((av_climb_rate * sizes.angle_m_s).to_radians())
        .draw_colored(cm.palette().vario.avg_climb, display)?;

        // draw climb rate indicator
        let climb_rate = num::clamp(cm.calculated.interpolated_climb_rate.value.to_m_s(), -5.1, 5.1);
        ClassicIndicator::new(d_sizes.radius as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate((climb_rate * sizes.angle_m_s).to_radians())
            .draw_colored(cm.palette().vario.needle, display)?;
        Ok(())
    }
}
