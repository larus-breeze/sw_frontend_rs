#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum KeyEvent {
    NoEvent,
    Btn1,
    Btn2,
    Btn3,
    BtnEsc,
    BtnEnc,
    Btn12,
    Btn23,
    Btn3Esc,
    Btn1Esc,
    Btn1EscEnc,
    Btn1S3,
    Btn2S3,
    Btn3S3,
    BtnEscS3,
    BtnEncS3,
    Btn12S3,
    Btn23S3,
    Btn3EscS3,
    Btn1EscS3,
    Btn1EscEncS3,
    Rotary1Left,
    Rotary1Right,
    Rotary2Left,
    Rotary2Right,
}


use core::cmp::Ordering;
use heapless::String;
use crate::utils::Concat;

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

pub const SW_VERSION: SwVersion = SwVersion { version: [0, 1, 0, 0]};

#[derive(Clone, Copy, PartialEq)]
pub enum DeviceEvent {
    FwAvailable(SwVersion),
    PrepareFwUpload,
    UploadInProgress,
    UploadFinished,
}