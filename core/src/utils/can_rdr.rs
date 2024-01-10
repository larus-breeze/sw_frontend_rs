use crate::{
    AirSpeed, CoreModel, FloatToAcceleration, FloatToAngularVelocity, FloatToDensity,
    FloatToPressure, FloatToSpeed, FlyMode,
};
use byteorder::{ByteOrder, LittleEndian as LE};
use can_dispatch::CanFrame;
use embedded_graphics::prelude::AngleUnit;

use crate::utils::sensor;

pub fn read_can_frame(cm: &mut CoreModel, frame: &CanFrame) {
    let id = frame.id();
    let mut rdr = Reader::new(frame.data());

    match id {
        sensor::AIRSPEED => {
            let tas = (rdr.pop_i16() as f32).km_h();
            let ias = (rdr.pop_i16() as f32).km_h();
            cm.sensor.airspeed = AirSpeed::from_speeds(ias, tas);
        }
        sensor::VARIO => {
            cm.sensor.climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            cm.sensor.average_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
        }
        sensor::ATHMOSPHERE => {
            cm.sensor.pressure = (rdr.pop_u32() as f32).n_m2();
            cm.sensor.density = (rdr.pop_u32() as f32).g_m3();
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
        sensor::ACCELERATION => {
            cm.sensor.g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
            cm.sensor.vertical_g_force = ((rdr.pop_i16() as f32) * 0.001).m_s2();
            cm.sensor.gps_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            match rdr.pop_u8() {
                2 => cm.control.fly_mode = FlyMode::Circling,
                1 => cm.control.fly_mode = FlyMode::Transition,
                _ => cm.control.fly_mode = FlyMode::StraightFlight,
            }
        }
        sensor::TURN_COORD => {
            cm.sensor.slip_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
            cm.sensor.turn_rate = ((rdr.pop_i16() as f32) * 0.001).rad_s();
            cm.sensor.nick_angle = ((rdr.pop_i16() as f32) * 0.001).rad();
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
