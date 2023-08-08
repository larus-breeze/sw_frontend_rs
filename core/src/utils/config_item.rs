use byteorder::{LittleEndian as LE, ByteOrder};

const MAGIC: &[u8; 8] = b"\x00\x00\x60\xb3\x5f\xf2\x6a\xa4";

#[allow(unused)]
#[repr(C, packed)]
#[derive(PartialEq)]
pub struct ConfigItem {
    config_id: u16,
    buf: [u8; 6]
}

#[allow(unused)]
impl ConfigItem {

    /// Create the magic config item, which is used to identify uninit memory
    pub fn magic() -> Self {
        ConfigItem::from_bytes(MAGIC)
    }

    // Compare item with magic item
    pub fn is_magic(other: &ConfigItem) -> bool {
        other == &Self::magic()
    }

    /// Create config item from id and byte array
    pub fn from_bytes(data: &[u8; 8]) -> Self {
        let config_id = LE::read_u16(&data[..2]);
        let mut buf = [0u8; 6];
        buf.copy_from_slice(&data[2..8]);
        ConfigItem {config_id, buf} 
    }

    /// Create config item from id and u32
    pub fn from_u32(config_id: u16, n: u32) -> Self {
        let mut buf = [0u8; 6];
        LE::write_u32(&mut buf[..4], n);
        ConfigItem { config_id, buf }
    }

    /// Create config item from id and u16
    pub fn from_u16(config_id: u16, n: u16) -> Self {
        let mut buf = [0u8; 6];
        LE::write_u16(&mut buf[..2], n);
        ConfigItem { config_id, buf }
    }

    /// Create config item from id and u8
    pub fn from_u8(config_id: u16, n: u8) -> Self {
        let mut buf = [0u8; 6];
        buf [0] = n;
        ConfigItem { config_id, buf }
    }

    /// Create config item from id and i32
    pub fn from_i32(config_id: u16, n: i32) -> Self {
        let mut buf = [0u8; 6];
        LE::write_i32(&mut buf[..4], n);
        ConfigItem { config_id, buf }
    }

    /// Create config item from id and i16
    pub fn from_i16(config_id: u16, n: i16) -> Self {
        let mut buf = [0u8; 6];
        LE::write_i16(&mut buf[..2], n);
        ConfigItem { config_id, buf }
    }

    /// Create config item from id and i8
    pub fn from_i8(config_id: u16, n: i8) -> Self {
        let mut buf = [0u8; 6];
        buf [0] = n as u8;
        ConfigItem { config_id, buf }
    }

    /// Create config item from id and f32
    pub fn from_f32(config_id: u16, n: f32) -> Self {
        let mut buf = [0u8; 6];
        LE::write_f32(&mut buf[..4], n);
        ConfigItem { config_id, buf }
    }

    /// Get config_id from config item
    pub fn config_id(&self) -> u16 {
        self.config_id
    }

    /// get u32 from config item
    pub fn as_u32(&self) -> u32 {
        LE::read_u32(&self.buf[..4])
    }

    /// get u16 from config item
    pub fn as_u16(&self) -> u16 {
        LE::read_u16(&self.buf[..2])
    }

    /// get u8 from config item
    pub fn as_u8(&self) -> u8 {
        self.buf[0]
    }

    /// get i32 from config item
    pub fn as_i32(&self) -> i32 {
        LE::read_i32(&self.buf[..4])
    }

    /// get i16 from config item
    pub fn as_i16(&self) -> i16 {
        LE::read_i16(&self.buf[..2])
    }

    /// get i8 from config item
    pub fn as_i8(&self) -> i8 {
        self.buf[0] as i8
    }

    /// get f32 from config item
    pub fn as_f32(&self) -> f32 {
        LE::read_f32(&self.buf[..4])
    }

    /// get byte array from config item
    pub fn as_bytes(&self) -> &[u8; 8] {
        // unsafe is ok here, becaus len and memory layout is fixed
        unsafe {core::mem::transmute::<&Self, &[u8; 8]>(self)}
    }

}