use crate::{
    controller::{
        helpers::{
            can_ids::{gps, sensor, sensor_legacy},
            frontend_masster, object_id, CanActive,
        },
        persist, Echo,
    },
    into_range_0_360, into_range_180_180,
    model::{editable::Content, GpsState, VarioModeControl},
    persist::set_vario_mode,
    AirSpeed, Angle, CanFrame, CoreController, CoreModel, Date, DateTime, F64ToCoord,
    FloatToAcceleration, FloatToAngularVelocity, FloatToDensity, FloatToLength, FloatToMass,
    FloatToPressure, FloatToSpeed, Frame, GenericFrame, GenericId, Latitude, Longitude,
    PersistenceId, SpecificFrame, SwVersion, Variant, VarioMode, DEGREE_PER_RAD,
};
use embedded_graphics::prelude::AngleUnit;

use super::CanConfigId;

impl CoreController {
    pub fn read_can_frame(&mut self, cm: &mut CoreModel, frame: &Frame) {
        match frame {
            Frame::Generic(generic_frame) => self.can_frame_read_generic(cm, generic_frame),
            Frame::Specific(specific_frame) => self.can_frame_read_specific(cm, specific_frame),
            Frame::Legacy(can_frame) => self.can_frame_read_legacy(cm, can_frame),
            Frame::IsMaster(is_master) => cm.control.is_can_master = *is_master,
        }
    }

