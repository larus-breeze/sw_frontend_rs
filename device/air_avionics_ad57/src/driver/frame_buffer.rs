use crate::driver::r61580::{
    is_r61580, Orientation, AVAIL_PIXELS, PORTRAIT_AVAIL_HEIGHT, PORTRAIT_AVAIL_WIDTH,
    PORTRAIT_ORIGIN_X, PORTRAIT_ORIGIN_Y, R61580,
};
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use core::{convert::TryInto, mem::transmute, ptr::addr_of};
use corelib::{Colors, CoreError, DrawImage, Rotation, RGB565_COLORS};
use embedded_graphics::{
    draw_target::DrawTarget, geometry::OriginDimensions, prelude::*, primitives::Rectangle, Pixel,
};
use embedded_hal::blocking::delay::DelayUs;
use st7789::ST7789;
use stm32f4xx_hal::{
    dma::Stream0,
    fsmc_lcd::{AccessMode, DataPins16, FsmcLcd, Lcd, LcdPins, SubBank1, Timing},
    gpio::{alt::fsmc, Output, Pin},
    pac::{interrupt, DMA2, FSMC, NVIC},
    rcc::{Enable, Reset},
};

#[allow(unused)]
pub trait SetRow {
    fn set_pos(&mut self, pos_x: u16, pos_y: u16);
    fn set_row(&mut self, pos_x: u16, pos_y: u16, buf: &mut [u16]);
}

pub type LcdReset = Pin<'D', 3, Output>;
pub type DevLcdPins = LcdPins<DataPins16, fsmc::Address, fsmc::ChipSelect1>;

enum DisplayDriver {
    R61580(R61580<Lcd<SubBank1>>),
    ST7789(ST7789<Lcd<SubBank1>, Pin<'D', 3, Output>, Pin<'D', 3, Output>>),
}

#[allow(dead_code)]
pub struct FrameBuffer {
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
    line_y: u16,
    lcd: DisplayDriver, //R61580<Lcd<SubBank1>>,
}
impl FrameBuffer {
    pub fn new(
        fsmc: FSMC,
        _stream0: Stream0<DMA2>, // just to show, that this resource is used
        lcd_pins: DevLcdPins,
        mut lcd_reset: LcdReset,
        delay: &mut impl DelayUs<u32>,
    ) -> (Display, Self) {
        #[link_section = ".ccmram.BUFFERS"]
        static mut FRAME_BUFFER: [u8; AVAIL_PIXELS] = [0; AVAIL_PIXELS];
        let buf = unsafe {
            transmute::<*const [u8; AVAIL_PIXELS], &mut [u8; AVAIL_PIXELS]>(addr_of!(FRAME_BUFFER))
        };
        let buf2 = unsafe {
            transmute::<*const [u8; AVAIL_PIXELS], &mut [u8; AVAIL_PIXELS]>(addr_of!(FRAME_BUFFER))
        };

        static mut LINE_BUFFER: [u16; 227] = [0xaaaa; 227];
        let line_buf =
            unsafe { transmute::<*const [u16; 227], &mut [u16; 227]>(addr_of!(LINE_BUFFER)) };

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

        unsafe {
            let dma2 = &*DMA2::ptr();
            let src = &raw const LINE_BUFFER as u32;
            dma2.st[0].cr.write(|w| w.bits(0x2_2a90));
            dma2.st[0].fcr.write(|w| w.bits(0x27));
            dma2.st[0].par.write(|w| w.bits(src)); // source address
            dma2.st[0].m0ar.write(|w| w.bits(0x6002_0000)); // dest address
        }

        let (_fsmc, mut lcd_interface) = FsmcLcd::new(fsmc, lcd_pins, &timing, &timing);

        let lcd = if is_r61580(&mut lcd_interface, &mut lcd_reset) {
            // Initialize RG61580 LCD driver
            let mut lcd = R61580::new(lcd_interface, lcd_reset);
            lcd.init();
            let _ = lcd.set_orientation(Orientation::Portrait);
            DisplayDriver::R61580(lcd)
        } else {
            let mut lcd = ST7789::new(lcd_interface, Some(lcd_reset), None, 320, 240);
            // Initialise the display and clear the screen
            lcd.init(delay).unwrap();
            lcd.set_orientation(st7789::Orientation::Portrait).unwrap();
            DisplayDriver::ST7789(lcd)
        };

        (
            Display {
                buf: buf2,
                rotation: Rotation::Rotate0,
            },
            FrameBuffer {
                buf,
                line_buf,
                lcd,
                line_y: 0,
            },
        )
    }

    pub fn flush(&mut self) {
        self.line_y = 0;
        NVIC::pend(interrupt::DMA2_STREAM0);
    }

