use crate::driver::r61580::AVAIL_PIXELS;

#[allow(dead_code)]
pub struct FrameBuffer {
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
}

#[allow(clippy::new_without_default)]
impl FrameBuffer {
    pub fn new() -> Self {
        static mut FRAME_BUFFER: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];
        let buf = unsafe { &mut FRAME_BUFFER };

        FrameBuffer { buf }
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
        FrameBuffer { buf: buf2 }
    }
}
