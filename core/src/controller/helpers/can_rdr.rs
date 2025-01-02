use crate::{
    controller::{
        helpers::{object_id, CanActive, can_ids::{gps, sensor, sensor_legacy}},
        Echo,
    },
    model::{GpsState, VarioModeControl},
    AirSpeed, Angle, CanFrame, CoreController, CoreModel, F64ToCoord,
    FloatToAcceleration, FloatToAngularVelocity, FloatToDensity, FloatToLength, FloatToMass,
    FloatToPressure, FloatToSpeed, FlyMode, Frame, GenericFrame, GenericId, Latitude, Longitude,
    PersistenceId, SpecificFrame, Variant,
};
use embedded_graphics::prelude::AngleUnit;

use super::CanConfigId;

impl CoreController {
    pub fn read_can_frame(&mut self, cm: &mut CoreModel, frame: &Frame) {
        match frame {
            Frame::Generic(generic_frame) => self.can_frame_read_generic(cm, generic_frame),
            Frame::Specific(specific_frame) => self.can_frame_read_specific(cm, specific_frame),
            Frame::Legacy(can_frame) => self.can_frame_read_legacy(cm, can_frame),
        }
    }

    fn can_frame_read_generic(&mut self, cm: &mut CoreModel, frame: &GenericFrame) {
        let mut rdr = frame.can_frame.reader();
        #[allow(clippy::single_match)]
        match GenericId::from(frame.generic_id) {
            GenericId::SetSysSetting => {
                let config_id = CanConfigId::from(rdr.pop_u16());
                self.can_frame_read_sys_config_value(cm, config_id, &frame.can_frame)
            }
            _ => (),
        }
    }

    fn can_frame_read_specific(&mut self, cm: &mut CoreModel, frame: &SpecificFrame) {
        #[allow(clippy::single_match)]
        match frame.object_id {
            object_id::SENSOR => self.can_frame_read_sensor_values(cm, frame),
            object_id::GPS => self.can_frame_read_gps_values(cm, frame),
            _ => (),
        }
    }

    fn can_frame_read_sys_config_value(
        &mut self,
        cm: &mut CoreModel,
        config_id: CanConfigId,
        frame: &CanFrame,
    ) {
        match config_id {
            CanConfigId::Volume => {
                let val = frame.read_u8(2) as i8;
                self.persist_set(cm, Variant::I8(val), PersistenceId::Volume, Echo::Nmea);
            }
            CanConfigId::MacCready => {
                let val = frame.read_f32(4).m_s();
                self.persist_set(cm, Variant::Speed(val), PersistenceId::McCready, Echo::Nmea);
            }
            CanConfigId::WaterBallast => {
                let val = frame.read_f32(4).kg();
                self.persist_set(
                    cm,
                    Variant::Mass(val),
                    PersistenceId::WaterBallast,
                    Echo::Nmea,
                );
            }
            CanConfigId::Bugs => {
                let val = frame.read_f32(4);
                self.persist_set(cm, Variant::F32(val), PersistenceId::Bugs, Echo::Nmea);
            }
            CanConfigId::Qnh => {
                let val = frame.read_f32(4).hpa();
                self.persist_set(cm, Variant::Pressure(val), PersistenceId::Qnh, Echo::Nmea);
            }
            CanConfigId::PilotWeight => {
                let val = frame.read_f32(4).kg();
                self.persist_set(
                    cm,
                    Variant::Mass(val),
                    PersistenceId::PilotWeight,
                    Echo::Nmea,
                )
            }
            CanConfigId::VarioModeControl => {
                let val = VarioModeControl::from(frame.read_u8(2));
                self.persist_set(
                    cm,
                    Variant::VarioModeControl(val),
                    PersistenceId::VarioModeControl,
                    Echo::None,
                );
            }
            CanConfigId::TcClimbRate => {
                let val = frame.read_f32(4);
                self.persist_set(
                    cm,
                    Variant::F32(val),
                    PersistenceId::TcClimbRate,
                    Echo::None,
                )
            }
            CanConfigId::TcSpeedToFly => {
                let val = frame.read_f32(4);
                self.persist_set(
                    cm,
                    Variant::F32(val),
                    PersistenceId::TcSpeedToFly,
                    Echo::None,
                )
            }
            _ => (),
        }
    }

