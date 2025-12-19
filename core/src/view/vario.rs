use super::{sprites::*, thermal_data::ThermalData};
use crate::{
    model::{
        CoreModel, DataSource, FlyMode, OverlayActive, SystemState, UnitVerticalSpeed, VarioMode,
    },
    utils::Colors,
    CoreError, DrawImage, Image,
};

use embedded_graphics::{
    geometry::AngleUnit,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
};
use num::clamp;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

pub fn draw_info1<D>(
    display: &mut D,
    cm: &CoreModel,
    cm_1s: &CoreModel,
    vario_mode: bool,
) -> Result<(), CoreError>
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
            cm.config.info1_stf.draw(display, cm_1s, sizes.info1_pos)?;
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

    pub fn draw<D>(
        &mut self,
        display: &mut D,
        cm: &CoreModel,
        cm_1s: &CoreModel,
    ) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        let sizes = &cm.device_const.sizes.vario;
        let d_sizes = &cm.device_const.sizes.display;

        let (img_background, img_unit) = match cm.config.unit_vertical_speed {
            UnitVerticalSpeed::Fpm => (
                Image::new(cm.device_const.images.wp_vario_10),
                Image::new(cm.device_const.images.fpm_100),
            ),
            UnitVerticalSpeed::Knots => (
                Image::new(cm.device_const.images.wp_vario_10),
                Image::new(cm.device_const.images.kt),
            ),
            UnitVerticalSpeed::Mps => (
                Image::new(cm.device_const.images.wp_vario_5),
                Image::new(cm.device_const.images.m_s),
            ),
        };

        // clear display
        display.clear(cm.palette().vario.background)?;

        // draaw wallpaper
        img_background.draw(display, Point::new(0, 0), Some(cm.palette().vario.scale))?;

        // draw scale unit
        img_unit.draw(display, sizes.unit_pos, Some(cm.palette().vario.background))?;

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

        // draw symbol for user profile
        let img = if cm.config.club_mode {
            match cm.config.user_profile {
                1 => cm.device_const.images.club_1,
                2 => cm.device_const.images.club_2,
                3 => cm.device_const.images.club_3,
                _ => cm.device_const.images.club_0,
            }
        } else {
            match cm.config.user_profile {
                1 => cm.device_const.images.normal_1,
                2 => cm.device_const.images.normal_2,
                3 => cm.device_const.images.normal_3,
                _ => cm.device_const.images.normal_0,
            }
        };
        display.draw_img(img, sizes.profile_pos, Some(cm.palette().vario.unit))?;

        // to save computing power: Only draw the central elements when they are visible.
        if cm.config.overlay_active == OverlayActive::None {
            if cm.sensor.gnss_velocity_accuracy_bad() || cm.sensor.magnetic_disturbance_bad() {
                display.draw_img(cm.device_const.images.attention, sizes.attention_pos, None)?;
            }

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
                    cm.config.info3_vario.draw(display, cm_1s)?;
                }
                VarioMode::SpeedToFly => {
                    // draw info1 field
                    draw_info1(display, cm, cm_1s, false)?;

                    // draw info2 field
                    cm.config.info2_stf.draw(display, cm_1s, sizes.info2_pos)?;

                    // draw info3 field
                    cm.config.info3_stf.draw(display, cm_1s)?;

                    // draw scal arc
                    let stf =
                        num::clamp(-cm.calculated.speed_to_fly_dif.to_km_h() / 10.0, -5.0, 5.0);
                    let angle_sweep = (sizes.scale_factor * stf).deg();
                    let col = cm.palette().vario.stf_arc;
                    Arc::with_center(d_sizes.center, sizes.stf_diameter, 180.0.deg(), angle_sweep)
                        .into_styled(PrimitiveStyle::with_stroke(col, sizes.stf_width))
                        .draw(display)?;
                }
            }
        }

        // Calculate the angles for the analog pointers.
        let avg_climb_rate = match cm.control.avg_climb_rate_src {
            DataSource::Frontend => cm.calculated.av2_climb_rate,
            DataSource::Sensorbox => cm.sensor.average_climb_rate,
        };

        let (mc_cready_angle, avg_climb_angle, climb_rate_angle) =
            match cm.config.unit_vertical_speed {
                UnitVerticalSpeed::Fpm => {
                    let av_climb_rate = clamp(avg_climb_rate.to_ft_min(), -1000.0, 1000.0);
                    let climb_rate = num::clamp(
                        cm.calculated.interpolated_climb_rate.value.to_ft_min(),
                        -1020.0,
                        1020.0,
                    );
                    let scale_factor = sizes.scale_factor / 200.0;
                    (
                        (cm.config.mc_cready.to_ft_min() * scale_factor).to_radians(),
                        (av_climb_rate * scale_factor).to_radians(),
                        (climb_rate * scale_factor).to_radians(),
                    )
                }
                UnitVerticalSpeed::Knots => {
                    let av_climb_rate = clamp(avg_climb_rate.to_kt(), -10.0, 10.0);
                    let climb_rate = num::clamp(
                        cm.calculated.interpolated_climb_rate.value.to_kt(),
                        -10.2,
                        10.2,
                    );
                    let scale_factor = sizes.scale_factor / 2.0;
                    (
                        (cm.config.mc_cready.to_kt() * scale_factor).to_radians(),
                        (av_climb_rate * scale_factor).to_radians(),
                        (climb_rate * scale_factor).to_radians(),
                    )
                }
                UnitVerticalSpeed::Mps => {
                    let av_climb_rate = clamp(avg_climb_rate.to_m_s(), -5.0, 5.0);
                    let climb_rate = num::clamp(
                        cm.calculated.interpolated_climb_rate.value.to_m_s(),
                        -5.1,
                        5.1,
                    );
                    (
                        (cm.config.mc_cready.to_m_s() * sizes.scale_factor).to_radians(),
                        (av_climb_rate * sizes.scale_factor).to_radians(),
                        (climb_rate * sizes.scale_factor).to_radians(),
                    )
                }
            };

        // draw mc_cready indicator
        ScaleMarker::new(d_sizes.radius as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate(mc_cready_angle)
            .draw_colored(cm.palette().vario.mc_cready, display)?;

        // draw average climb rate marker
        SimpleIndicator::at_base(
            (d_sizes.radius - sizes.indicator_len) as i32,
            d_sizes.center,
        )
        .zero_pos(pos::NINE_O_CLOCK)
        .rotate(avg_climb_angle)
        .draw_colored(cm.palette().vario.avg_climb, display)?;

        // draw climb rate indicator
        ClassicIndicator::new(d_sizes.radius as i32, d_sizes.center)
            .zero_pos(pos::NINE_O_CLOCK)
            .rotate(climb_rate_angle)
            .draw_colored(cm.palette().vario.needle, display)?;
        Ok(())
    }
}
