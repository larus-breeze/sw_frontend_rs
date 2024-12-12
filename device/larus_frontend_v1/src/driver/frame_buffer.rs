use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
/// The display driver consists of two components:
/// - Display unit, which uses the draw target trait and is used to build the content.
/// - Framebuffer, which is used to copy the content to the LCD.
///
/// Both components access the same buffer memory. Decoupling is achieved by calling the copy
/// routine after the image has been built up.
use core::{mem::transmute, ptr::addr_of};
use corelib::{Colors, CoreError, DrawImage};
use embedded_graphics::{draw_target::DrawTarget, prelude::*, primitives::Rectangle};
use stm32h7xx_hal::{
    device::MDMA,
    dma::{
        mdma::{MdmaConfig, MdmaIncrement, StreamX},
        traits::Direction,
        MasterTransfer, MemoryToMemory, Transfer,
    },
    gpio::{Output, Pin},
};

use crate::driver::LcdInterface;
use st7789::ST7789;

const AVAIL_PIXELS: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize;

type DisplayDriver = ST7789<LcdInterface, Pin<'C', 0, Output>, Pin<'F', 5, Output>>;
type Stream0 = StreamX<MDMA, 0>;
type Transfer0 = Transfer<
    Stream0,
    MemoryToMemory<u16>,
    MemoryToMemory<u16>,
    &'static mut [u16; 25600],
    MasterTransfer,
>;

#[link_section = ".axisram.AXISRAM"]
static mut FRAME_BUFFER: [u16; AVAIL_PIXELS] = [0; AVAIL_PIXELS];

#[derive(Copy, Clone, PartialEq, Eq)]
enum DmaState {
    State1,
    State2,
    State3,
}

const MDMA_GISR0: *const u16 = 0x5200_0000 as *const u16;
const FMC_DATA: *const u16 = 0xc002_0000 as *mut u16;

pub struct FrameBuffer {
    di_driver: DisplayDriver,
    dma_transfer: Option<Transfer0>,
    dma_state: DmaState,
}

impl FrameBuffer {
    /// Creates a frame buffer object and a display object and returns them. The display object
    /// is used by the core component to draw the LCD image. The frame buffer is used by the DMA
    /// copy routine to transport the image from the Stm32 to the LCD.
    pub fn new(di_driver: DisplayDriver, stream0: Stream0) -> (Self, Display) {
        // Note on safety: The frame buffer is used as a display and as a buffer for the DMA
        // transfer. This is ok, as these processes run one after the other and there are no
        // conflicts.
        let buf = unsafe {
            transmute::<*const [u16; AVAIL_PIXELS], &'static mut [u16; AVAIL_PIXELS]>(addr_of!(
                FRAME_BUFFER
            ))
        };

        let dma_state = DmaState::State1;
        let dma_transfer = Self::create_transfer(stream0, dma_state);
        (
            FrameBuffer {
                di_driver,
                dma_transfer: Some(dma_transfer),
                dma_state,
            },
            Display { buf },
        )
    }

    /// The flush() routine triggers the DMA transfer, which consists of 3 parts. Parts 2 and 3
    /// are automatically triggered by the DMA transfer complete interrupt.
    pub fn flush(&mut self) {
        let buf = [0_u16; 0];
        let _ = self
            .di_driver
            .set_pixels(0, 0, DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16, buf);
        self.dma_state = DmaState::State1;

        let dma_transfer = self.dma_transfer.take().unwrap();
        let (stream0, _, _, _) = dma_transfer.free();
        let mut dma_transfer = Self::create_transfer(stream0, self.dma_state);

        dma_transfer.start(|_| {});
        self.dma_transfer = Some(dma_transfer);
    }

    /// The interrupt service routine completes the transfer. Note: This routine checks whether
    /// the DMA interrupt was triggered by stream 0 before it acts. This is to ensure
    /// compatibility with any further DMA transfers.
    pub fn on_interrupt(&mut self) {
        // Note on safety: Reading a u16 is not a problem
        if (unsafe { core::ptr::read_volatile(MDMA_GISR0) } & 0x0001) != 0 {
            // Is there an interrupt on stream0
            let mut dma_transfer = self.dma_transfer.take().unwrap();
            dma_transfer.clear_transfer_complete_interrupt();

            self.dma_state = match self.dma_state {
                DmaState::State1 => DmaState::State2,
                DmaState::State2 => DmaState::State3,
                DmaState::State3 => {
                    self.dma_transfer = Some(dma_transfer);
                    return;
                }
            };

            let (stream0, _, _, _) = dma_transfer.free();
            let mut dma_transfer = Self::create_transfer(stream0, self.dma_state);

            dma_transfer.start(|_| {});
            self.dma_transfer = Some(dma_transfer);
        }
    }

    fn create_transfer(stream0: Stream0, dma_state: DmaState) -> Transfer0 {
        let config = MdmaConfig::default()
            .source_increment(MdmaIncrement::Increment)
            .destination_increment(MdmaIncrement::Fixed)
            .transfer_complete_interrupt(true);
        // Note on safety: The DMA requires the correct address of where to copy from and to.
        // This process has been carefully developed and tested.
        let src_ptr = unsafe {
            match dma_state {
                DmaState::State1 => addr_of!(FRAME_BUFFER[0]),
                DmaState::State2 => addr_of!(FRAME_BUFFER[25_600]),
                DmaState::State3 => addr_of!(FRAME_BUFFER[51_200]),
            }
        };
        let src: &'static mut [u16; 25600] = unsafe { &mut *(src_ptr as *mut [u16; 25600]) };
        let dst: &'static mut [u16; 25600] = unsafe { &mut *(FMC_DATA as *mut [u16; 25600]) };
        Transfer::init_master(stream0, MemoryToMemory::new(), dst, Some(src), config)
    }
}

const PORT_AVAIL_HEI_M1: u32 = DISPLAY_HEIGHT - 1;
const PORT_AVAIL_WID_M1: u32 = DISPLAY_WIDTH - 1;

pub struct Display {
    buf: &'static mut [u16; AVAIL_PIXELS],
}

impl DrawTarget for Display {
    type Color = Colors;
    type Error = CoreError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds. `DrawTarget` implementation are required
            // to discard any out of bounds pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=PORT_AVAIL_WID_M1, y @ 0..=PORT_AVAIL_HEI_M1)) = coord.try_into() {
                let index: u32 = x + y * DISPLAY_WIDTH;
                self.buf[index as usize] = color.into_storage();
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.
        let area = area.intersection(&self.bounding_box());
        let mut row_start_idx = (area.top_left.y as u32) * DISPLAY_WIDTH + area.top_left.x as u32;

        for _row in 0..area.size.height {
            for idx in row_start_idx..(row_start_idx + area.size.width) {
                self.buf[idx as usize] = color.into_storage();
            }
            row_start_idx += DISPLAY_WIDTH;
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buf[0..AVAIL_PIXELS].fill(color.into_storage());
        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }
}

impl DrawImage for Display {
    const DISPLAY_HEIGHT: u32 = DISPLAY_HEIGHT;
    const DISPLAY_WIDTH: u32 = DISPLAY_WIDTH;

    fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors) {
        for dx in 0..len {
            self.buf[idx + dx] = color.into_storage();
        }
    }
}
