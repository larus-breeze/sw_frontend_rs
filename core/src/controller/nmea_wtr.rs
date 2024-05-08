use tfmt::{uWrite, uwrite};

use crate::{
    CoreModel, SysConfigId, CoreError,
    controller::nmea_rdr::NmeaRxBuffer,
    utils::ParseSlice,
    FloatToSpeed, FloatToPressure, 
};
use super::GpsState;
use heapless::{Deque, Vec};

pub struct NmeaHandler {
    rx_data: NmeaRxBuffer,
    tx_data: NmeaTxBuffer,
    readout_idx: u32,
    pers_id: Deque<SysConfigId, 10>,
}

impl Default for NmeaHandler {
    fn default() -> Self {
        NmeaHandler {
            rx_data: NmeaRxBuffer::new(),
            tx_data: NmeaTxBuffer::new(),
            readout_idx: 0,
            pers_id: Deque::new(),
        }
    }
}


impl NmeaHandler {
    pub fn recv_u8(&mut self, cm: &mut CoreModel, b: u8) {
        if self.rx_data.recv_u8(b) {
            let _ = self.nmea_parse(cm);
        }
    }

    pub fn nmea_recv_slice(&mut self, cm: &mut CoreModel, bytes: &[u8]) {
        for b in bytes {
            self.recv_u8(cm, *b);
        }
    }

    fn nmea_parse(&mut self, cm: &mut CoreModel) -> Result<(), CoreError> {
        fn in_range(val: f32, lower: f32, upper: f32) -> Result<f32, CoreError> {
            if val >= lower && val <= upper {
                Ok(val)
            } else {
                Err(CoreError::ParseError)
            }
        }

        // check checksum
        self.rx_data.check()?;

        self.rx_data.compare_chunk(b"$PLARS")?;
        self.rx_data.compare_chunk(b"H")?;

        let cmd: Vec<u8, 10> = Vec::from_slice(self.rx_data.next_chunk()?)
            .map_err(|_| CoreError::ParseError)?;

        let s = self.rx_data.next_chunk()?;
        let val = f32::from_slice(s)?;

        match cmd.as_slice() {
            b"MC" => Ok(cm.config.mc_cready = in_range(val, 0.0, 9.9)?.m_s()),
            b"BAL" => Ok(cm
                .glider_data
                .set_ballast_ratio(in_range(val, 1.00, 1.60)?)),
            b"BUGS" => Ok(cm.glider_data.bugs = in_range(val, 0.0, 30.0)?),
            b"QNH" => Ok(cm
                .sensor
                .pressure_altitude
                .set_qnh(in_range(val, 900.0, 1100.0)?.hpa())),
            _ => Err(CoreError::ParseError),
        }
    }

    pub fn nmea_config(&mut self, id: SysConfigId) {
        // no error if deque is full
        match id {
            SysConfigId::Bugs
            | SysConfigId::MacCready
            | SysConfigId::WaterBallast
            | SysConfigId::Qnh => {
                let _ = self.pers_id.push_back(id);
            }
            _ => (),
        }
    }

    pub fn nmea_cyclic(&mut self, short: bool) {
        if short {
            self.readout_idx = 106;
        } else {
            self.readout_idx = 100;
        }
    }

    pub fn nmea_next(&mut self, cm: &mut CoreModel) -> Option<&[u8]> {
        if self.pers_id.len() > 0 {
            let id = self.pers_id.pop_front().unwrap();
            return self.nmea_plars(cm, id);
        }
        self.readout_idx += 1;
        match self.readout_idx {
            // rarely sent
            101 => Some(self.nmea_gprmc(cm)),
            102 => Some(self.nmea_gpgga(cm)),
            103 => Some(self.nmea_hchdt(cm)),
            104 => Some(self.nmea_plarw(cm, true)),
            105 => Some(self.nmea_plard(cm)),
            106 => Some(self.nmea_plarb(cm)),

            // often sent
            107 => Some(self.nmea_plarw(cm, false)),
            108 => Some(self.nmea_plara(cm)),
            109 => Some(self.nmea_plarv(cm)),
            _ => None,
        }
    }

    fn nmea_gprmc(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let _ = uwrite!(
            self.tx_data,
            "$GPRMC,{:n},A,{:n},{:n},{:.1},{:.1},{:n},,,A",
            cm.sensor.gps_date_time.time(),
            cm.sensor.gps_lat,
            cm.sensor.gps_lon,
            cm.sensor.gps_ground_speed.to_kt(),
            cm.sensor.gps_track.to_degrees(),
            cm.sensor.gps_date_time.date(),
        );
        self.tx_data.finish()
    }

