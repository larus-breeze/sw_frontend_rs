use tfmt::{uWrite, uwrite};

use crate::{CoreModel, SysConfigId};

use super::GpsState;

impl CoreModel {
    pub fn nmea_config(&mut self, id: SysConfigId) {
        // no error if deque is full
        match id {
            SysConfigId::Bugs | SysConfigId::MacCready | SysConfigId::WaterBallast | SysConfigId::Qnh => {
                let _ = self.control.nmea.pers_id.push_back(id);
            }
            _ => (),
        }
        
    }

    pub fn nmea_cyclic(&mut self, short: bool) {
        if short {
            self.control.nmea.readout_idx = 106;
        } else {
            self.control.nmea.readout_idx = 100;
        }
    }

    pub fn nmea_next(&mut self) -> Option<&[u8]> {
        if  self.control.nmea.pers_id.len() > 0 {
            let id = self.control.nmea.pers_id.pop_front().unwrap();
            return self.nmea_plars(id);
        }
        self.control.nmea.readout_idx += 1;
        match self.control.nmea.readout_idx {
            // rarely sent
            101 => Some(self.nmea_gprmc()),
            102 => Some(self.nmea_gpgga()),
            103 => Some(self.nmea_hchdt()),
            104 => Some(self.nmea_plarw(true)),
            105 => Some(self.nmea_plard()),
            106 => Some(self.nmea_plarb()),

            // often sent
            107 => Some(self.nmea_plarw(false)),
            108 => Some(self.nmea_plara()),
            109 => Some(self.nmea_plarv()),
            _ => None,
        }
    }

