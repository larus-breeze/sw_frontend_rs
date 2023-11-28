use crate::RGB565_COLORS;
use crate::driver::r61580::{
    Instruction, AVAIL_PIXELS, PORTRAIT_AVAIL_HEIGHT, PORTRAIT_AVAIL_WIDTH,
    PORTRAIT_ORIGIN_X, PORTRAIT_ORIGIN_Y,
};
use stm32f4xx_hal::fsmc_lcd::{SubBank1, Lcd};

#[allow(dead_code)]
pub struct FrameBuffer 
{
    // REMARK: These are conceptual thoughts, currently no DMA has been implemented.
    //
    // A note aboute the safety of FrameBuffer: REMARK: A
    //
    // The display driver and the DMA copy routine must both have access to the FRAME_BUFFER and DMA_FINISHED variables.
    // While the display driver needs write access to the buffer, the DMA copy routine reads from it. According to plan
    // these processes run staggered. First is written and then read. If this time offset is not fulfilled, this is not
    // a catastrophe, because nothing crashes. There is just some flickering. However, the DMA_FINISHED variable is
    // intended to detect such an overlap.
    //
    // How the Flag DMA_FINSHED is used:
    // After the display driver has successfully written to the buffer, the flag is set to false and a DMA interrupt is
    // triggered. The interrupt service routine recognizes by the pattern flag false and DMA empty that it must restart
    // the DMA process. After the complete copying of the data the DMA interrupt service routine sets the flag to true.
    // Before the next buffer write, this flag can be used to assess whether the copy process was completed successfully.
    pub buf: &'static mut [u8; AVAIL_PIXELS],
    di: Option<Lcd<SubBank1>>,
    idx_x: usize,
    idx_y: usize,
}

#[allow(clippy::new_without_default)]
impl FrameBuffer {
    pub fn new(di: Lcd<SubBank1>) -> Self {
        #[link_section = ".ccmram.BUFFERS"]
        static mut FRAME_BUFFER: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];

        let buf = unsafe { &mut FRAME_BUFFER };

        FrameBuffer { buf, di: Some(di), idx_x: 0, idx_y: 0 }
    }
}

/// Framebuffer for buffering the LCD content
///
/// The framebuffer is used by two instances. One instance writes the contents, another reads them
/// and passes them to the LCD
#[allow(clippy::transmute_ptr_to_ref)]
impl FrameBuffer {
    // Safety: The framebuffer must be written by one instance and read by another. The individual
    // accesses are atomic. It also does not matter if while the buffer is being written,
    // it is being read at the same time, since the contents only change slowly. However,
    // the process takes care that an overlap does not happen.
    pub fn split(&self) -> FrameBuffer {
        let raw = self.buf as *const [u8; AVAIL_PIXELS];
        let buf2 = unsafe {
            core::mem::transmute::<*const [u8; AVAIL_PIXELS], &mut [u8; AVAIL_PIXELS]>(raw)
        };
        FrameBuffer { buf: buf2, di: None, idx_x: 0, idx_y: 0}
    }

    pub fn flush(&mut self) {
        if self.di.is_some() {
            for y in 0..PORTRAIT_AVAIL_HEIGHT {
            // This implementation is dirty and fast. At this point, we own the display interface, so
            // we can go this way without fear.
                write_command_and_data(Instruction::PosX as u8, PORTRAIT_ORIGIN_X);
                write_command_and_data(Instruction::PosY as u8, y + PORTRAIT_ORIGIN_Y);
                write_command(Instruction::Gram as u8);
                let idx_y = (y * PORTRAIT_AVAIL_WIDTH) as usize;
                for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                    let color = self.buf[x + idx_y] as usize;
                    write_data(RGB565_COLORS[color]);
                }
            }
            /* Here is the implementation as intended, but slow and memory-hungry
            
            if let Some(di) = &mut self.di {
                let _ = di.send_commands(U8(&[Instruction::PosX as u8]));
                let _ = di.send_data(U16(&[PORTRAIT_ORIGIN_X]));
                let _ = di.send_commands(U8(&[Instruction::PosY as u8]));
                let _ = di.send_data(U16(&[y + PORTRAIT_ORIGIN_Y]));
                let _ = di.send_commands(U8(&[Instruction::Gram as u8]));
                let idx_y = (y * PORTRAIT_AVAIL_WIDTH) as usize;
                let mut col_buf = [0_u16; PORTRAIT_AVAIL_WIDTH as usize];
                for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                    col_buf[x] = RGB565_COLORS[self.buf[x + idx_y] as usize];
                }
                let _ = di.send_data(U16(&col_buf));
            }*/
        }
    }
}




#[inline]
fn write_command(cmd: u8) {
    // Safety: Writing u8 is atomic, so unsafe is ok
    unsafe { core::ptr::write_volatile(0x60000000 as *mut u8, cmd) }
}

#[inline]
fn write_data(data: u16) {
    // Safety: Writing u16 is atomic, so unsafe is ok
    unsafe { core::ptr::write_volatile(0x60020000 as *mut u16, data) };
}

#[inline]
fn write_command_and_data(cmd: u8, data: u16) {
    write_command(cmd);
    write_data(data)
}

