use crate::{CoreError, PersistenceId};
use heapless::{Deque, Vec};
use tfmt::{uWrite, uwrite};

const HEX_TAB: &[u8; 16] = b"0123456789ABCDEF";

pub struct NmeaBuffer {
    pub rx: RxBuffer,
    pub tx: TxBuffer,
    pub to_send: Vec<u8, 10>,
    pub pers_id: Deque<PersistenceId, 16>,
}

impl NmeaBuffer {
    pub const fn new() -> Self {
        NmeaBuffer {
            rx: RxBuffer::new(),
            tx: TxBuffer::new(),
            to_send: Vec::new(),
            pers_id: Deque::new(),
        }
    }
}

enum RxState {
    WaitForStart,
    ReceiveData,
}

pub struct RxBuffer {
    buf: [u8; 82],
    idx: usize,
    chunk_idx: usize,
    state: RxState,
}

impl RxBuffer {
    pub const fn new() -> Self {
        RxBuffer {
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

pub struct TxBuffer {
    buf: [u8; 82],
    idx: usize,
}

impl TxBuffer {
    pub const fn new() -> Self {
        TxBuffer {
            buf: [0; 82],
            idx: 0,
        }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn finish(&mut self) -> &[u8] {
        let mut cs = 0_u8;
        for b in &self.buf[1..self.idx] {
            cs ^= b;
        }
        let _ = uwrite!(self, "*{:02X}\r\n", cs);
        &self.buf[..self.idx]
    }
}

impl uWrite for TxBuffer {
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
