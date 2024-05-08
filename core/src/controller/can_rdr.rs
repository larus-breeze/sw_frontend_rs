use crate::{
    model::{GpsState, VarioModeControl}, AirSpeed, Angle, CanActive, CanFrame, CoreController, CoreModel, F64ToCoord, FloatToAcceleration, FloatToAngularVelocity, FloatToDensity, FloatToLength, FloatToMass, FloatToPressure, FloatToSpeed, FlyMode, Frame, GenericFrame, GenericId, Latitude, Longitude, SpecificFrame, SysConfigId
};
use byteorder::{ByteOrder, LittleEndian as LE};
use embedded_graphics::prelude::AngleUnit;

use crate::utils::{object_id, sensor_legacy};

impl CoreController {
    pub fn read_can_frame(&mut self, cm: &mut CoreModel, frame: &Frame) {
        match frame {
            Frame::Generic(generic_frame) => self.can_frame_read_generic(cm, generic_frame),
            Frame::Specific(specific_frame) => self.can_frame_read_specific(cm, specific_frame),
            Frame::Legacy(can_frame) => self.can_frame_read_legacy(cm, can_frame),
        }
    }

    fn can_frame_read_generic(&mut self, cm: &mut CoreModel, frame: &GenericFrame) {
        let mut rdr: Reader<'_> = Reader::new(frame.can_frame.data());
        #[allow(clippy::single_match)]
        match GenericId::from(frame.generic_id) {
            GenericId::SetSysSetting => {
                let config_id = SysConfigId::from(rdr.pop_u16());
                self.can_frame_read_sys_config_value(cm, config_id, &frame.can_frame)
            }
            _ => (),
        }
    }

    fn can_frame_read_specific(&mut self, cm: &mut CoreModel, frame: &SpecificFrame) {
        #[allow(clippy::single_match)]
        match frame.object_id {
            object_id::SENSOR => self.can_frame_read_sensor_values(cm, frame),
            _ => (),
        }
    }

    fn can_frame_read_sys_config_value(&mut self, cm: &mut CoreModel, config_id: SysConfigId, frame: &CanFrame) {
        match config_id {
            SysConfigId::MacCready => {
                cm.config.mc_cready = frame.read_f32(4).m_s();
                self.push_persistence_id(cm, crate::PersistenceId::McCready);
            }
            SysConfigId::PilotWeight => {
                cm.glider_data.pilot_weight = frame.read_f32(4).kg();
                self.push_persistence_id(cm, crate::PersistenceId::PilotWeight);
            }
            SysConfigId::VolumeVario => {
                cm.config.volume = frame.read_u8(2) as i8;
                self.push_persistence_id(cm, crate::PersistenceId::Volume);
            }
            SysConfigId::WaterBallast => {
                cm.glider_data.water_ballast = frame.read_f32(4).kg();
                self.push_persistence_id(cm, crate::PersistenceId::WaterBallast);
            }
            SysConfigId::VarioModeControl => {
                cm.control.vario_mode_control = VarioModeControl::from(frame.read_u8(2));
                self.push_persistence_id(cm, crate::PersistenceId::VarioModeControl);
            }
            _ => (),
        }
    }