    fn can_frame_read_legacy(&mut self, cm: &mut CoreModel, frame: &CanFrame) {
        fn norm_0_2pi(r: i16) -> Angle {
            let mut r = r % 6284;
            if r < 0 {
                r += 6284;
            }
            ((r as f32) * 0.001).rad()
        }

        fn norm_mpi_ppi(r: i16) -> Angle {
            let mut r = r % 6284;
            if r > 3142 {
                r -= 6284
            }
            ((r as f32) * 0.001).rad()
        }

        let id = frame.id();
        let mut rdr = frame.reader();

        match id {
            sensor_legacy::EULER_ANGLES => {
                cm.sensor.euler_roll = norm_mpi_ppi(rdr.pop_i16());
                cm.sensor.euler_pitch = norm_mpi_ppi(rdr.pop_i16());
                cm.sensor.euler_yaw = norm_0_2pi(rdr.pop_i16());
            }
            sensor_legacy::ACCELERATION => {
                cm.sensor.g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
                cm.sensor.vertical_g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
                let _ = ((rdr.pop_i16() as f32) * 0.001).m_s(); // gps_climb_rate
                match rdr.pop_u8() {
                    0 => cm.control.fly_mode = FlyMode::StraightFlight,
                    2 => cm.control.fly_mode = FlyMode::Circling,
                    _ => (),
                }
            }
            sensor_legacy::AIRSPEED => {
                let tas = (rdr.pop_i16() as f32).km_h();
                let ias = (rdr.pop_i16() as f32).km_h();
                cm.sensor.airspeed = AirSpeed::from_speeds(ias, tas);
            }
            sensor_legacy::ATHMOSPHERE => {
                cm.sensor.pressure = (rdr.pop_u32() as f32).n_m2();
                cm.sensor.density = (rdr.pop_u32() as f32).g_m3();
                cm.sensor
                    .pressure_altitude
                    .set_static_pressure(cm.sensor.pressure);
            }
            sensor_legacy::GPS_DATE_TIME => {
                let year = 2000 + rdr.pop_u8() as u16;
                let month = rdr.pop_u8();
                let day = rdr.pop_u8();
                let hour = rdr.pop_u8();
                let min = rdr.pop_u8();
                let sec = rdr.pop_u8();
                cm.sensor
                    .gps_date_time
                    .set_date_time(year, month, day, hour, min, sec);
            }
            sensor_legacy::GPS_LAT_LON => {
                cm.sensor.gps_lat = Latitude(((rdr.pop_i32() as f64) * 1.0e-7).deg());
                cm.sensor.gps_lon = Longitude(((rdr.pop_i32() as f64) * 1.0e-7).deg());
            }
            sensor_legacy::GPS_ALT => {
                cm.sensor.gps_altitude = (rdr.pop_i32() as f32).mm();
                cm.sensor.gps_geo_seperation = (rdr.pop_i32() as f32 * 0.1).m();
            }
            sensor_legacy::GPS_TRK_SPD => {
                cm.sensor.gps_track = (rdr.pop_i16() as f32 * 0.001).rad();
                cm.sensor.gps_ground_speed = (rdr.pop_u16() as f32).km_h();
                if cm.sensor.gps_ground_speed < 1.0.km_h() {
                    cm.sensor.gps_track = 0.0_f32.rad();
                }
                if cm.sensor.gps_track < 0.0_f32.rad() {
                    cm.sensor.gps_track += 360.0_f32.deg();
                }
            }
            sensor_legacy::GPS_SATS => {
                cm.sensor.gps_sats = rdr.pop_u8();
                match rdr.pop_u8() {
                    1 => cm.sensor.gps_state = GpsState::PosAvail,
                    3 => cm.sensor.gps_state = GpsState::HeadingAvail,
                    _ => cm.sensor.gps_state = GpsState::NoGps,
                }
            }
            sensor_legacy::TURN_COORD => {
                cm.sensor.slip_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
                cm.sensor.turn_rate = ((rdr.pop_i16() as f32) * 0.001).rad_s();
                cm.sensor.nick_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
            }
            sensor_legacy::VARIO => {
                cm.sensor.climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
                cm.sensor.average_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
                cm.control.can_devices |= CanActive::SensorboxLegacy as u32;
            }
            sensor_legacy::WIND => {
                cm.sensor
                    .wind_vector
                    .set_angle(((rdr.pop_i16() as f32) * 0.001).rad());
                cm.sensor
                    .wind_vector
                    .set_speed((rdr.pop_i16() as f32).km_h());
                cm.sensor
                    .average_wind
                    .set_angle(((rdr.pop_i16() as f32) * 0.001).rad());
                cm.sensor
                    .average_wind
                    .set_speed((rdr.pop_i16() as f32).km_h());
            }
            _ => (), // all other frames are ignored
        }
    }

