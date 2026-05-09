use crate::{
    model::{CoreModel, GpsState, SystemState},
    tformat,
    utils::Colors,
    CoreError, DrawImage,
};

use embedded_graphics::{draw_target::DrawTarget, prelude::Point};
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

#[allow(unused)]
use micromath::F32Ext;

struct LineInfo {
    name: &'static str,
    value: heapless::String<30>,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DeviceLineView {
    // headings
    Empty = 0,
    SensorBox,
    VarioDisplay,

    // lines of information (Vario Display)
    DisplayVersion = 20,
    SupplyVoltage,
    IluminationVoltage,
    TempPcb,
    InPinBreaks,
    InPinDrain,
    InPinGear,
    InPinSpeedToFly,
    OutPinFlash,

    // lines of information (Sensor Box)
    SensorboxVersion = 50,
    AhrsPitch,
    AhrsRoll,
    AhrsYaw,
    GnssAccuracityOk,
    MagneticDisturbanceOk,
    Ias,
    Tas,
    Density,
    Gforce,
    GpsAltitude,
    GpsGroundSpeed,
    GpsTrack,
    GpsHeading,
    GpsSats,
    GpsState,
    GpsAccLen,
    GpsAccHeading,
    GnssBaseline,
    GnssBaseLen,
    GnssDown,
    AntSlaveRight,
    GnssRelNE,
    NickAngle,
    Pressure,
    SlipAngle,
    TurnRate,
    HorizonAvailable,
}

impl DeviceLineView {
    pub fn draw<D>(&self, display: &mut D, cm: &CoreModel, point: Point) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
    {
        if let Some(header_text) = self.header_text() {
            cm.device_const.small_font.render_aligned(
                header_text,
                point,
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(cm.palette().list_edit.header),
                display,
            )?;
        } else {
            let line_info = self.line_info(cm);
            let p1 = Point::new(point.x - 3, point.y);
            cm.device_const.small_font.render_aligned(
                line_info.name,
                p1,
                VerticalPosition::Top,
                HorizontalAlignment::Right,
                FontColor::Transparent(cm.palette().list_edit.item),
                display,
            )?;
            let p2 = Point::new(point.x + 10, point.y);
            cm.device_const.small_font.render_aligned(
                line_info.value.as_str(),
                p2,
                VerticalPosition::Top,
                HorizontalAlignment::Left,
                FontColor::Transparent(cm.palette().list_edit.item),
                display,
            )?;
        }
        Ok(())
    }

    fn header_text(&self) -> Option<&'static str> {
        match self {
            DeviceLineView::Empty => Some(""),
            DeviceLineView::SensorBox => Some("Sensor Box"),
            DeviceLineView::VarioDisplay => Some("Vario Display"),
            DeviceLineView::GnssBaseline => Some("GNSS Baseline"),
            _ => None,
        }
    }