    fn can_frame_read_generic(&mut self, cm: &mut CoreModel, frame: &GenericFrame) {
        let mut rdr = frame.can_frame.reader();
        #[allow(clippy::single_match)]
        match GenericId::from(frame.generic_id) {
            GenericId::SetSysSetting => {
                if let Some(value) = rdr.pop_u16() {
                    let config_id = CanConfigId::from(value);
                    self.can_frame_read_sys_config_value(cm, config_id, &frame.can_frame)
                }
            }
            _ => (),
        }
        if frame.can_frame.id() == 0x521 {
            // Version Sensorbox
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&frame.can_frame.data()[4..8]);
            cm.sensor.sw_version = SwVersion::from_bytes(bytes);
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
                persist::persist_set(
                    self,
                    cm,
                    Variant::I8(val),
                    PersistenceId::Volume,
                    Echo::Nmea,
                );
            }
            CanConfigId::MacCready => {
                if let Some(val) = frame.read_opt_f32(4) {
                    let val = val.m_s();
                    persist::persist_set(
                        self,
                        cm,
                        Variant::Speed(val),
                        PersistenceId::McCready,
                        Echo::Nmea,
                    );
                }
            }
            CanConfigId::WaterBallast => {
                if let Some(val) = frame.read_opt_f32(4) {
                    let val = val.kg();
                    persist::persist_set(
                        self,
                        cm,
                        Variant::Mass(val),
                        PersistenceId::WaterBallast,
                        Echo::Nmea,
                    );
                }
            }
            CanConfigId::Bugs => {
                if let Some(val) = frame.read_opt_f32(4) {
                    persist::persist_set(
                        self,
                        cm,
                        Variant::F32(val),
                        PersistenceId::Bugs,
                        Echo::Nmea,
                    );
                }
            }
            CanConfigId::Qnh => {
                if let Some(val) = frame.read_opt_f32(4) {
                    let val = val.hpa();
                    persist::persist_set(
                        self,
                        cm,
                        Variant::Pressure(val),
                        PersistenceId::Qnh,
                        Echo::Nmea,
                    );
                }
            }
            CanConfigId::PilotWeight => {
                if let Some(val) = frame.read_opt_f32(4) {
                    let val = val.kg();
                    persist::persist_set(
                        self,
                        cm,
                        Variant::Mass(val),
                        PersistenceId::PilotWeight,
                        Echo::Nmea,
                    )
                }
            }
            CanConfigId::VarioModeControl => (), // do nothing
            CanConfigId::TcClimbRate => {
                if let Some(val) = frame.read_opt_f32(4) {
                    persist::persist_set(
                        self,
                        cm,
                        Variant::F32(val),
                        PersistenceId::TcClimbRate,
                        Echo::None,
                    )
                }
            }
            CanConfigId::TcSpeedToFly => {
                if let Some(val) = frame.read_opt_f32(4) {
                    persist::persist_set(
                        self,
                        cm,
                        Variant::F32(val),
                        PersistenceId::TcSpeedToFly,
                        Echo::None,
                    )
                }
            }
            CanConfigId::VarioMode => {
                let vario_mode = VarioMode::from(frame.read_u8(2));
                set_vario_mode(cm, self, vario_mode, VarioModeControl::Can);
            }
            CanConfigId::WaterBallastFraction => {
                if let Some(fraction) = frame.read_opt_f32(4) {
                    cm.glider_data.set_ballast_fraction(fraction);
                    persist::persist_set(
                        self,
                        cm,
                        Variant::Mass(cm.glider_data.water_ballast),
                        PersistenceId::WaterBallast,
                        Echo::Nmea,
                    )
                }
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

        if id == frontend_masster::AVG_CLIMB_RATES {
            cm.control.avg_climb_slave_ticks = 3; // 3s timeout for slave mode
            if let Some(av2_climb_rate) = rdr.pop_f32() {
                cm.calculated.av2_climb_rate = av2_climb_rate.m_s();
            }
            if let Some(thermal_climb_rate) = rdr.pop_f32() {
                cm.calculated.thermal_climb_rate = thermal_climb_rate.m_s();
            }
        } else {
            match id {
                sensor_legacy::EULER_ANGLES => {
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.euler_roll = norm_mpi_ppi(value);
                    }
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.euler_pitch = norm_mpi_ppi(value);
                    }
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.euler_yaw = norm_0_2pi(value);
                    }
                }
                sensor_legacy::ACCELERATION => {
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.g_force = ((value as f32) * 0.001).m_s2();
                    }
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.vertical_g_force = ((value as f32) * 0.001).m_s2();
                    }
                }
                sensor_legacy::AIRSPEED => {
                    if let (Some(tas), Some(ias)) = (rdr.pop_i16(), rdr.pop_i16()) {
                        cm.sensor.airspeed = AirSpeed::from_speeds((ias as f32).km_h(), (tas as f32).km_h());
                    }
                }
                sensor_legacy::ATHMOSPHERE => {
                    if let Some(value) = rdr.pop_u32() {
                        cm.sensor.pressure = (value as f32).n_m2();
                    }
                    if let Some(value) = rdr.pop_u32() {
                        cm.sensor.density = (value as f32).g_m3();
                    }
                    cm.sensor
                        .pressure_altitude
                        .set_static_pressure(cm.sensor.pressure);
                }
                sensor_legacy::GPS_DATE_TIME => {
                    if let (Some(year), Some(month), Some(day), Some(hour), Some(min), Some(sec))= 
                        (rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8()) {
                        let date_time = DateTime::from_vals(year as u16 +2000, month, day, hour, min, sec);
                        persist::set_date_time(cm, self, date_time);
                    }
                }
                sensor_legacy::GPS_LAT_LON => {
                    if let Some(value) = rdr.pop_i32() {
                        cm.sensor.gps_lat = Latitude(((value as f64) * 1.0e-7).deg());
                    }
                    if let Some(value) = rdr.pop_i32() {
                        cm.sensor.gps_lon = Longitude(((value as f64) * 1.0e-7).deg());
                    }
                }
                sensor_legacy::GPS_ALT => {
                    if let Some(value) = rdr.pop_i32() {
                        cm.sensor.gps_altitude = (value as f32).mm();
                    }
                    if let Some(value) = rdr.pop_i32() {
                        cm.sensor.gps_geo_seperation = (value as f32 * 0.1).m();
                    }
                }
                sensor_legacy::GPS_TRK_SPD => {
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.gps_track = (value as f32 * 0.001).rad();
                        if let Some(value) = rdr.pop_u16() {
                            cm.sensor.gps_ground_speed = (value as f32).km_h();
                            if cm.sensor.gps_ground_speed < 1.0.km_h() {
                                cm.sensor.gps_track = 0.0_f32.rad();
                            }
                            if cm.sensor.gps_track < 0.0_f32.rad() {
                                cm.sensor.gps_track += 360.0_f32.deg();
                            }
                        }
                    }
                }
                sensor_legacy::GPS_SATS => {
                    if let Some(value) = rdr.pop_u8() {
                        cm.sensor.gps_sats = value;
                    }
                    if let Some(value) = rdr.pop_u8() {
                        match value {
                            1 => cm.sensor.gps_state = GpsState::PosAvail,
                            3 => cm.sensor.gps_state = GpsState::HeadingAvail,
                            _ => cm.sensor.gps_state = GpsState::NoGps,
                        }
                    }
                }
                sensor_legacy::TURN_COORD => {
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.slip_angle = ((value as f32) * 0.001).rad();
                    }
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.turn_rate = ((value as f32) * 0.001).rad_s();                
                    }
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.nick_angle = ((value as f32) * 0.001).rad();
                    }
                }
                sensor_legacy::VARIO => {
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.climb_rate = ((value as f32) * 0.001).m_s();
                    }
                    if let Some(value) = rdr.pop_i16() {
                        cm.sensor.average_climb_rate = ((value as f32) * 0.001).m_s();
                    }
                    cm.control.can_devices |= CanActive::SensorboxLegacy as u32;
                }
                sensor_legacy::WIND => {
                    if let (Some(angle), Some(speed)) = (rdr.pop_i16(), rdr.pop_i16()) {
                        cm.sensor
                            .wind_vector
                            .set_angle(((angle as f32) * 0.001).rad());
                        cm.sensor
                            .wind_vector
                            .set_speed((speed as f32).km_h());
                    }
                    if let (Some(angle), Some(speed)) = (rdr.pop_i16(), rdr.pop_i16()) {
                        cm.sensor
                            .average_wind
                            .set_angle(((angle as f32) * 0.001).rad());
                        cm.sensor
                            .average_wind
                            .set_speed((speed as f32).km_h());
                    }
                }
                _ => (), // all other frames are ignored
            }
        }
    }

    fn can_frame_read_sensor_values(&mut self, cm: &mut CoreModel, frame: &SpecificFrame) {
        let mut rdr = frame.can_frame.reader();

        match frame.specific_id {
            sensor::EULER_ROLL_NICK => {
                if let Some(roll) = rdr.pop_f32() {
                    cm.sensor.euler_roll = into_range_180_180(roll.rad());
                }
                if let Some(pitch) = rdr.pop_f32() {
                    cm.sensor.euler_pitch = into_range_180_180(pitch.rad());
                }
            }
            sensor::EULER_YAW_TURN_RATE => {
                if let Some(yaw) = rdr.pop_f32() {
                    cm.sensor.euler_yaw = into_range_0_360(yaw.rad());
                }
                if let Some(turn_rate) = rdr.pop_f32() {
                    cm.sensor.turn_rate = turn_rate.rad_s();
                }
            }
            sensor::TAS_IAS => {
                let tas = rdr.pop_f32();
                let ias = rdr.pop_f32();
                if let Some(tas) = tas {
                    if let Some(ias) = ias {
                        cm.sensor.airspeed = AirSpeed::from_speeds(ias.m_s(), tas.m_s());
                    }
                }
            }
            sensor::VARIO_AV_VARIO => {
                if let Some(climb_rate) = rdr.pop_f32() {
                    cm.sensor.climb_rate = climb_rate.m_s();
                    cm.control.can_devices |= CanActive::SensorboxLegacy as u32;
                    // vario ok -> canbus ok
                }
                if let Some(average_climb_rate) = rdr.pop_f32() {
                    cm.sensor.average_climb_rate = average_climb_rate.m_s();
                }
            }
            sensor::WIND_DIR_SPEED => {
                if let Some(wind_dir) = rdr.pop_f32() {
                    cm.sensor.wind_vector.set_angle(wind_dir.rad());
                }
                if let Some(wind_speed) = rdr.pop_f32() {
                    cm.sensor.wind_vector.set_speed(wind_speed.m_s());
                }
            }
            sensor::AV_WIND_DIR_SPEED => {
                if let Some(avg_wind_dir) = rdr.pop_f32() {
                    cm.sensor.average_wind.set_angle(avg_wind_dir.rad());
                }
                if let Some(avg_wind_speed) = rdr.pop_f32() {
                    cm.sensor.average_wind.set_speed(avg_wind_speed.m_s());
                }
            }
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
            }
            sensor::G_FORCE_VERTICAL_GF => {
                if let Some(g_force) = rdr.pop_f32() {
                    cm.sensor.g_force = g_force.m_s2();
                }
                if let Some(vertical_g_force) = rdr.pop_f32() {
                    cm.sensor.vertical_g_force = vertical_g_force.m_s2();
                }
            }
            sensor::SLIP_PITCH_ANGLE => {
                if let Some(slip_angle) = rdr.pop_f32() {
                    cm.sensor.slip_angle = slip_angle.rad();
                }
                if let Some(nick_angle) = rdr.pop_f32() {
                    cm.sensor.nick_angle = nick_angle.rad();
                }
            }
            sensor::UBATT_CIRCLE_MODE => (), // ignore this datagram
            sensor::SYSTEM_STATE_GIT_TAG => {
                if let Some(value) = rdr.pop_u32() {
                    cm.sensor.larus_box_system_state = value;
                }
            }
            sensor::CONFIG_VALUE => {
                if let Some(value) = rdr.pop_u32() {
                    let config_id = CanConfigId::from(value as u16);
                    match config_id {
                        CanConfigId::SensTiltRoll
                        | CanConfigId::SensTiltPitch
                        | CanConfigId::SensTiltYaw => {
                            if let Some(rad) = rdr.pop_f32() {
                                let deg = rad * DEGREE_PER_RAD;
                                cm.control.editor.content = Content::F32(Some(deg));
                            }
                        }
                        CanConfigId::BlockHorizon => {
                            if let Some(value) = rdr.pop_u32() {
                                let date = Date::from_u32(value);
                                cm.control.editor.content = Content::Date(Some(date));
                            }
                        }
                        _ => cm.control.editor.content = Content::F32(rdr.pop_f32()),
                    }
                }
            }
            _ => (),
        }
    }

    fn can_frame_read_gps_values(&mut self, cm: &mut CoreModel, frame: &SpecificFrame) {
        let mut rdr = frame.can_frame.reader();

        match frame.specific_id {
            gps::DATE_TIME => {
                if let (Some(year), Some(month), Some(day), Some(hour), Some(min), Some(sec))= 
                       (rdr.pop_u16(), rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8(), rdr.pop_u8()) {
                    let date_time = DateTime::from_vals(year, month, day, hour, min, sec);
                    persist::set_date_time(cm, self, date_time);
                }
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
                if let Some(value) = rdr.pop_u8() {
                    cm.sensor.gps_sats = value;
                }
                if let Some(value) = rdr.pop_u8() {
                    match value {
                        1 => cm.sensor.gps_state = GpsState::PosAvail,
                        3 => cm.sensor.gps_state = GpsState::HeadingAvail,
                        _ => cm.sensor.gps_state = GpsState::NoGps,
                    }
                }
            }
            _ => (),
        }
    }
}