    fn can_frame_read_sensor_values(&mut self, cm: &mut CoreModel, frame: &SpecificFrame) {

        let mut rdr = frame.can_frame.reader();

        match frame.specific_id {
            sensor::EULER_ROLL_NICK => {
                if let Some(roll) = rdr.pop_f32() {
                    cm.sensor.euler_roll = roll.rad();
                }
                if let Some(pitch) = rdr.pop_f32() {
                    cm.sensor.euler_pitch = pitch.rad();
                }
            },
            sensor::EULER_YAW_TURN_RATE => {
                if let Some(yaw) = rdr.pop_f32() {
                    cm.sensor.euler_yaw = yaw.rad();
                }
                if let Some(turn_rate) = rdr.pop_f32() {
                    cm.sensor.turn_rate = turn_rate.rad_s();
                }
            },
            sensor::TAS_IAS => {
                let tas = rdr.pop_f32();
                let ias = rdr.pop_f32();
                if tas.is_some() && ias.is_some() {
                    cm.sensor.airspeed = AirSpeed::from_speeds(ias.unwrap().m_s(), tas.unwrap().m_s());
               }
            },
            sensor::VARIO_AV_VARIO => {
                if let Some(climb_rate) = rdr.pop_f32() {
                    cm.sensor.climb_rate = climb_rate.m_s();
                    cm.control.can_devices |= CanActive::SensorboxLegacy as u32; // vario ok -> canbus ok
                }
                if let Some(average_climb_rate) = rdr.pop_f32() {
                    cm.sensor.average_climb_rate = average_climb_rate.m_s();
                }
            },
            sensor::WIND_DIR_SPEED => {
                if let Some(wind_dir) = rdr.pop_f32() {
                    cm.sensor.wind_vector.set_angle(wind_dir.rad());
                }
                if let Some(wind_speed) = rdr.pop_f32() {
                    cm.sensor.wind_vector.set_speed(wind_speed.m_s());
                }
            },
            sensor::AV_WIND_DIR_SPEED => {
                if let Some(avg_wind_dir) = rdr.pop_f32() {
                    cm.sensor.average_wind.set_angle(avg_wind_dir.rad());
                }
                if let Some(avg_wind_speed) = rdr.pop_f32() {
                    cm.sensor.average_wind.set_speed(avg_wind_speed.m_s());
                }
            },
            sensor::AMB_PRESS_AIR_DENS => {
                if let Some(pressure) = rdr.pop_f32() {
                    cm.sensor.pressure = pressure.n_m2();
                    cm.sensor
                        .pressure_altitude
                        .set_static_pressure(cm.sensor.pressure);
                }
                if let Some(density) = rdr.pop_f32() {
                    cm.sensor.density = density.kg_m3();
                }
            },
            sensor::G_FORCE_VERTICAL_GF => {
                if let Some(g_force) = rdr.pop_f32() {
                    cm.sensor.g_force = g_force.m_s2();
                }
                if let Some(vertical_g_force) = rdr.pop_f32() {
                    cm.sensor.vertical_g_force = vertical_g_force.m_s2();
                }
            },
            sensor::SLIP_PITCH_ANGLE => {
                if let Some(slip_angle) = rdr.pop_f32() {
                    cm.sensor.slip_angle = slip_angle.rad();
                }
                if let Some(nick_angle) = rdr.pop_f32() {
                    cm.sensor.nick_angle = nick_angle.rad();
                }
            },
            sensor::UBATT_CIRCLE_MODE => {
                if let Some(_ubatt) = rdr.pop_f32() {
                    // we ignore ubatt from sensorbox device
                }
                match rdr.pop_u8() {
                    0 => cm.control.fly_mode = FlyMode::StraightFlight,
                    2 => cm.control.fly_mode = FlyMode::Circling,
                    _ => (),
                }
            },
            _ => (),
        }
    }

    fn can_frame_read_gps_values(&mut self, cm: &mut CoreModel, frame: &SpecificFrame) {
        let mut rdr = frame.can_frame.reader();

        match frame.specific_id {
            gps::DATE_TIME => {
                let year = rdr.pop_u16();
                let month = rdr.pop_u8();
                let day = rdr.pop_u8();
                let hour = rdr.pop_u8();
                let min = rdr.pop_u8();
                let sec = rdr.pop_u8();
                cm.sensor
                    .gps_date_time
                    .set_date_time(year, month, day, hour, min, sec);
            }
            gps::LATITUDE => {
                if let Some(latitude) = rdr.pop_f64() {
                    cm.sensor.gps_lat = Latitude(latitude.rad())
                }
            }
            gps::LONGITUDE => {
                if let Some(longitude) = rdr.pop_f64() {
                    cm.sensor.gps_lon = Longitude(longitude.rad())
                }
            }
            gps::ALTITUDE_GEO_SEP => {
                if let Some(altitude) = rdr.pop_f32() {
                    cm.sensor.gps_altitude = altitude.m();
                }
                if let Some(geo_seperation) = rdr.pop_f32() {
                    cm.sensor.gps_geo_seperation = geo_seperation.m();
                }
            }
            gps::GROUND_TRACK_SPEED => {
                if let Some(track) = rdr.pop_f32() {
                    cm.sensor.gps_track = track.rad();
                }
                if let Some(speed) = rdr.pop_f32() {
                    cm.sensor.gps_ground_speed = speed.m_s();
                }
            }
            gps::NO_SAT_FIX_TYPE => {
                cm.sensor.gps_sats = rdr.pop_u8();
                match rdr.pop_u8() {
                    1 => cm.sensor.gps_state = GpsState::PosAvail,
                    3 => cm.sensor.gps_state = GpsState::HeadingAvail,
                    _ => cm.sensor.gps_state = GpsState::NoGps,
                }
            }
            _ => (),
        }
    }
}