    fn nmea_gpgga(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let gps_quality_indicator = match cm.sensor.gps_state {
            GpsState::PosAvail => 1,
            GpsState::HeadingAvail => 2,
            _ => 0,
        };
        let _ = uwrite!(
            self.tx_data,
            "$GPGGA,{:n},{:n},{:n},{},{},1.0,{:.1},M,{:.1},M,,",
            cm.sensor.gps_date_time.time(),
            cm.sensor.gps_lat,
            cm.sensor.gps_lon,
            gps_quality_indicator,
            cm.sensor.gps_sats,
            cm.sensor.gps_altitude.to_m(),
            cm.sensor.gps_geo_seperation.to_m(),
        );
        self.tx_data.finish()
    }

    fn nmea_hchdt(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let _ = uwrite!(
            self.tx_data,
            "$HCHDT,{:.1},T",
            cm.sensor.euler_yaw.to_degrees(),
        );
        self.tx_data.finish()
    }

    fn nmea_plara(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let _ = uwrite!(
            self.tx_data,
            "$PLARA,{:.1},{:.1},{:.1}",
            cm.sensor.euler_roll.to_degrees(),
            cm.sensor.euler_nick.to_degrees(),
            cm.sensor.euler_yaw.to_degrees(),
        );
        self.tx_data.finish()
    }

    fn nmea_plarb(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let _ = uwrite!(
            self.tx_data,
            "$PLARB,{:.2}",
            cm.device.supply_voltage,
        );
        self.tx_data.finish()
    }

    fn nmea_plard(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let _ = uwrite!(
            self.tx_data,
            "$PLARD,{:.2},M",
            cm.sensor.density.to_g_m3(),
        );
        self.tx_data.finish()
    }

    fn nmea_plars(&mut self, cm: &mut CoreModel, id: SysConfigId) -> Option<&[u8]> {
        self.tx_data.reset();
        let _ = match id {
            SysConfigId::MacCready => uwrite!(
                self.tx_data,
                "$PLARS,L,MC,{:.1}",
                cm.config.mc_cready.to_m_s()
            ),
            SysConfigId::WaterBallast => uwrite!(
                self.tx_data,
                "$PLARS,L,BAL,{:.2}",
                cm.glider_data.ballast_ratio(),
            ),
            SysConfigId::Bugs => uwrite!(
                self.tx_data,
                "$PLARS,L,BUGS,{:.0}",
                (cm.glider_data.bugs - 1.0) * 100.0
            ),
            SysConfigId::Qnh => uwrite!(
                self.tx_data,
                "$PLARS,L,QNH,{:.1}",
                cm.sensor.pressure_altitude.qnh().to_hpa()
            ),
            _ => return None,
        };
        Some(self.tx_data.finish())
    }

    fn nmea_plarv(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.tx_data.reset();
        let _ = uwrite!(
            self.tx_data,
            "$PLARV,{:.2},{:.2},{:.0},{:.0}",
            cm.sensor.climb_rate.to_m_s(),
            cm.sensor.average_climb_rate.to_m_s(),
            cm.sensor.pressure_altitude.qne_altitude().to_m(),
            cm.sensor.airspeed.tas().to_km_h(),
        );
        self.tx_data.finish()
    }

    fn nmea_plarw(&mut self, cm: &mut CoreModel, average: bool) -> &[u8] {
        self.tx_data.reset();
        let (kind, wind) = if average {
            ("A", cm.sensor.average_wind)
        } else {
            ("I", cm.sensor.wind_vector)
        };
        let _ = uwrite!(
            self.tx_data,
            "$PLARW,{:.0},{:.0},{},A",
            wind.angle().to_degrees(),
            wind.speed().to_km_h(),
            kind,
        );
        self.tx_data.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controller::nmea_wtr::NmeaHandler, model::GpsState, AirSpeed, Coord, CoreModel, FloatToDensity, FloatToLength, FloatToMass, FloatToPressure, FloatToSpeed, HwVersion, Latitude, Longitude, SwVersion, WindVector
    };
    use embedded_graphics::geometry::AngleUnit;

    const HW_VERSION: HwVersion = HwVersion::from_bytes([1, 3, 1, 0]);
    const SW_VERSION: SwVersion = SwVersion {
        version: [0, 0, 0, 0],
    };

    #[test]
    fn gpgga() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor
            .gps_date_time
            .set_date_time(2023, 06, 23, 12, 05, 20);
        cm.sensor.gps_lon = Longitude(Coord(-0.1498276674644056));
        cm.sensor.gps_lat = Latitude(Coord(-0.8672530930250163));
        cm.sensor.gps_state = GpsState::HeadingAvail;
        cm.sensor.gps_sats = 23;
        cm.sensor.gps_altitude = 2745.9.m();
        cm.sensor.gps_geo_seperation = 12.3.m();

