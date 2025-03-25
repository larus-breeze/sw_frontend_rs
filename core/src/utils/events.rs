use crate::utils::SwVersion;
use crate::PinState;

pub enum Event {
    KeyItem(KeyEvent),
    DeviceItem(DeviceEvent),
    InputItem(InputPinState),
}

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

#[derive(Clone, Copy, PartialEq)]
pub enum DeviceEvent {
    FwAvailable(SwVersion),
    PrepareFwUpload,
    UploadInProgress,
    UploadFinished,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputPinState {
    Io1(PinState),
    Io2(PinState),
    Io3(PinState),
    Io4(PinState),
}