    fn can_frame_read_legacy(&mut self, cm: &mut CoreModel, frame: &CanFrame) {
        fn norm_rad(mut r: i16) -> Angle {
            if r < 0 {
                r += 6284
            }
            ((r as f32) * 0.001).rad()
        }

        let id = frame.id();
        let mut rdr = Reader::new(frame.data());

        match id {
            sensor_legacy::EULER_ANGLES => {
                cm.sensor.euler_roll = norm_rad(rdr.pop_i16());
                cm.sensor.euler_nick = norm_rad(rdr.pop_i16());
                cm.sensor.euler_yaw = norm_rad(rdr.pop_i16());
            }
            sensor_legacy::ACCELERATION => {
                cm.sensor.g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
                cm.sensor.vertical_g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
                cm.sensor.gps_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
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

    fn can_frame_read_sensor_values(&mut self, _cm: &mut CoreModel, _frame: &SpecificFrame) {

        /* FIXME: This is an unfinished fragment
        let mut rdr: Reader<'_> = Reader::new(frame.can_frame.data());
        match frame.specific_id {
            //sensor::EULER_ANGLES => (),
            sensor::HEADING_MAGN_DECL => (),
            sensor::TAS_IAS => {
                let (tas_ok, tas) = (rdr.f32_is_finite(), rdr.pop_f32());
                let (ias_ok, ias) = (rdr.f32_is_finite(), rdr.pop_f32());
                if tas_ok && ias_ok {
                    cm.sensor.airspeed = AirSpeed::from_speeds(ias.m_s(), tas.m_s());
                }
            },
            sensor::VARIO_AV_VARIO => (),
            sensor::WIND_DIR_SPEED => (),
            sensor::AV_WIND_DIR_SPEED => (),
            sensor::AMB_PRESS_AIR_DENS => {
                if rdr.f32_is_finite() {
                    cm.sensor.pressure = rdr.pop_f32().n_m2();
                }
                if rdr.f32_is_finite() {
                    cm.sensor.density = rdr.pop_f32().g_m3();
                }
            },
            sensor::AC_ANG_FRONT_RIGHT => (),
            sensor::TURN_RATE_STATE => (),
            sensor::CALC_TRIFT_ANGLE => (),
            sensor::SYSTEM_STATE_GIT => (),
            sensor::SUPPLY_VOLTAGE => (),
            _ => (),

        }*/
    }
}

/* FIXME: This is an unfinished fragment

missing:
        sensor_legacy::ACCELERATION => {
            cm.sensor.g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
            cm.sensor.vertical_g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
            cm.sensor.gps_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            match rdr.pop_u8() {
                0 => cm.control.fly_mode = FlyMode::StraightFlight,
                2 => cm.control.fly_mode = FlyMode::Circling,
                _ => (),
            }
        }
        sensor_legacy::GPS_ALT => {
            cm.sensor.gps_altitude = (rdr.pop_u32() as f32).mm();
            cm.sensor.gps_geo_seperation = (rdr.pop_u32() as f32 * 0.1).m();
        }
        sensor_legacy::GPS_TRK_SPD => {
            cm.sensor.gps_track = (rdr.pop_i16() as f32 * 0.001).rad();
            cm.sensor.gps_ground_speed = (rdr.pop_u16() as f32).km_h();
            if cm.sensor.gps_ground_speed < 1.0.km_h() {
                cm.sensor.gps_track = 0.0.rad();
            }
            if cm.sensor.gps_track < 0.0.rad() {
                cm.sensor.gps_track += 360.0.deg();
            }
        }

*/

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    #[inline]
    #[allow(unused)]
    fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    #[inline]
    #[allow(unused)]
    fn pop_u32(&mut self) -> u32 {
        let idx = self.pos;
        self.pos += 4;
        LE::read_u32(&self.data[idx..self.pos])
    }

    #[inline]
    #[allow(unused)]
    fn pop_u16(&mut self) -> u16 {
        let idx = self.pos;
        self.pos += 2;
        LE::read_u16(&self.data[idx..self.pos])
    }

    #[inline]
    #[allow(unused)]
    fn pop_u8(&mut self) -> u8 {
        let idx = self.pos;
        self.pos += 1;
        self.data[idx]
    }

    #[inline]
    #[allow(unused)]
    fn pop_i32(&mut self) -> i32 {
        let idx = self.pos;
        self.pos += 4;
        LE::read_i32(&self.data[idx..self.pos])
    }

    #[inline]
    #[allow(unused)]
    fn pop_i16(&mut self) -> i16 {
        let idx = self.pos;
        self.pos += 2;
        LE::read_i16(&self.data[idx..self.pos])
    }

    #[inline]
    #[allow(unused)]
    fn pop_i8(&mut self) -> i8 {
        let idx = self.pos;
        self.pos += 1;
        self.data[idx] as i8
    }

    #[inline]
    #[allow(unused)]
    fn pop_f32(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 4;
        LE::read_f32(&self.data[idx..self.pos])
    }

    #[inline]
    #[allow(unused)]
    fn f32_is_finite(&mut self) -> bool {
        let idx = self.pos;
        LE::read_f32(&self.data[self.pos..self.pos + 4]).is_finite()
    }

    #[inline]
    #[allow(unused)]
    fn pop_f64(&mut self) -> f64 {
        let idx = self.pos;
        self.pos += 8;
        LE::read_f64(&self.data[idx..self.pos])
    }

    #[inline]
    #[allow(unused)]
    fn f64_is_finite(&mut self) -> bool {
        let idx = self.pos;
        LE::read_f64(&self.data[self.pos..self.pos + 8]).is_finite()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
