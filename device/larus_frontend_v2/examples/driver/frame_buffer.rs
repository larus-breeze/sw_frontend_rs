use core::{mem, mem::transmute, ptr::addr_of};
use corelib::basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use embedded_hal::can::Frame;
use stm32h7xx_hal::pac;
use super::{Display, Ltdc};

pub const AVAIL_PIXELS: usize = (DISPLAY_HEIGHT * DISPLAY_WIDTH) as usize;
pub type TBuffer = &'static mut [u8; AVAIL_PIXELS];

#[link_section = ".axisram.AXISRAM"]
pub static mut FRAME_BUFFER_1: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];

#[link_section = ".axisram.AXISRAM"]
pub static mut FRAME_BUFFER_2: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];

/// Two static buffers are used. One to supply the LCD and one to build up the next image. This 
/// always takes place alternately.
pub struct FrameBuffer {
    foreground: TBuffer,
    background: TBuffer,
    ltdc: Ltdc,
}

impl FrameBuffer {
    pub fn new(mut ltdc: Ltdc) -> Self {
        let foreground = unsafe {
            transmute::<*const [u8; AVAIL_PIXELS], TBuffer>(addr_of!(FRAME_BUFFER_1))
        };   
        let background = unsafe {
            transmute::<*const [u8; AVAIL_PIXELS], TBuffer>(addr_of!(FRAME_BUFFER_2))
        };
        for idx in 0..AVAIL_PIXELS {
            foreground[idx] = 7; // Black
            background[idx] = 7; // Black
        }

        ltdc.init_layer(background.as_ptr());
        FrameBuffer { foreground, background, ltdc }
    }

    /// Swap foreground and background and return the reference to the current foreground. At the 
    /// same time, the LTDC is informed to use the buffer for background.
    pub fn swap_buffers(&mut self) -> TBuffer {
        mem::swap(&mut self.foreground, &mut self.background);
        self.ltdc.set_frame_buffer(self.background.as_ptr());

        // The process ensures that the respective buffer is only used by one instance, so unsafe 
        // is ok here.
        unsafe { transmute::<&mut [u8; AVAIL_PIXELS], TBuffer>(self.foreground)}
    }
}

