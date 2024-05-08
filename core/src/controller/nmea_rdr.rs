use crate::CoreError;

/*pub struct NmeaHandler {
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
}*/

const HEX_TAB: &[u8; 16] = b"0123456789ABCDEF";

enum RxState {
    WaitForStart,
    ReceiveData,
}

pub struct NmeaRxBuffer {
    buf: [u8; 82],
    idx: usize,
    chunk_idx: usize,
    state: RxState,
}

impl NmeaRxBuffer {
    pub const fn new() -> Self {
        NmeaRxBuffer {
            buf: [0; 82],
            idx: 0,
            chunk_idx: 0,
            state: RxState::WaitForStart,
        }
    }

    pub fn recv_u8(&mut self, b: u8) -> bool {
        match self.state {
            RxState::WaitForStart => {
                if b == b'$' {
                    // nmea start sign detected
                    self.state = RxState::ReceiveData;
                    self.buf[0] = b;
                    self.idx = 1;
                }
            }
            RxState::ReceiveData => {
                if self.idx >= 82 {
                    // no valid nmea data received, data will be lost
                    self.state = RxState::WaitForStart;
                } else {
                    self.buf[self.idx] = b;
                    self.idx += 1;
                    if b == b'\r' {
                        // be ready for new data
                        self.state = RxState::WaitForStart;
                        // end of nmea string reached, parse data now
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn compare_chunk(&mut self, s: &[u8]) -> Result<(), CoreError> {
        let slice = self.next_chunk()?;
        if slice == s {
            return Ok(());
        } else {
            return Err(CoreError::ParseError);
        }
    }

    pub fn next_chunk(&mut self) -> Result<&[u8], CoreError> {
        let mut idx = self.chunk_idx;
        while idx < self.idx {
            let b = self.buf[idx];
            if b == b',' || b == b'*' {
                let r_idx = self.chunk_idx;
                self.chunk_idx = idx + 1;
                return Ok(&self.buf[r_idx..idx]);
            }
            idx += 1;
        }
        Err(CoreError::ParseError)
    }

    pub fn check(&mut self) -> Result<(), CoreError> {
        let mut cs = 0;
        let mut end_deteced = false;
        let mut idx = 1;
        while idx < self.idx && idx < 80 {
            let b = self.buf[idx];
            idx += 1;
            if b == b'*' {
                end_deteced = true;
                break;
            }
            cs ^= b;
        }
        if !end_deteced {
            // datagram isn't complete
            return Err(CoreError::ParseError);
        }
        if HEX_TAB[(cs & 0x0f) as usize] != self.buf[idx + 1]
            && HEX_TAB[(cs >> 4) as usize] != self.buf[idx]
        {
            // checksum is not correct
            return Err(CoreError::ParseError);
        }
        self.chunk_idx = 0;
        Ok(())
    }
}

