use crate::{HwVersion, SwVersion};

#[repr(C)]
pub struct MetaDataV1 {
    pub magic: u64,
    pub crc: u32,
    pub meta_version: u32,
    pub storage_addr: usize,
    pub hw_version: HwVersion,
    pub sw_version: SwVersion,
    pub copy_func: usize,
    pub new_app: usize,
    pub new_app_len: usize,
    pub new_app_dest: usize,
}

impl Default for MetaDataV1 {
    fn default() -> Self {
        MetaDataV1 {
            magic: 0x1c80_73ab_2085_3579,
            crc: 0,
            meta_version: 1,
            storage_addr: 0,
            hw_version: HwVersion::default(),
            sw_version: SwVersion::current(),
            copy_func: 0,
            new_app: 0,
            new_app_len: 0,
            new_app_dest: 0,
        }
    }
}

impl MetaDataV1 {
    pub fn to_bytes(&self) -> &[u8; 44] {
        unsafe { core::mem::transmute(self) }
    }
}
