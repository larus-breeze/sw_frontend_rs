use crate::{
    controller::Echo, model::GpsState, utils::ParseSlice, CoreController, CoreError, CoreModel,
    FloatToPressure, FloatToSpeed, PersistenceId,
};
use heapless::Vec;
use tfmt::uwrite;

impl CoreController {
    pub fn recv_u8(&mut self, cm: &mut CoreModel, b: u8) {
        if self.nmea_buffer.rx.recv_u8(b) {
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
        self.nmea_buffer.rx.check()?;

        self.nmea_buffer.rx.compare_chunk(b"$PLARS")?;
        self.nmea_buffer.rx.compare_chunk(b"H")?;

        let cmd: Vec<u8, 10> = Vec::from_slice(self.nmea_buffer.rx.next_chunk()?)
            .map_err(|_| CoreError::ParseError)?;

        let s = self.nmea_buffer.rx.next_chunk()?;
        let val = f32::from_slice(s)?;

        match cmd.as_slice() {
            b"MC" => {
                let val = in_range(val, 0.0, 9.9)?.m_s();
                self.persist_set_maccready(cm, val, Echo::Can)
            }
            b"BAL" => {
                cm.glider_data
                    .set_ballast_fraction(in_range(val, 0.00, 1.00)?);
                let val = cm.glider_data.water_ballast;
                self.persist_set_water_ballast(cm, val, Echo::Can);
            }
            b"BUGS" => {
                let val = 1.0 + in_range(val, 0.0, 50.0)? / 100.0;
                self.persist_set_bugs(cm, val, Echo::Can);
            }
            b"QNH" => {
                let val = in_range(val, 900.0, 1100.0)?.hpa();
                self.persist_set_pilot_qnh(cm, val, Echo::Can);
            }
            _ => return Err(CoreError::ParseError),
        }
        Ok(())
    }

    pub fn nmea_config(&mut self, id: PersistenceId) {
        // no error if deque is full
        match id {
            PersistenceId::Bugs
            | PersistenceId::McCready
            | PersistenceId::WaterBallast
            | PersistenceId::Qnh => {
                let _ = self.nmea_buffer.pers_id.push_back(id);
            }
            _ => (),
        }
    }

    pub fn nmea_cyclic_1s(&mut self) {
        let _ = self
            .nmea_buffer
            .to_send
            .extend_from_slice(&[0, 1, 2, 3, 4, 5]);
    }

    pub fn nmea_cyclic_200ms(&mut self) {
        let _ = self.nmea_buffer.to_send.extend_from_slice(&[6, 7, 8]);
    }

    pub fn nmea_next(&mut self, cm: &mut CoreModel) -> Option<&[u8]> {
        if self.nmea_buffer.pers_id.len() > 0 {
            let id = self.nmea_buffer.pers_id.pop_front().unwrap();
            return self.nmea_plars(cm, id);
        }
        if let Some(idx) = self.nmea_buffer.to_send.pop() {
            match idx {
                // rarely sent
                0 => Some(self.nmea_gprmc(cm)),
                1 => Some(self.nmea_gpgga(cm)),
                2 => Some(self.nmea_hchdt(cm)),
                3 => Some(self.nmea_plarw(cm, true)),
                4 => Some(self.nmea_plard(cm)),
                5 => Some(self.nmea_plarb(cm)),

                // often sent
                6 => Some(self.nmea_plarw(cm, false)),
                7 => Some(self.nmea_plara(cm)),
                8 => Some(self.nmea_plarv(cm)),

                // not known
                _ => None,
            }
        } else {
            None
        }
    }

    fn nmea_gprmc(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$GPRMC,{:n},A,{:n},{:n},{:.1},{:.1},{:n},,,A",
            cm.sensor.gps_date_time.time(),
            cm.sensor.gps_lat,
            cm.sensor.gps_lon,
            cm.sensor.gps_ground_speed.to_kt(),
            cm.sensor.gps_track.to_degrees(),
            cm.sensor.gps_date_time.date(),
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_gpgga(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let gps_quality_indicator = match cm.sensor.gps_state {
            GpsState::PosAvail => 1,
            GpsState::HeadingAvail => 2,
            _ => 0,
        };
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$GPGGA,{:n},{:n},{:n},{},{},1.0,{:.1},M,{:.1},M,,",
            cm.sensor.gps_date_time.time(),
            cm.sensor.gps_lat,
            cm.sensor.gps_lon,
            gps_quality_indicator,
            cm.sensor.gps_sats,
            cm.sensor.gps_altitude.to_m(),
            cm.sensor.gps_geo_seperation.to_m(),
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_hchdt(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$HCHDT,{:.1},T",
            cm.sensor.euler_yaw.to_degrees(),
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_plara(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let mut roll = cm.sensor.euler_roll.to_degrees();
        if roll > 180.0 {
            roll -= 360.0
        }
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$PLARA,{:.1},{:.1},{:.1}",
            roll,
            cm.sensor.euler_nick.to_degrees(),
            cm.sensor.euler_yaw.to_degrees(),
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_plarb(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$PLARB,{:.2}",
            cm.device.supply_voltage,
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_plard(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$PLARD,{:.2},M",
            cm.sensor.density.to_g_m3(),
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_plars(&mut self, cm: &mut CoreModel, id: PersistenceId) -> Option<&[u8]> {
        self.nmea_buffer.tx.reset();
        let _ = match id {
            PersistenceId::McCready => uwrite!(
                self.nmea_buffer.tx,
                "$PLARS,L,MC,{:.1}",
                cm.config.mc_cready.to_m_s()
            ),
            PersistenceId::WaterBallast => uwrite!(
                self.nmea_buffer.tx,
                "$PLARS,L,BAL,{:.3}",
                cm.glider_data.ballast_fraction(),
            ),
            PersistenceId::Bugs => uwrite!(
                self.nmea_buffer.tx,
                "$PLARS,L,BUGS,{:.0}",
                (cm.glider_data.bugs - 1.0) * 100.0
            ),
            PersistenceId::Qnh => uwrite!(
                self.nmea_buffer.tx,
                "$PLARS,L,QNH,{:.1}",
                cm.sensor.pressure_altitude.qnh().to_hpa()
            ),
            _ => return None,
        };
        Some(self.nmea_buffer.tx.finish())
    }

    fn nmea_plarv(&mut self, cm: &mut CoreModel) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$PLARV,{:.2},{:.2},{:.0},{:.0}",
            cm.sensor.climb_rate.to_m_s(),
            cm.sensor.average_climb_rate.to_m_s(),
            cm.sensor.pressure_altitude.qne_altitude().to_m(),
            cm.sensor.airspeed.tas().to_km_h(),
        );
        self.nmea_buffer.tx.finish()
    }

    fn nmea_plarw(&mut self, cm: &mut CoreModel, average: bool) -> &[u8] {
        self.nmea_buffer.tx.reset();
        let (kind, wind) = if average {
            ("A", cm.sensor.average_wind)
        } else {
            ("I", cm.sensor.wind_vector)
        };
        let _ = uwrite!(
            self.nmea_buffer.tx,
            "$PLARW,{:.0},{:.0},{},A",
            wind.angle().to_degrees(),
            wind.speed().to_km_h(),
            kind,
        );
        self.nmea_buffer.tx.finish()
    }
}

pub fn nmea_cyclic_200ms(_cm: &mut CoreModel, cc: &mut CoreController) {
    cc.nmea_cyclic_200ms();
}
