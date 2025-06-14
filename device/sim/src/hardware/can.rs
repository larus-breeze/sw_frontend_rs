use corelib::*;
use std::{net::UdpSocket, time::Duration};
use byteorder::{ByteOrder, LittleEndian as LE};


pub struct CanReader {
    socket: UdpSocket,
    buf: [u8; 10],
}

impl CanReader {
    pub fn new(addr: &str) -> Self {
        let socket = UdpSocket::bind(addr)
            .expect("Could not open socket");
        socket.set_read_timeout(Some(Duration::from_millis(10)))
            .expect("Could not set read timeout");
        CanReader { socket, buf: [0; 10] }
    }

    pub fn read(&mut self) -> Option<Frame> {
        if let Ok((cnt, _adr)) = self.socket.recv_from(&mut self.buf) {
            let id = LE::read_u16(&self.buf[..2]);
            let can_frame = CanFrame::from_slice(id, &self.buf[2..cnt]);
            let frame = if id >= 0x120 && id <= 0x12f {
                Frame::Specific(SpecificFrame {
                    can_frame,
                    specific_id: id & 0x0f,
                    object_id: 2, // Sensorbox
                })
            } else if id >= 0x140 && id <= 0x14f {
                Frame::Specific(SpecificFrame {
                    can_frame,
                    specific_id: id & 0x0f,
                    object_id: 3, // GPS
                })
            } else {
                Frame::Legacy(can_frame)
            };
            Some(frame)
        } else {
            None
        }
    }
}
