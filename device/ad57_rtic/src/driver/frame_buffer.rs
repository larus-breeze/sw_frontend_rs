use crate::driver::r61580::{
    Orientation, AVAIL_PIXELS, PORTRAIT_AVAIL_HEIGHT, PORTRAIT_AVAIL_WIDTH,
    PORTRAIT_ORIGIN_X, PORTRAIT_ORIGIN_Y, R61580,
};
use core::convert::TryInto;
use corelib::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    Colors, CoreError, DrawImage, RGB565_COLORS,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point},
    prelude::*,
    primitives::Rectangle,
    Pixel,
};
use stm32f4xx_hal::{
    fsmc_lcd::{AccessMode, DataPins16, FsmcLcd, Lcd, LcdPins, SubBank1, Timing},
    gpio::{alt::fsmc, Output, Pin},
    pac::{interrupt, FSMC, NVIC},
    rcc::{Enable, Reset},
};

pub trait SetRow {
    fn set_row(&mut self, pos_x: u16, pos_y: u16, buf: &mut [u16]);
}

pub type LcdReset = Pin<'D', 3, Output>;
pub type DevLcdPins = LcdPins<DataPins16, fsmc::Address, fsmc::ChipSelect1>;

enum DisplayDriver {
    R61580(R61580<Lcd<SubBank1>>),
}

#[allow(dead_code)]
pub struct FrameBuffer
{
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
    line_buf: [u16; PORTRAIT_AVAIL_WIDTH as usize],
    line_y: u16,
    lcd: DisplayDriver, //R61580<Lcd<SubBank1>>,
}
impl FrameBuffer {
    pub fn new(fsmc: FSMC, lcd_pins: DevLcdPins, lcd_reset: LcdReset) -> (Display, Self) {
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

        let (_fsmc, lcd_interface) = FsmcLcd::new(fsmc, lcd_pins, &timing, &timing);

        // Initialize RG61580 LCD driver
        let mut lcd = R61580::new(lcd_interface, lcd_reset);
        lcd.init();
        let _ = lcd.set_orientation(Orientation::Portrait);

        (
            Display { buf: buf2 },
            FrameBuffer {
                buf,
                line_buf: [0xaaaa; 227],
                lcd: DisplayDriver::R61580(lcd),
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
                while self.line_y < PORTRAIT_AVAIL_HEIGHT {
                    let idx_y = (self.line_y * PORTRAIT_AVAIL_WIDTH) as usize;
                    for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
                        let color_idx = self.buf[x + idx_y] as usize;
                        let color = RGB565_COLORS[color_idx];
                        self.line_buf[x] = color;
                    }
                    lcd.set_row(
                        PORTRAIT_ORIGIN_X, 
                        PORTRAIT_ORIGIN_Y + self.line_y, 
                        &mut self.line_buf
                    );
                    self.line_y += 1;
                }
            }
        }
    }
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