    fn nmea_gprmc(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$GPRMC,{:n},A,{:n},{:n},{:.1},{:.1},{:n},,,A",
            self.sensor.gps_date_time.time(),
            self.sensor.gps_lat,
            self.sensor.gps_lon,
            self.sensor.gps_ground_speed.to_kt(),
            self.sensor.gps_track.to_degrees(),
            self.sensor.gps_date_time.date(),
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_gpgga(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let gps_quality_indicator = match self.sensor.gps_state {
            GpsState::PosAvail => 1,
            GpsState::HeadingAvail => 2,
            _ => 0,
        };
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$GPGGA,{:n},{:n},{:n},{},{},1.0,{:.1},M,{:.1},M,,",
            self.sensor.gps_date_time.time(),
            self.sensor.gps_lat,
            self.sensor.gps_lon,
            gps_quality_indicator,
            self.sensor.gps_sats,
            self.sensor.gps_altitude.to_m(),
            self.sensor.gps_geo_seperation.to_m(),
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_hchdt(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$HCHDT,{:.1},T",
            self.sensor.euler_yaw.to_degrees(),
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_plara(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$PLARA,{:.1},{:.1},{:.1}",
            self.sensor.euler_roll.to_degrees(),
            self.sensor.euler_nick.to_degrees(),
            self.sensor.euler_yaw.to_degrees(),
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_plarb(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$PLARB,{:.2}",
            self.device.supply_voltage,
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_plard(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$PLARD,{:.2},M",
            self.sensor.density.to_g_m3(),
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_plars(&mut self, id: SysConfigId) -> Option<&[u8]> {
        self.control.nmea.tx_data.reset();
        let _ = match id {
            SysConfigId::MacCready => uwrite!(
                self.control.nmea.tx_data,
                "$PLARS,L,MC,{:.1}",
                self.config.mc_cready.to_m_s()
            ),
            SysConfigId::WaterBallast => uwrite!(
                self.control.nmea.tx_data,
                "$PLARS,L,BAL,{:.2}",
                self.glider_data.ballast_ratio(),
            ),
            SysConfigId::Bugs => uwrite!(
                self.control.nmea.tx_data,
                "$PLARS,L,BUGS,{:.0}",
                (self.glider_data.bugs - 1.0) * 100.0
            ),
            SysConfigId::Qnh => uwrite!(
                self.control.nmea.tx_data,
                "$PLARS,L,QNH,{:.1}",
                self.sensor.pressure_altitude.qnh().to_hpa()
            ),
            _ => return None,
        };
        Some(self.control.nmea.tx_data.finish())
    }

    fn nmea_plarv(&mut self) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$PLARV,{:.2},{:.2},{:.0},{:.0}",
            self.sensor.climb_rate.to_m_s(),
            self.sensor.average_climb_rate.to_m_s(),
            self.sensor.pressure_altitude.qne_altitude().to_m(),
            self.sensor.airspeed.tas().to_km_h(),
        );
        self.control.nmea.tx_data.finish()
    }

    fn nmea_plarw(&mut self, average: bool) -> &[u8] {
        self.control.nmea.tx_data.reset();
        let (kind, wind) = if average {
            ("A", self.sensor.average_wind)
        } else {
            ("I", self.sensor.wind_vector)
        };
        let _ = uwrite!(
            self.control.nmea.tx_data,
            "$PLARW,{:.0},{:.0},{},A",
            wind.angle().to_degrees(),
            wind.speed().to_km_h(),
            kind,
        );
        self.control.nmea.tx_data.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic_config::MAX_TX_FRAMES, model::GpsState, AirSpeed, Coord, CoreModel, FloatToDensity,
        FloatToLength, FloatToMass, FloatToPressure, FloatToSpeed, HwVersion, Latitude, Longitude,
        QIdleEvents, QTxFrames, SwVersion, WindVector,
    };
    use embedded_graphics::geometry::AngleUnit;
    use heapless::spsc::Queue;

    const HW_VERSION: HwVersion = HwVersion::from_bytes([1, 3, 1, 0]);
    const SW_VERSION: SwVersion = SwVersion {
        version: [0, 0, 0, 0],
    };

    #[test]
    fn gpgga() {
        let mut cm = core_model();
        cm.sensor
            .gps_date_time
            .set_date_time(2023, 06, 23, 12, 05, 20);
        cm.sensor.gps_lon = Longitude(Coord(-0.1498276674644056));
        cm.sensor.gps_lat = Latitude(Coord(-0.8672530930250163));
        cm.sensor.gps_state = GpsState::HeadingAvail;
        cm.sensor.gps_sats = 23;
        cm.sensor.gps_altitude = 2745.9.m();
        cm.sensor.gps_geo_seperation = 12.3.m();

        let s = cm.nmea_gpgga();
        assert_eq!(
            s,
            b"$GPGGA,120520.00,4941.39652,S,835.06958,W,2,23,1.0,2745.9,M,12.3,M,,*56\r\n"
        );
    }

    #[test]
    fn gprmc() {
        let mut cm = core_model();
        cm.sensor
            .gps_date_time
            .set_date_time(2023, 06, 23, 12, 05, 20);
        cm.sensor.gps_lon = Longitude(Coord(0.1498276674644056));
        cm.sensor.gps_lat = Latitude(Coord(0.8672530930250163));
        cm.sensor.gps_state = GpsState::HeadingAvail;
        cm.sensor.gps_ground_speed = 123.4.kt();
        cm.sensor.gps_track = 321.4_f32.deg();

        let s = cm.nmea_gprmc();
        assert_eq!(
            s,
            b"$GPRMC,120520.00,A,4941.39652,N,835.06958,E,123.4,321.4,230623,,,A*53\r\n"
        );
    }

    #[test]
    fn hchdt() {
        let mut cm = core_model();
        cm.sensor.euler_yaw = 123.4_f32.deg();
        let s = cm.nmea_hchdt();
        assert_eq!(s, b"$HCHDT,123.4,T*2D\r\n");
    }

    #[test]
    fn plara() {
        let mut cm = core_model();
        cm.sensor.euler_roll = 123.4_f32.deg();
        cm.sensor.euler_nick = 98.7_f32.deg();
        cm.sensor.euler_yaw = 12.3_f32.deg();
        let s = cm.nmea_plara();
        assert_eq!(s, b"$PLARA,123.4,98.7,12.3*4E\r\n");
    }

    #[test]
    fn plarb() {
        let mut cm = core_model();
        cm.device.supply_voltage = 13.12;
        let s = cm.nmea_plarb();
        assert_eq!(s, b"$PLARB,13.12*4E\r\n");
    }

    #[test]
    fn plard() {
        let mut cm = core_model();
        cm.sensor.density = 922.54.g_m3();
        let s = cm.nmea_plard();
        assert_eq!(s, b"$PLARD,922.54,M*10\r\n");
    }

    #[test]
    fn plars() {
        let mut cm = core_model();
        cm.config.mc_cready = 1.7.m_s();
        cm.config.glider_idx = 105;
        cm.glider_data.empty_weight = 295.0.kg();
        cm.glider_data.pilot_weight = 90.0.kg();
        cm.glider_data.water_ballast = 100.0.kg();
        cm.glider_data.bugs = 1.23;
        cm.sensor.pressure_altitude.set_qnh(1031.37.hpa());

        let s = cm.nmea_plars(crate::SysConfigId::MacCready);
        assert_eq!(s.unwrap(), b"$PLARS,L,MC,1.7*1A\r\n");

        let s = cm.nmea_plars(crate::SysConfigId::WaterBallast);
        assert_eq!(s.unwrap(), b"$PLARS,L,BAL,1.26*68\r\n");

        let s = cm.nmea_plars(crate::SysConfigId::Bugs);
        assert_eq!(s.unwrap(), b"$PLARS,L,BUGS,23*3E\r\n");

        let s = cm.nmea_plars(crate::SysConfigId::Qnh);
        assert_eq!(s.unwrap(), b"$PLARS,L,QNH,1031.4*72\r\n");
    }

    #[test]
    fn plarv() {
        let mut cm = core_model();
        cm.sensor.climb_rate = 2.50.m_s();
        cm.sensor.average_climb_rate = 1.25.m_s();
        cm.sensor
            .pressure_altitude
            .set_static_pressure(97_717.0_f32.n_m2());
        cm.sensor.airspeed = AirSpeed::from_tas_at_nn(111.1.km_h());
        let s = cm.nmea_plarv();
        assert_eq!(s, b"$PLARV,2.50,1.25,305,111*5F\r\n");
    }

    #[test]
    fn plarw() {
        let mut cm = core_model();
        cm.sensor.average_wind = WindVector::new(45.6.km_h(), 321.0_f32.deg());
        let s = cm.nmea_plarw(true);
        assert_eq!(s, b"$PLARW,321,46,A,A*6A\r\n");

        cm.sensor.wind_vector = WindVector::new(45.6.km_h(), 321.0_f32.deg());
        let s = cm.nmea_plarw(false);
        assert_eq!(s, b"$PLARW,321,46,I,A*62\r\n");
    }

    fn core_model() -> CoreModel {
        let (p_tx_frames, _c_tx_frames) = {
            static mut Q_TX_FRAMES: QTxFrames<MAX_TX_FRAMES> = Queue::new();
            // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
            unsafe { Q_TX_FRAMES.split() }
        };

        // This queue routes the StorageItems from the controller to the idle loop.
        let (p_idle_events, _c_idle_events) = {
            static mut Q_IDLE_EVENTS: QIdleEvents = Queue::new();
            // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
            unsafe { Q_IDLE_EVENTS.split() }
        };

        CoreModel::new(p_idle_events, p_tx_frames, 1234_u32, HW_VERSION, SW_VERSION)
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
