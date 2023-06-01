use core::{
    convert::TryInto,
    sync::atomic::AtomicBool,
};
use embedded_graphics::{
    prelude::*,
};
use crate::{Colors, RGB565_COLORS};
use crate::driver::r61580::{
        R61580,
        AVAIL_PIXELS,
        PORTRAIT_AVAIL_HEIGHT,
        PORTRAIT_AVAIL_WIDTH,
};

const PORT_AVAIL_HEI_M1: u32 = PORTRAIT_AVAIL_HEIGHT as u32 - 1;
const PORT_AVAIL_WID_M1: u32 = PORTRAIT_AVAIL_WIDTH as u32 - 1;


use embedded_hal::digital::v2::OutputPin;

use crate::utils::Error;


/// A note aboute the safety of FrameBuffer:
/// 
/// The display driver and the DMA copy routine must both have access to the FRAME_BUFFER and DMA_FINISHED variables. 
/// While the display driver needs write access to the buffer, the DMA copy routine reads from it. According to plan 
/// these processes run staggered. First is written and then read. If this time offset is not fulfilled, this is not 
/// a catastrophe, because nothing crashes. There is just some flickering. However, the DMA_FINISHED variable is
/// intended to detect such an overlap.
/// 
/// How the Flag DMA_FINSHED is used:
/// After the display driver has successfully written to the buffer, the flag is set to false and a DMA interrupt is 
/// triggered. The interrupt service routine recognizes by the pattern flag false and DMA empty that it must restart 
/// the DMA process. After the complete copying of the data the DMA interrupt service routine sets the flag to true. 
/// Before the next buffer write, this flag can be used to assess whether the copy process was completed successfully.

#[allow(dead_code)]
pub struct FrameBuffer {
    pub buf: &'static mut [u8; AVAIL_PIXELS],
    pub dma_finished: &'static mut AtomicBool,
}

impl FrameBuffer {
    pub fn new() -> Self {
        static mut FRAME_BUFFER: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];
        static mut DMA_FINISHED: AtomicBool = AtomicBool::new(false);

        let buf = unsafe {&mut FRAME_BUFFER};
        let dma_finished = unsafe{&mut DMA_FINISHED};

        FrameBuffer { buf: buf, dma_finished: dma_finished }
    }
}

impl FrameBuffer {
    pub fn split(&self) -> FrameBuffer {
        let raw = self.buf as *const [u8; AVAIL_PIXELS];
        let buf2 = unsafe { core::mem::transmute::<*const [u8; AVAIL_PIXELS], &mut [u8; AVAIL_PIXELS]>(raw) };
        let raw = self.dma_finished as *const AtomicBool;
        let dma2 = unsafe { core::mem::transmute::<*const AtomicBool, &mut AtomicBool>(raw) };
        FrameBuffer { buf: buf2, dma_finished: dma2 }
    }
}

#[allow(unused)]
pub struct Display<RST, PinE>
where
    RST: OutputPin<Error = PinE>,
{
    buf: &'static mut [u8; AVAIL_PIXELS],
    dma_finished: &'static mut AtomicBool,
    lcd: R61580<RST>
}

impl<RST, PinE> Display<RST, PinE>
where
    RST: OutputPin<Error = PinE>,
{
    pub fn new(lcd: R61580<RST>, fb: FrameBuffer)  -> Self
    where
        RST: OutputPin<Error = PinE>,
    {
        Display { buf: fb.buf, dma_finished: fb.dma_finished, lcd}
    }

    /// Updates the display from the Display.
    #[allow(unused)]
    pub fn flush(&mut self) -> Result<(), Error<PinE>> {
        let mut row: [u16; PORTRAIT_AVAIL_HEIGHT as usize] = [0; PORTRAIT_AVAIL_HEIGHT as usize];
        for y in 0..PORTRAIT_AVAIL_HEIGHT as usize {
            for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                row[x] = RGB565_COLORS[self.buf[y*PORTRAIT_AVAIL_WIDTH as usize+ x] as usize];
            }
            self.lcd.set_pixels(0, y as u16, row);
        }
        Ok(())
    }
}

impl<RST, PinE> DrawTarget for Display<RST, PinE>
where
    RST: OutputPin<Error = PinE>,
{
    type Color = Colors;
    // `Display` uses a Display and doesn't need to communicate with the display
    // controller to draw pixel, which means that drawing operations can never fail. To reflect
    // this the type `Infallible` was chosen as the `Error` type.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds. `DrawTarget` implementation are required
            // to discard any out of bounds pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=PORT_AVAIL_WID_M1, y @ 0..=PORT_AVAIL_HEI_M1)) = coord.try_into()            {
                let index: u32 = x + y * PORTRAIT_AVAIL_WIDTH as u32;
                self.buf[index as usize] = color.into_storage();
            }
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buf[0..AVAIL_PIXELS].fill(color.into_storage());
        Ok(())
    }
}

impl<RST, PinE> OriginDimensions for Display<RST, PinE>
where
    RST: OutputPin<Error = PinE>,
{
    fn size(&self) -> Size {
        Size::new(PORTRAIT_AVAIL_WIDTH as u32, PORTRAIT_AVAIL_HEIGHT as u32)
    }
}

