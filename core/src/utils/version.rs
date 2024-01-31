use crate::utils::Concat;
use core::cmp::Ordering;
use heapless::String;

#[rustfmt::skip]
pub const SW_VERSION: SwVersion = SwVersion { version: [0, 1, 1, 9]};
#[rustfmt::skip]
pub const HW_VERSION: HwVersion = HwVersion { version: [3, 0, 0, 0]};

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub struct HwVersion {
    version: [u8; 4],
}

impl HwVersion {
    pub fn manufacturer(&self) -> u8 {
        self.version[0]
    }

    pub fn major(&self) -> u8 {
        self.version[1]
    }

    pub fn minor(&self) -> u8 {
        self.version[2]
    }

    pub fn patch(&self) -> u8 {
        self.version[3]
    }

    pub fn is_compatible(&self, other: &HwVersion) -> bool {
        (self.manufacturer() == other.manufacturer())
            & (self.major() == other.major())
            & (self.minor() == other.minor())
    }

    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        HwVersion { version: bytes }
    }
}

impl Default for HwVersion {
    fn default() -> Self {
        HwVersion {
            version: [0, 0, 0, 0],
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub struct SwVersion {
    version: [u8; 4],
}

impl SwVersion {
    pub fn major(&self) -> u8 {
        self.version[0]
    }
    pub fn minor(&self) -> u8 {
        self.version[1]
    }
    pub fn patch(&self) -> u8 {
        self.version[2]
    }
    pub fn build_index(&self) -> u8 {
        self.version[3]
    }

    pub fn as_string(&self) -> String<20> {
        Concat::<20>::default()
            .push_str("Version ")
            .push_u8(self.version[0])
            .push_str(".")
            .push_u8(self.version[1])
            .push_str(".")
            .push_u8(self.version[2])
            .push_str("_")
            .push_u8(self.version[3])
            .as_string()
    }
}

impl PartialOrd for SwVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_u32 = u32::from_be_bytes(self.version);
        let other_u32 = u32::from_be_bytes(other.version);
        Some(self_u32.cmp(&other_u32))
    }
}

impl SwVersion {
    pub const fn current() -> Self {
        SW_VERSION
    }
}

impl defmt::Format for SwVersion {
    fn format(&self, f: defmt::Formatter) {
        // format the bitfields of the register as struct fields
        defmt::write!(
            f,
            "SW {}.{}.{}_{}",
            self.version[0],
            self.version[1],
            self.version[2],
            self.version[3],
        )
    }
}

impl SwVersion {
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        SwVersion { version: bytes }
    }
}
