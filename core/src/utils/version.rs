use core::cmp::Ordering;
use heapless::String;
use crate::utils::Concat;

pub const SW_VERSION: SwVersion = SwVersion { version: [0, 1, 0, 0]};
pub const HW_VERSION: HwVersion = HwVersion { version: [3, 0, 0, 0]};

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub struct HwVersion {
    version: [u8; 4],
}

impl HwVersion {
    pub fn major(&self) -> u8 { self.version[0] }
    pub fn minor(&self) -> u8 { self.version[1] }
    pub fn patch(&self) -> u8 { self.version[2] }

    pub fn is_compatible(&self, other: &HwVersion) -> bool {
        if (self.major() == other.major()) & (self.minor() == other.minor()) {
            true
        } else {
            false
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub struct SwVersion {
    version: [u8; 4], 
}

impl SwVersion {
    pub fn major(&self) -> u8 { self.version[0] }
    pub fn minor(&self) -> u8 { self.version[1] }
    pub fn patch(&self) -> u8 { self.version[2] }
    pub fn build_index(&self) -> u8 { self.version[3] }

    pub fn as_string(&self) -> String<20> {
        Concat::<20>::default()
            .push_str("Version ")
            .push_u8(self.version[0]).push_str(".")
            .push_u8(self.version[1]).push_str(".")
            .push_u8(self.version[2]).push_str("_")
            .push_u8(self.version[3])
            .as_string()
    }
}

impl PartialOrd for SwVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_u32 = u32::from_be_bytes(self.version);
        let other_u32 = u32::from_be_bytes(other.version);
        Some(*&self_u32.cmp(&other_u32))
    }
}

impl Default for SwVersion {
    fn default() -> Self {
        SW_VERSION
    }
}

