use crate::{
    model::VarioModeControl, AirSpeed, CoreModel, FloatToAcceleration, FloatToAngularVelocity,
    FloatToDensity, FloatToLength, FloatToMass, FloatToPressure, FloatToSpeed, FlyMode, GenericId,
    SysConfigId,
    Frame, CanFrame, GenericFrame,
};
use byteorder::{ByteOrder, LittleEndian as LE};
use embedded_graphics::prelude::AngleUnit;

use crate::utils::sensor;

pub fn read_can_frame(cm: &mut CoreModel, frame: &Frame) {
    match frame {
        Frame::Generic(generic_frame) => read_generic_frame(cm, generic_frame),
        Frame::Specific(_specific_frame) => (),
        Frame::Legacy(can_frame) => read_legacy_frame(cm, can_frame),
    }
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
        sensor::ACCELERATION => {
            cm.sensor.g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
            cm.sensor.vertical_g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
            cm.sensor.gps_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            match rdr.pop_u8() {
                0 => cm.control.fly_mode = FlyMode::StraightFlight,
                2 => cm.control.fly_mode = FlyMode::Circling,
                _ => (),
            }
        }
        sensor::AIRSPEED => {
            let tas = (rdr.pop_i16() as f32).km_h();
            let ias = (rdr.pop_i16() as f32).km_h();
            cm.sensor.airspeed = AirSpeed::from_speeds(ias, tas);
        }
        sensor::ATHMOSPHERE => {
            cm.sensor.pressure = (rdr.pop_u32() as f32).n_m2();
            cm.sensor.density = (rdr.pop_u32() as f32).g_m3();
        }
        sensor::GPS_ALT => {
            cm.sensor.gps_altitude = (rdr.pop_u32() as f32).mm();
            cm.sensor.gps_geo_seperation = (rdr.pop_u32() as f32 * 0.1).m();
        }
        sensor::GPS_TRK_SPD => {
            cm.sensor.gps_track = (rdr.pop_i16() as f32 * 0.001).rad();
            cm.sensor.gps_ground_speed = (rdr.pop_u16() as f32).km_h();
            if cm.sensor.gps_ground_speed < 1.0.km_h() {
                cm.sensor.gps_track = 0.0.rad();
            }
            if cm.sensor.gps_track < 0.0.rad() {
                cm.sensor.gps_track += 360.0.deg();
            }
        }
        sensor::TURN_COORD => {
            cm.sensor.slip_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
            cm.sensor.turn_rate = ((rdr.pop_i16() as f32) * 0.001).rad_s();
            cm.sensor.nick_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
        }
        sensor::VARIO => {
            cm.sensor.climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            cm.sensor.average_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
        }
        sensor::WIND => {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
