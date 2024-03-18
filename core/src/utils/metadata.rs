use crate::{HwVersion, SwVersion};

#[repr(C)]
pub struct MetaDataV1 {
    pub magic: u64,
    pub crc: u32,
    pub meta_version: u32,
    pub storage_addr: u32,
    pub hw_version: HwVersion,
    pub sw_version: SwVersion,
    pub copy_func: u32,
    pub new_app: u32,
    pub new_app_len: u32,
    pub new_app_dest: u32,
}

impl Default for MetaDataV1 {
    fn default() -> Self {
        MetaDataV1 {
            magic: 0x1c80_73ab_2085_3579,
            crc: 0,
            meta_version: 1,
            storage_addr: 0,
            hw_version: HwVersion {
                version: [0, 0, 0, 0],
            },
            sw_version: SwVersion {
                version: [0, 0, 0, 0],
            },
            copy_func: 0,
            new_app: 0,
            new_app_len: 0,
            new_app_dest: 0,
        }
    }
}

impl MetaDataV1 {
    pub fn to_bytes(&self) -> &[u8; SIZE_METADATA_V1] {
        unsafe { core::mem::transmute(self) }
    }
}

pub const SIZE_METADATA_V1: usize = core::mem::size_of::<MetaDataV1>();