    pub fn on_interrupt(&mut self) {
        match &mut self.lcd {
            DisplayDriver::R61580(lcd) => {
                unsafe {
                    let dma2 = &*DMA2::ptr();
                    dma2.st[0].cr.modify(|_, w| w.en().clear_bit()); // disable stream0
                    dma2.lifcr.write(|w| w.ctcif0().set_bit()); // reset dma complete ir
                }

                if self.line_y < PORTRAIT_AVAIL_HEIGHT {
                    let idx_y = (self.line_y * PORTRAIT_AVAIL_WIDTH) as usize;
                    for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                        let color = self.buf[x + idx_y] as usize;
                        self.line_buf[x] = RGB565_COLORS[color];
                    }
                    lcd.set_pos(PORTRAIT_ORIGIN_X, PORTRAIT_ORIGIN_Y + self.line_y);
                    self.line_y += 1;

                    unsafe {
                        let dma2 = &*DMA2::ptr();
                        dma2.st[0].ndtr.write(|w| w.bits(227)); // count DMA moves
                        dma2.st[0].cr.modify(|_, w| w.en().set_bit()); // enable stream0
                    }
                }
            }
            DisplayDriver::ST7789(lcd) => {
                while self.line_y < PORTRAIT_AVAIL_HEIGHT {
                    let idx_y = (self.line_y * PORTRAIT_AVAIL_WIDTH) as usize;
                    for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                        let color_idx = self.buf[x + idx_y] as usize;
                        let color = RGB565_COLORS[color_idx];
                        self.line_buf[x] = color;
                    }
                    let _ = lcd.set_pixels(
                        PORTRAIT_ORIGIN_X,
                        PORTRAIT_ORIGIN_Y + self.line_y,
                        PORTRAIT_ORIGIN_X + PORTRAIT_AVAIL_WIDTH,
                        PORTRAIT_ORIGIN_Y + self.line_y + 1,
                        *self.line_buf,
                    );
                    self.line_y += 1;
                }
            }
        }
    }
}

pub struct Display {
    buf: &'static mut [u8; AVAIL_PIXELS],
    rotation: Rotation,
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
        match self.rotation {
            Rotation::Rotate180 => {
                for Pixel(coord, color) in pixels.into_iter() {
                    if let Ok((x @ 0..=PORT_AVAIL_WID_M1, y @ 0..=PORT_AVAIL_HEI_M1)) =
                        coord.try_into()
                    {
                        let idx: u32 =
                            PORT_AVAIL_WID_M1 - x + (PORT_AVAIL_HEI_M1 - y) * DISPLAY_WIDTH;
                        self.buf[idx as usize] = color.into_storage();
                    }
                }
            }
            _ => {
                for Pixel(coord, color) in pixels.into_iter() {
                    // Check if the pixel coordinates are out of bounds. `DrawTarget` implementation are required
                    // to discard any out of bounds pixels without returning an error or causing a panic.
                    if let Ok((x @ 0..=PORT_AVAIL_WID_M1, y @ 0..=PORT_AVAIL_HEI_M1)) =
                        coord.try_into()
                    {
                        let idx: u32 = x + y * PORTRAIT_AVAIL_WIDTH as u32;
                        self.buf[idx as usize] = color.into_storage();
                    }
                }
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.
        let area = area.intersection(&self.bounding_box());

        match self.rotation {
            Rotation::Rotate180 => {
                let mut row_start_idx = PORTRAIT_AVAIL_WIDTH as u32 - area.top_left.x as u32
                    + (PORTRAIT_AVAIL_HEIGHT as u32 - area.top_left.y as u32) * DISPLAY_WIDTH;
                for _y in 0..area.size.height {
                    for x in 0..area.size.width {
                        let idx = row_start_idx - x;
                        self.buf[idx as usize] = color.into_storage();
                    }
                    row_start_idx -= DISPLAY_WIDTH;
                }
            }
            _ => {
                let mut row_start_idx =
                    (area.top_left.y as u32) * DISPLAY_WIDTH + area.top_left.x as u32;
                for _ in 0..area.size.height {
                    for idx in row_start_idx..(row_start_idx + area.size.width) {
                        self.buf[idx as usize] = color.into_storage();
                    }
                    row_start_idx += DISPLAY_WIDTH;
                }
            }
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
    const DISPLAY_HEIGHT: u32 = DISPLAY_HEIGHT;
    const DISPLAY_WIDTH: u32 = DISPLAY_WIDTH;

    fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    unsafe fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors) {
        match self.rotation {
            Rotation::Rotate180 => {
                let idx = AVAIL_PIXELS - idx - 1;
                for dx in 0..len {
                    self.buf[idx - dx] = color.into_storage();
                }
            }
            _ => {
                for dx in 0..len {
                    self.buf[idx + dx] = color.into_storage();
                }
            }
        }
    }
}
