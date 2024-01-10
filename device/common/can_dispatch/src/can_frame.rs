use byteorder::{ByteOrder, LittleEndian as LE};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Frame {
    Legacy(CanFrame),
    Specific(SpecificFrame),
    Generic(GenericFrame),
}

impl Frame {
    pub fn specific(can_frame: CanFrame, specific_id: u16, object_id: u16) -> Self {
        let specific_frame = SpecificFrame {
            can_frame,
            specific_id,
            object_id,
        };
        Frame::Specific(specific_frame)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SpecificFrame {
    pub can_frame: CanFrame,
    pub specific_id: u16,
    pub object_id: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct GenericFrame {
    pub can_frame: CanFrame,
    pub generic_id: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CanFrame {
    id: u16,
    rtr: bool,
    len: u8,
    data: [u8; 8],
}

#[allow(unused)]
impl CanFrame {
    pub fn empty_from_id(id: u16) -> Self {
        CanFrame {
            id,
            rtr: false,
            len: 0,
            data: [0u8; 8],
        }
    }

    pub fn remote_trans_rq(id: u16, len: u8) -> Self {
        CanFrame {
            id,
            rtr: true,
            len,
            data: [0u8; 8],
        }
    }

    pub fn from_slice(id: u16, src: &[u8]) -> Self {
        let mut data = [0u8; 8];
        let len = src.len();
        data[..len].copy_from_slice(&src[..len]);
        CanFrame {
            id,
            rtr: false,
            len: len as u8,
            data,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    pub fn is_heartbeat(&self) -> bool {
        (self.id >= 0x400) && ((self.id & 0x00f) == 0)
    }

    pub fn generic_id(&self) -> Option<u16> {
        if self.id >= 0x400 {
            Some(self.id & 0x00f)
        } else {
            None
        }
    }

    pub fn specific_id(&self) -> Option<u16> {
        if self.id < 0x400 {
            Some(self.id & 0x00f)
        } else {
            None
        }
    }

    pub fn is_rtr(&self) -> bool {
        self.rtr
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }

    pub fn dlc(&self) -> u8 {
        self.len
    }

    pub fn vda(&self) -> u16 {
        (self.id & 0x3f0) >> 4
    }

    pub fn read_u32(&self, idx: usize) -> u32 {
        LE::read_u32(&self.data[idx..idx + 4])
    }

    pub fn read_u16(&self, idx: usize) -> u16 {
        LE::read_u16(&self.data[idx..idx + 2])
    }

    pub fn read_u8(&self, idx: usize) -> u8 {
        self.data[idx]
    }

    pub fn read_i32(&self, idx: usize) -> i32 {
        LE::read_u32(&self.data[idx..idx + 4]) as i32
    }

    pub fn read_i16(&self, idx: usize) -> i16 {
        LE::read_u16(&self.data[idx..idx + 2]) as i16
    }

    pub fn read_i8(&self, idx: usize) -> i8 {
        self.data[idx] as i8
    }

    pub fn read_f32(&self, idx: usize) -> f32 {
        LE::read_f32(&self.data[idx..idx + 4])
    }

    pub fn push_u32(mut self, val: u32) -> Self {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_u32(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    pub fn push_u16(mut self, val: u16) -> Self {
        let idx = self.len as usize;
        self.len += 2;
        LE::write_u16(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    pub fn push_u8(mut self, val: u8) -> Self {
        self.data[self.len as usize] = val;
        self.len += 1;
        self
    }

    pub fn push_i32(mut self, val: i32) -> Self {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_i32(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    pub fn push_i16(mut self, val: i16) -> Self {
        let idx = self.len as usize;
        self.len += 2;
        LE::write_i16(&mut self.data[idx..(self.len as usize)], val);
        self
    }

    pub fn push_i8(mut self, val: i8) -> Self {
        self.data[self.len as usize] = val as u8;
        self.len += 1;
        self
    }

    pub fn push_f32(mut self, val: f32) -> Self {
        let idx = self.len as usize;
        self.len += 4;
        LE::write_f32(&mut self.data[idx..(self.len as usize)], val);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::CanFrame;

    #[test]
    fn is_heartbeat() {
        let can_frame = CanFrame::empty_from_id(0x400);
        assert_eq!(can_frame.is_heartbeat(), true);
        let can_frame = CanFrame::empty_from_id(0x610);
        assert_eq!(can_frame.is_heartbeat(), true);
        let can_frame = CanFrame::empty_from_id(0x611);
        assert_eq!(can_frame.is_heartbeat(), false);
        let can_frame = CanFrame::empty_from_id(0x300);
        assert_eq!(can_frame.is_heartbeat(), false);
    }

    #[test]
    fn generic_id() {
        let can_frame = CanFrame::empty_from_id(0x400);
        assert_eq!(can_frame.generic_id(), Some(0));
        let can_frame = CanFrame::empty_from_id(0x613);
        assert_eq!(can_frame.generic_id(), Some(3));
        let can_frame = CanFrame::empty_from_id(0x61f);
        assert_eq!(can_frame.generic_id(), Some(15));
        let can_frame = CanFrame::empty_from_id(0x3ff);
        assert_eq!(can_frame.generic_id(), None);
        let can_frame = CanFrame::empty_from_id(0x123);
        assert_eq!(can_frame.generic_id(), None);
    }

    #[test]
    fn specific_id() {
        let can_frame = CanFrame::empty_from_id(0x400);
        assert_eq!(can_frame.specific_id(), None);
        let can_frame = CanFrame::empty_from_id(0x613);
        assert_eq!(can_frame.specific_id(), None);
        let can_frame = CanFrame::empty_from_id(0x31f);
        assert_eq!(can_frame.specific_id(), Some(15));
        let can_frame = CanFrame::empty_from_id(0x3ff);
        assert_eq!(can_frame.specific_id(), Some(15));
        let can_frame = CanFrame::empty_from_id(0x123);
        assert_eq!(can_frame.specific_id(), Some(3));
    }
}
