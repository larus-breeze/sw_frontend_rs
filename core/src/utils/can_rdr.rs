use crate::{CoreModel, FloatToSpeed, AirSpeed, FloatToPressure, FloatToDensity};
use byteorder::{LittleEndian as LE, ByteOrder};
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
            let tas = rdr.u16_into().km_h();
            let ias = rdr.u16_into().km_h();
            core_model.sensor.airspeed = AirSpeed::from_speeds(ias, tas);
        },
        sensor::VARIO => {
            core_model.sensor.climb_rate = (rdr.i16_into() * 0.001).m_s();
            core_model.sensor.average_climb_rate = (rdr.i16_into() * 0.001).m_s();
        },
        sensor::WIND => {
            core_model.sensor.wind.set_angle((rdr.i16_into() * 0.001).rad());
            core_model.sensor.wind.set_speed(rdr.u16_into().km_h());
            core_model.sensor.average_wind.set_angle((rdr.i16_into() * 0.001).rad());
            core_model.sensor.average_wind.set_speed(rdr.u16_into().km_h());
        },
        sensor::ATHMOSPHERE => {
            core_model.sensor.pressure = rdr.u16_into().n_m2();
            core_model.sensor.density = rdr.u16_into().g_m3();
        },
        _ => (), // all other frames are ignored
    }
}

struct Reader<'a> {
    data: &'a[u8],
    pos: usize,
}

impl <'a>Reader<'a> {
    #[inline]
    #[allow(unused)]
    fn new(data: &'a[u8]) -> Self {
        Reader {data, pos:0}
    }

    #[inline]
    #[allow(unused)]
    fn u32_into(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 4;
        LE::read_u32(&self.data[idx..self.pos]) as f32
    }

    #[inline]
    #[allow(unused)]
    fn u16_into(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 2;
        LE::read_u16(&self.data[idx..self.pos]) as f32
    }

    #[inline]
    #[allow(unused)]
    fn u8_into(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 1;
        self.data[idx] as f32
    }

    #[inline]
    #[allow(unused)]
    fn i32_into(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 4;
        LE::read_i32(&self.data[idx..self.pos]) as f32
    }

    #[inline]
    #[allow(unused)]
    fn i16_into(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 2;
        LE::read_i16(&self.data[idx..self.pos]) as f32
    }

    #[inline]
    #[allow(unused)]
    fn i8_into(&mut self) -> f32 {
        let idx = self.pos;
        self.pos += 1;
        (self.data[idx] as i8) as f32
    }

    #[inline]
    #[allow(unused)]
    fn f32_into(&mut self) -> f32 {
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