    fn ok(ok: bool) -> &'static str {
        if ok {
            "Ok"
        } else {
            "Not ok"
        }
    }

    fn dgnss_available(cm: &CoreModel) -> bool {
        cm.sensor.gps_state == GpsState::HeadingAvail
    }

    fn length_comparison(
        measured: Option<crate::Length>,
        configured: Option<crate::Length>,
        available: bool,
    ) -> heapless::String<30> {
        if !available {
            return tformat!(30, "-").unwrap();
        }

        match (measured, configured) {
            (Some(measured), Some(configured)) => {
                let measured = measured.to_m();
                let configured = configured.to_m();
                let error = measured - configured;
                let error_sign = if error < 0.0 { "-" } else { "+" };
                let error_abs = if error < 0.0 { -error } else { error };
                tformat!(
                    30,
                    "{:.2}/{:.2} {}{:.2} m",
                    measured,
                    configured,
                    error_sign,
                    error_abs
                )
                .unwrap()
            }
            (Some(measured), None) => tformat!(30, "{:.2}/- m", measured.to_m()).unwrap(),
            (None, Some(configured)) => tformat!(30, "-/{:.2} m", configured.to_m()).unwrap(),
            (None, None) => tformat!(30, "-").unwrap(),
        }
    }

    fn length_value(value: Option<crate::Length>, available: bool) -> heapless::String<30> {
        if !available {
            return tformat!(30, "-").unwrap();
        }

        if let Some(value) = value {
            tformat!(30, "{:.2} m", value.to_m()).unwrap()
        } else {
            tformat!(30, "-").unwrap()
        }
    }

    fn rel_ne_value(cm: &CoreModel) -> heapless::String<30> {
        if !Self::dgnss_available(cm) {
            return tformat!(30, "-").unwrap();
        }

        match (cm.sensor.dgps_rel_pos_n, cm.sensor.dgps_rel_pos_e) {
            (Some(north), Some(east)) => {
                let north = north.to_m();
                let east = east.to_m();
                let north_sign = if north < 0.0 { "-" } else { "+" };
                let east_sign = if east < 0.0 { "-" } else { "+" };
                let north_abs = if north < 0.0 { -north } else { north };
                let east_abs = if east < 0.0 { -east } else { east };
                tformat!(
                    30,
                    "{}{:.2}/{}{:.2} m",
                    north_sign,
                    north_abs,
                    east_sign,
                    east_abs
                )
                .unwrap()
            }
            _ => tformat!(30, "-").unwrap(),
        }
    }

    fn line_info(&self, cm: &CoreModel) -> LineInfo {
        let dgnss_available = Self::dgnss_available(cm);
        let mut lv = match self {
            // Vario Display values
            DeviceLineView::DisplayVersion => LineInfo {
                name: "FW Version: ",
                value: tformat!(
                    30,
                    "{}",
                    cm.device_const.misc.sw_version.as_string().as_str()
                )
                .unwrap(),
            },
            DeviceLineView::SupplyVoltage => LineInfo {
                name: "Sup Volt: ",
                value: tformat!(30, "{:.1} V", cm.device.supply_voltage).unwrap(),
            },
            DeviceLineView::IluminationVoltage => LineInfo {
                name: "Ilumn Volt: ",
                value: tformat!(30, "{:.1} V", cm.device.illumination_voltage).unwrap(),
            },
            DeviceLineView::TempPcb => LineInfo {
                name: "Temp Pcb: ",
                value: tformat!(30, "{:.1} °C", cm.device.temperature_pcb).unwrap(),
            },
            DeviceLineView::InPinBreaks => LineInfo {
                name: "Pin Breaks: ",
                value: tformat!(30, "in {}", cm.control.hw_pins.in_breakes.as_str()).unwrap(),
            },
            DeviceLineView::InPinDrain => LineInfo {
                name: "Pin Drain: ",
                value: tformat!(30, "in {}", cm.control.hw_pins.in_drain.as_str()).unwrap(),
            },
            DeviceLineView::InPinGear => LineInfo {
                name: "Pin Gear: ",
                value: tformat!(30, "in {}", cm.control.hw_pins.in_gear.as_str()).unwrap(),
            },
            DeviceLineView::InPinSpeedToFly => LineInfo {
                name: "Pin Stf: ",
                value: tformat!(30, "in {}", cm.control.hw_pins.in_speed_to_fly.as_str()).unwrap(),
            },
            DeviceLineView::OutPinFlash => LineInfo {
                name: "Pin Flash: ",
                value: tformat!(30, "out {}", cm.control.hw_pins.out_flash.as_str()).unwrap(),
            },

            // Sensorbox values
            DeviceLineView::SensorboxVersion => LineInfo {
                name: "FW Version: ",
                value: tformat!(30, "{}", cm.sensor.sw_version.as_string().as_str()).unwrap(),
            },
            DeviceLineView::AhrsPitch => LineInfo {
                name: "AHRS Pitch: ",
                value: tformat!(30, "{:.0}°", cm.sensor.euler_pitch.to_degrees()).unwrap(),
            },
            DeviceLineView::AhrsRoll => LineInfo {
                name: "AHRS Roll: ",
                value: tformat!(30, "{:.0}°", cm.sensor.euler_roll.to_degrees()).unwrap(),
            },
            DeviceLineView::AhrsYaw => LineInfo {
                name: "AHRS Yaw: ",
                value: tformat!(30, "{:.0}°", cm.sensor.euler_yaw.to_degrees()).unwrap(),
            },
            DeviceLineView::GnssAccuracityOk => LineInfo {
                name: "GNSS Data: ",
                value: tformat!(30, "{}", Self::ok(!cm.sensor.gnss_velocity_accuracy_bad()))
                    .unwrap(),
            },
            DeviceLineView::MagneticDisturbanceOk => LineInfo {
                name: "Magn Data: ",
                value: tformat!(30, "{}", Self::ok(!cm.sensor.magnetic_disturbance_bad())).unwrap(),
            },
            DeviceLineView::Ias => LineInfo {
                name: "IAS: ",
                value: tformat!(30, "{:.0} km/h", cm.sensor.airspeed.ias().to_km_h()).unwrap(),
            },
            DeviceLineView::Tas => LineInfo {
                name: "TAS: ",
                value: tformat!(30, "{:.0} km/h", cm.sensor.airspeed.tas().to_km_h()).unwrap(),
            },
            DeviceLineView::Density => LineInfo {
                name: "Density: ",
                value: tformat!(30, "{:.3} kg/m³", cm.sensor.density.to_kg_m3()).unwrap(),
            },
            DeviceLineView::Gforce => LineInfo {
                name: "G-Force: ",
                value: tformat!(30, "{:.1} m/s²", cm.sensor.g_force.to_m_s2()).unwrap(),
            },
            DeviceLineView::GpsAltitude => LineInfo {
                name: "GNSS Alt: ",
                value: tformat!(30, "{:.0} m", cm.sensor.gps_altitude.to_m()).unwrap(),
            },
            DeviceLineView::GpsGroundSpeed => LineInfo {
                name: "GNSS GS: ",
                value: tformat!(30, "{:.0} km/h", cm.sensor.gps_ground_speed.to_km_h()).unwrap(),
            },
            DeviceLineView::GpsTrack => LineInfo {
                name: "GNSS Track: ",
                value: tformat!(30, "{:.0}°", cm.sensor.gps_track.to_degrees()).unwrap(),
            },
            DeviceLineView::GpsHeading => LineInfo {
                name: "GNSS Heading: ",
                value: if dgnss_available {
                    tformat!(30, "{:.0}°", cm.sensor.gps_heading.to_degrees()).unwrap()
                } else {
                    tformat!(30, "-").unwrap()
                },
            },
            DeviceLineView::GpsSats => LineInfo {
                name: "GNSS Sats: ",
                value: tformat!(30, "{}", cm.sensor.gps_sats).unwrap(),
            },
            DeviceLineView::GpsState => LineInfo {
                name: "GNSS State: ",
                value: tformat!(30, "{}", cm.sensor.gps_state.as_str()).unwrap(),
            },
            DeviceLineView::GpsAccLen => LineInfo {
                name: "GNSS Acc: ",
                value: if dgnss_available {
                    tformat!(30, "{:.0} cm", cm.sensor.dgps_acc_len.to_m() * 100.0).unwrap()
                } else {
                    tformat!(30, "-").unwrap()
                },
            },
            DeviceLineView::GpsAccHeading => LineInfo {
                name: "GNSS HdgAcc: ",
                value: if dgnss_available {
                    tformat!(30, "{:.1} °", cm.sensor.dgps_acc_heading.to_degrees()).unwrap()
                } else {
                    tformat!(30, "-").unwrap()
                },
            },
            DeviceLineView::GnssBaseLen => LineInfo {
                name: "Base Len: ",
                value: Self::length_comparison(
                    cm.sensor.dgps_rel_pos_length,
                    cm.sensor.antenna_baselen,
                    dgnss_available,
                ),
            },
            DeviceLineView::GnssDown => LineInfo {
                name: "Down: ",
                value: Self::length_comparison(
                    cm.sensor.dgps_rel_pos_d,
                    cm.sensor.antenna_slave_down,
                    dgnss_available,
                ),
            },
            DeviceLineView::AntSlaveRight => LineInfo {
                name: "Right cfg: ",
                value: Self::length_value(cm.sensor.antenna_slave_right, dgnss_available),
            },
            DeviceLineView::GnssRelNE => LineInfo {
                name: "Rel N/E: ",
                value: Self::rel_ne_value(cm),
            },
            DeviceLineView::NickAngle => LineInfo {
                name: "Nick Angle: ",
                value: tformat!(30, "{:.0}°", cm.sensor.nick_angle.to_degrees()).unwrap(),
            },
            DeviceLineView::Pressure => LineInfo {
                name: "Pressure: ",
                value: tformat!(30, "{:.1} hPa", cm.sensor.pressure.to_hpa()).unwrap(),
            },
            DeviceLineView::SlipAngle => LineInfo {
                name: "Slip Angle: ",
                value: tformat!(30, "{:.1} °", cm.sensor.slip_angle.to_degrees()).unwrap(),
            },
            DeviceLineView::TurnRate => LineInfo {
                name: "Turn Rate: ",
                value: tformat!(30, "{:.0} °/s", cm.sensor.turn_rate.to_deg_s()).unwrap(),
            },
            DeviceLineView::HorizonAvailable => LineInfo {
                name: "Hor Avail: ",
                value: tformat!(30, "{}", !cm.sensor.horizon_blocked()).unwrap(),
            },

            // These are empty or header lines, so this never can be addressed
            DeviceLineView::Empty
            | DeviceLineView::SensorBox
            | DeviceLineView::VarioDisplay
            | DeviceLineView::GnssBaseline => LineInfo {
                name: "Error",
                value: tformat!(30, "Error").unwrap(),
            },
        };
        if cm.control.system_state == SystemState::NoCom && (*self as u8) >= 50 {
            lv.value = tformat!(30, "-").unwrap();
        }
        lv
    }
}
