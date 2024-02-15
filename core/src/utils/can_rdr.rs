use crate::{
    model::VarioModeControl, AirSpeed, CanFrame, CoreModel, 
    FloatToAcceleration, FloatToAngularVelocity, FloatToDensity, FloatToLength, FloatToMass, FloatToPressure, FloatToSpeed, 
    FlyMode, Frame, GenericFrame, GenericId, SpecificFrame, SysConfigId
};
use byteorder::{ByteOrder, LittleEndian as LE};
use embedded_graphics::prelude::AngleUnit;

use crate::utils::{sensor_legacy, object_id};

pub fn read_can_frame(cm: &mut CoreModel, frame: &Frame) {
    match frame {
        Frame::Generic(generic_frame) => read_generic_frame(cm, generic_frame),
        Frame::Specific(specific_frame) => read_specific_frame(cm, specific_frame),
        Frame::Legacy(can_frame) => read_legacy_frame(cm, can_frame),
    }
}

fn read_specific_frame(cm: &mut CoreModel, frame: &SpecificFrame) {
    match frame.object_id {
        object_id::SENSOR => read_sensor_values(cm, frame),
        _ => (),
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
fn read_sensor_values(_cm: &mut CoreModel, _frame: &SpecificFrame) {

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

fn read_generic_frame(cm: &mut CoreModel, frame: &GenericFrame) {
    let mut rdr: Reader<'_> = Reader::new(frame.can_frame.data());
    #[allow(clippy::single_match)]
    match GenericId::from(frame.generic_id) {
        GenericId::SetSysSetting => {
            let config_id = SysConfigId::from(rdr.pop_u16());
            read_sys_config_value(cm, config_id, &frame.can_frame)
        }
        _ => (),
    }
}

fn read_sys_config_value(cm: &mut CoreModel, config_id: SysConfigId, frame: &CanFrame) {
    match config_id {
        SysConfigId::MacCready => {
            cm.config.mc_cready = frame.read_f32(4).m_s();
            cm.push_persistence_id(crate::PersistenceId::McCready);
        }
        SysConfigId::PilotWeight => {
            cm.glider_data.pilot_weight = frame.read_f32(4).kg();
            cm.push_persistence_id(crate::PersistenceId::PilotWeight);
        }
        SysConfigId::VolumeVario => {
            cm.config.volume = frame.read_u8(2) as i8;
            cm.push_persistence_id(crate::PersistenceId::Volume);
        }
        SysConfigId::WaterBallast => {
            cm.glider_data.water_ballast = frame.read_f32(4).kg();
            cm.push_persistence_id(crate::PersistenceId::WaterBallast);
        }
        SysConfigId::VarioModeControl => {
            cm.control.vario_mode_control = VarioModeControl::from(frame.read_u8(2));
            cm.push_persistence_id(crate::PersistenceId::VarioModeControl);
        }
        _ => (),
    }
}

fn read_legacy_frame(cm: &mut CoreModel, frame: &CanFrame) {
    let id = frame.id();
    let mut rdr = Reader::new(frame.data());

    match id {
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
        sensor_legacy::TURN_COORD => {
            cm.sensor.slip_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
            cm.sensor.turn_rate = ((rdr.pop_i16() as f32) * 0.001).rad_s();
            cm.sensor.nick_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
        }
        sensor_legacy::VARIO => {
            cm.sensor.climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            cm.sensor.average_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
