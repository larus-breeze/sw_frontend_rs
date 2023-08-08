use crate::{AirSpeed, CoreModel, FloatToDensity, FloatToPressure, FloatToSpeed};
use byteorder::{ByteOrder, LittleEndian as LE};
use embedded_graphics::prelude::AngleUnit;

use crate::utils::sensor;
use embedded_can::{Frame, Id};

pub fn read_can_frame<F: Frame>(core_model: &mut CoreModel, frame: &F) {
    let id = match frame.id() {
        Id::Extended(_) => return, // we don't use extended Ids
        Id::Standard(standard_id) => standard_id.as_raw(),
    };
    let mut rdr = Reader::new(frame.data());

    match id {
        sensor::AIRSPEED => {
            let tas = (rdr.pop_u16() as f32).km_h();
            let ias = (rdr.pop_u16() as f32).km_h();
            core_model.sensor.airspeed = AirSpeed::from_speeds(ias, tas);
        }
        sensor::VARIO => {
            core_model.sensor.climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
            core_model.sensor.average_climb_rate = ((rdr.pop_i16() as f32) * 0.001).m_s();
        }
        sensor::WIND => {
            core_model
                .sensor
                .wind
                .set_angle(((rdr.pop_i16() as f32) * 0.001).rad());
            core_model
                .sensor
                .wind
                .set_speed((rdr.pop_u16() as f32).km_h());
            core_model
                .sensor
                .average_wind
                .set_angle(((rdr.pop_i16() as f32) * 0.001).rad());
            core_model
                .sensor
                .average_wind
                .set_speed((rdr.pop_u16() as f32).km_h());
        }
        sensor::ATHMOSPHERE => {
            core_model.sensor.pressure = (rdr.pop_u16() as f32).n_m2();
            core_model.sensor.density = (rdr.pop_u16() as f32).g_m3();
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
