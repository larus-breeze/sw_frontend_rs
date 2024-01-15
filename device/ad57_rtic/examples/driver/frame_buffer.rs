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
    dma::{DmaDirection, Channel0, Transfer, StreamX, StreamsTuple, config::DmaConfig, MemoryToMemory, Stream0, traits::{Direction, DMASet, }},
    pac::{DMA2, Peripherals as DevicePeripherals},
};
use embedded_graphics::{prelude::*, draw_target::DrawTarget, Pixel, primitives::Rectangle, geometry::{OriginDimensions, Point}};

use super::delay_ms;


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
    di: LcdInterface,
    idx_x: usize,
    idx_y: usize,
}
impl FrameBuffer {
    pub fn new(fsmc: FSMC, lcd_pins: LcdPins, lcd_reset: LcdReset) -> (Self, Display) {
        #[link_section = ".ccmram.BUFFERS"]
        static mut FRAME_BUFFER: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];

        let buf = unsafe { &mut FRAME_BUFFER };
        let buf2 = unsafe { &mut FRAME_BUFFER };

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

        let mut lcd_interface = LcdInterface::new(fsmc, lcd_pins, &timing, &timing);

        // Initialize RG61580 LCD driver
        let mut lcd = R61580::new(&mut lcd_interface, lcd_reset);
        let _ = lcd.set_orientation(&mut lcd_interface, Orientation::Portrait);

        (
            FrameBuffer {
                buf,
                di: lcd_interface,
                idx_x: 0,
                idx_y: 0,
            },
            Display {
                buf: buf2,
            }
        )
    }
}

const DMA2_S0CR: usize = 0x4002_6410;
const DMA2_S0NDTR: usize = 0x4002_6414;
const DMA2_S0PAR: usize = 0x4002_6418;
const DMA2_S0MOAR: usize = 0x4002_641c;
const DMA2_S0FCR: usize = 0x4002_6424;
const LCD_RAM: usize = 0x6002_0000;



/// Framebuffer for buffering the LCD content
///
/// The framebuffer is used by two instances. One instance writes the contents, another reads them
/// and passes them to the LCD
#[allow(clippy::transmute_ptr_to_ref)]
impl FrameBuffer {
    pub fn flush(&mut self) {
        static mut buf: [u8; 227] = [0_u8; 227];
        for y in 0..PORTRAIT_AVAIL_HEIGHT {
            let mut b: &mut u8 = unsafe { core::mem::transmute::<usize, &mut u8>(LCD_RAM) };
            let mut b2: &mut [u8; 227]= unsafe { core::mem::transmute::<&u8, &mut [u8;227]>(b) };
                // This implementation is dirty and fast. At this point, we own the display interface, so
            // we can go this way without fear.
            /*unsafe {
                core::ptr::write_volatile(DMA2_S0CR as *mut u32, 0x0002_2a90);
                core::ptr::write_volatile(DMA2_S0NDTR as *mut u32, 0x0000_8000);
                core::ptr::write_volatile(DMA2_S0PAR as *mut u32, buf.as_ptr() as u32);
                core::ptr::write_volatile(DMA2_S0MOAR as *mut u32, 0x6002_0000);
                core::ptr::write_volatile(DMA2_S0FCR as *mut u32, 0x0000_0027);
            }*/
            let idx_y = (y * PORTRAIT_AVAIL_WIDTH) as usize;
            for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                let color = self.buf[x + idx_y] as usize;
                unsafe { buf[x] = RGB565_COLORS[color] as u8 };
                //write_data(RGB565_COLORS[color]);
            }
            type LcdDma = Transfer<
                Stream0<DMA2>,
                0,
                MemoryToMemory<u8>,
                MemoryToMemory<u8>,
                &'static mut [u8; 227],
            >;
            let dp = unsafe { DevicePeripherals::steal() };
            let streams = StreamsTuple::new(dp.DMA2);
            let config = DmaConfig::default()
                .memory_increment(true)
                .peripheral_increment(false);

            type Mem2Mem = DMASet<StreamX<DMA2, 0>, 0, MemoryToMemory<u16>>;

            let mem2mem: MemoryToMemory<u8> = MemoryToMemory::new();
            //let mem2mem = stm32f4xx_hal::dma::MemoryToMemory::<u8>::new();
            unsafe {
                let lcd_transfer: LcdDma = Transfer::init_memory_to_memory(streams.0, mem2mem, &mut buf, b2, config);
            }

            write_command_and_data(Instruction::PosX as u8, PORTRAIT_ORIGIN_X);
            write_command_and_data(Instruction::PosY as u8, y + PORTRAIT_ORIGIN_Y);
            write_command(Instruction::Gram as u8);
            /*unsafe {
                core::ptr::write_volatile(DMA2_S0CR as *mut u32, 0x0002_2a91);
            }
            delay_ms(1);*/
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