        let s = nh.nmea_gpgga(&mut cm);
        assert_eq!(
            s,
            b"$GPGGA,120520.00,4941.39652,S,835.06958,W,2,23,1.0,2745.9,M,12.3,M,,*56\r\n"
        );
    }

    #[test]
    fn gprmc() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor
            .gps_date_time
            .set_date_time(2023, 06, 23, 12, 05, 20);
        cm.sensor.gps_lon = Longitude(Coord(0.1498276674644056));
        cm.sensor.gps_lat = Latitude(Coord(0.8672530930250163));
        cm.sensor.gps_state = GpsState::HeadingAvail;
        cm.sensor.gps_ground_speed = 123.4.kt();
        cm.sensor.gps_track = 321.4_f32.deg();

        let s = nh.nmea_gprmc(&mut cm);
        assert_eq!(
            s,
            b"$GPRMC,120520.00,A,4941.39652,N,835.06958,E,123.4,321.4,230623,,,A*53\r\n"
        );
    }

    #[test]
    fn hchdt() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor.euler_yaw = 123.4_f32.deg();
        let s = nh.nmea_hchdt(&mut cm);
        assert_eq!(s, b"$HCHDT,123.4,T*2D\r\n");
    }

    #[test]
    fn plara() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor.euler_roll = 123.4_f32.deg();
        cm.sensor.euler_nick = 98.7_f32.deg();
        cm.sensor.euler_yaw = 12.3_f32.deg();
        let s = nh.nmea_plara(&mut cm);
        assert_eq!(s, b"$PLARA,123.4,98.7,12.3*4E\r\n");
    }

    #[test]
    fn plarb() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.device.supply_voltage = 13.12;
        let s = nh.nmea_plarb(&mut cm);
        assert_eq!(s, b"$PLARB,13.12*4E\r\n");
    }

    #[test]
    fn plard() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor.density = 922.54.g_m3();
        let s = nh.nmea_plard(&mut cm);
        assert_eq!(s, b"$PLARD,922.54,M*10\r\n");
    }

    #[test]
    fn plars() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.config.mc_cready = 1.7.m_s();
        cm.config.glider_idx = 105;
        cm.glider_data.empty_weight = 295.0.kg();
        cm.glider_data.pilot_weight = 90.0.kg();
        cm.glider_data.water_ballast = 100.0.kg();
        cm.glider_data.bugs = 1.23;
        cm.sensor.pressure_altitude.set_qnh(1031.37.hpa());

        let s = nh.nmea_plars(&mut cm, crate::SysConfigId::MacCready);
        assert_eq!(s.unwrap(), b"$PLARS,L,MC,1.7*1A\r\n");

        let s = nh.nmea_plars(&mut cm, crate::SysConfigId::WaterBallast);
        assert_eq!(s.unwrap(), b"$PLARS,L,BAL,1.26*68\r\n");

        let s = nh.nmea_plars(&mut cm, crate::SysConfigId::Bugs);
        assert_eq!(s.unwrap(), b"$PLARS,L,BUGS,23*3E\r\n");

        let s = nh.nmea_plars(&mut cm, crate::SysConfigId::Qnh);
        assert_eq!(s.unwrap(), b"$PLARS,L,QNH,1031.4*72\r\n");
    }

    #[test]
    fn plarv() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor.climb_rate = 2.50.m_s();
        cm.sensor.average_climb_rate = 1.25.m_s();
        cm.sensor
            .pressure_altitude
            .set_static_pressure(97_717.0_f32.n_m2());
        cm.sensor.airspeed = AirSpeed::from_tas_at_nn(111.1.km_h());
        let s = nh.nmea_plarv(&mut cm);
        assert_eq!(s, b"$PLARV,2.50,1.25,305,111*5F\r\n");
    }

    #[test]
    fn plarw() {
        let mut cm = core_model();
        let mut nh = NmeaHandler::default();
        cm.sensor.average_wind = WindVector::new(45.6.km_h(), 321.0_f32.deg());
        let s = nh.nmea_plarw(&mut cm, true);
        assert_eq!(s, b"$PLARW,321,46,A,A*6A\r\n");

        cm.sensor.wind_vector = WindVector::new(45.6.km_h(), 321.0_f32.deg());
        let s = nh.nmea_plarw(&mut cm, false);
        assert_eq!(s, b"$PLARW,321,46,I,A*62\r\n");
    }

    fn core_model() -> CoreModel {
        CoreModel::new(1234_u32, HW_VERSION, SW_VERSION)
    }
}

pub struct NmeaTxBuffer {
    buf: [u8; 82],
    idx: usize,
}

impl NmeaTxBuffer {
    pub const fn new() -> Self {
        NmeaTxBuffer {
            buf: [0; 82],
            idx: 0,
        }
    }

    fn reset(&mut self) {
        self.idx = 0;
    }

    fn finish(&mut self) -> &[u8] {
        let mut cs = 0_u8;
        for b in &self.buf[1..self.idx] {
            cs ^= b;
        }
        let _ = uwrite!(self, "*{:02X}\r\n", cs);
        &self.buf[..self.idx]
    }
}

impl uWrite for NmeaTxBuffer {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let start = self.idx;

        // Silently ignore errors
        if let Some(buf) = self.buf.get_mut(start..start + len) {
            buf.copy_from_slice(bytes);
            self.idx += len;
        }

        Ok(())
    }
}
