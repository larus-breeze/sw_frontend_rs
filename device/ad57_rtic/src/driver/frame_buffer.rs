use core::convert::TryInto;
use crate::driver::r61580::{
    Orientation, Instruction, AVAIL_PIXELS, PORTRAIT_AVAIL_HEIGHT, PORTRAIT_AVAIL_WIDTH, PORTRAIT_ORIGIN_X,
    PORTRAIT_ORIGIN_Y, R61580
};
use corelib::{RGB565_COLORS, Colors, CoreError, basic_config::{DISPLAY_WIDTH, DISPLAY_HEIGHT}, DrawImage};
use fmc_lcd::{AccessMode, LcdInterface, LcdPins, Timing, FSMC};
use stm32f4xx_hal::{
    gpio::{Output, Pin},
    rcc::{Enable, Reset},
    pac::{NVIC, interrupt},
    pac,
};
use embedded_graphics::{prelude::*, draw_target::DrawTarget, Pixel, primitives::Rectangle, geometry::{OriginDimensions, Point}};

pub type LcdReset = Pin<'D', 3, Output>;

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
    line_buf: &'static mut [u16; PORTRAIT_AVAIL_WIDTH as usize],
    di: LcdInterface,
    line_y: u16,
}
impl FrameBuffer {
    pub fn new(fsmc: FSMC, lcd_pins: LcdPins, lcd_reset: LcdReset) -> (Self, Display) {
        #[link_section = ".ccmram.BUFFERS"]
        static mut FRAME_BUFFER: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];
        let buf = unsafe { &mut FRAME_BUFFER };
        let buf2 = unsafe { &mut FRAME_BUFFER };

        static mut LINE_BUFFER: [u16; 227] = [0xaaaa; 227];
        let line_buf = unsafe { &mut LINE_BUFFER };

        let timing = Timing::default()
            .data(3)
            .address_setup(6)
            .bus_turnaround(0)
            .address_hold(1)
            .access_mode(AccessMode::ModeB);

        unsafe {
            // Enable the FSMC/FMC peripheral
            FSMC::enable_unchecked();
            FSMC::reset_unchecked();
        }

        let rcc = unsafe { &*pac::RCC::ptr() };
        rcc.ahb1enr.modify(|_, w| w.dma2en().set_bit());        // enable ahb1 clock for dma2

        unsafe {
            let dma2 = &*pac::DMA2::ptr() ;
            let src = LINE_BUFFER.as_ptr() as u32;
            dma2.st[0].cr.write(|w| w.bits(0x2_2a90));
            dma2.st[0].fcr.write(|w| w.bits(0x27));
            dma2.st[0].par.write(|w| w.bits(src));          // source address
            dma2.st[0].m0ar.write(|w| w.bits(0x6002_0000)); // dest address
        }

        let mut lcd_interface = LcdInterface::new(fsmc, lcd_pins, &timing, &timing);

        // Initialize RG61580 LCD driver
        let mut lcd = R61580::new(&mut lcd_interface, lcd_reset);
        let _ = lcd.set_orientation(&mut lcd_interface, Orientation::Portrait);

        (
            FrameBuffer {
                buf,
                line_buf,
                di: lcd_interface,
                line_y: 0,
            },
            Display {
                buf: buf2,
            }
        )
    }
}

/// Framebuffer for buffering the LCD content
///
/// The framebuffer is used by two instances. One instance writes the contents, another reads them
/// and passes them to the LCD
#[allow(clippy::transmute_ptr_to_ref)]
impl FrameBuffer {
    pub fn flush(&mut self) {
        self.line_y = 0;
        NVIC::pend(interrupt::DMA2_STREAM0);
    }

    pub fn on_interrupt(&mut self) {
        unsafe {
            let dma2 = &*pac::DMA2::ptr() ;
            dma2.st[0].cr.modify(|_, w| w.en().clear_bit());    // disable stream0
            dma2.lifcr.write(|w| w.ctcif0().set_bit());         // reset dma complete ir
        }
    
        if self.line_y < PORTRAIT_AVAIL_HEIGHT {
            let idx_y = (self.line_y * PORTRAIT_AVAIL_WIDTH) as usize;
            for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                let color = self.buf[x + idx_y] as usize;
                self.line_buf[x] = RGB565_COLORS[color];
            }
            write_command_and_data(Instruction::PosX as u8, PORTRAIT_ORIGIN_X);
            write_command_and_data(Instruction::PosY as u8, self.line_y + PORTRAIT_ORIGIN_Y);
            write_command(Instruction::Gram as u8);
            self.line_y += 1;

            unsafe {
                let dma2 = &*pac::DMA2::ptr();
                dma2.st[0].ndtr.write(|w| w.bits(227));         // count DMA moves
                dma2.st[0].cr.modify(|_, w| w.en().set_bit());  // enable stream0
            }
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

pub struct Display {
    buf: &'static mut [u8; AVAIL_PIXELS],
}

const PORT_AVAIL_HEI_M1: u32 = PORTRAIT_AVAIL_HEIGHT as u32 - 1;
const PORT_AVAIL_WID_M1: u32 = PORTRAIT_AVAIL_WIDTH as u32 - 1;

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
                let index: u32 = x + y * PORTRAIT_AVAIL_WIDTH as u32;
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
        Size::new(PORTRAIT_AVAIL_WIDTH as u32, PORTRAIT_AVAIL_HEIGHT as u32)
    }
}

impl DrawImage for Display {
    fn draw_img(&mut self, img: &[u8], offset: Point) -> Result<(), CoreError> {
        // Safety: the img format has been defined in terms of compatibility,(_fsmc,  so the conversion is ok here
        let img16 =
            unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u16, img.len() / 2) };
        // At the moment we only know format 1
        assert!(img16[0] == 1);

        // The image is really built for our display?
        assert!(img16[1] == DISPLAY_WIDTH as u16);
        assert!(img16[2] + offset.y as u16 <= DISPLAY_HEIGHT as u16);

        // Let's write the pixels
        let color_cnt = img16[3];
        let mut idx = 4;
        let ofs = offset.x as usize + offset.y as usize * DISPLAY_WIDTH as usize;
        for _ in 0..color_cnt {
            let color = img16[idx] as u8;
            let px_cnt = img16[idx + 1] as usize;
            idx += 2;
            for b_idx in img16.iter().skip(idx).take(px_cnt) {
                self.buf[*b_idx as usize + ofs] = color;
            }
            idx += px_cnt;
        }
        Ok(())
    }
}
