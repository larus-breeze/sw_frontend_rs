use crate::driver::r61580::{
    Instruction, Orientation, AVAIL_PIXELS, PORTRAIT_AVAIL_HEIGHT, PORTRAIT_AVAIL_WIDTH,
    PORTRAIT_ORIGIN_X, PORTRAIT_ORIGIN_Y, R61580,
};
use crate::{driver::frame_buffer::FrameBuffer, Colors, RGB565_COLORS};
use stm32f4xx_hal::{
    gpio::{Output, Pin, alt::fsmc},
    fsmc_lcd::{AccessMode, DataPins16, FsmcLcd, LcdPins, Timing},
    pac::FSMC,
};

use core::convert::TryInto;
use embedded_graphics::{prelude::*, primitives::Rectangle};
use vario_display::*;

const PORT_AVAIL_HEI_M1: u32 = PORTRAIT_AVAIL_HEIGHT as u32 - 1;
const PORT_AVAIL_WID_M1: u32 = PORTRAIT_AVAIL_WIDTH as u32 - 1;

pub type LcdReset = Pin<'D', 3, Output>;
pub type DevLcdPins = LcdPins<DataPins16, fsmc::Address, fsmc::ChipSelect1>;

#[allow(unused)]
pub struct Display {
    buf: &'static mut [u8; AVAIL_PIXELS],
    lcd: R61580<LcdReset>,
}

impl Display {
    pub fn new(fsmc: FSMC, lcd_pins: DevLcdPins, lcd_reset: LcdReset, fb: FrameBuffer) -> Self {

        let timing = Timing::default()
            .data(3)
            .address_setup(6)
            .bus_turnaround(0)
            .address_hold(1)
            .access_mode(AccessMode::ModeB);

        let (_fsmc, _interface) = FsmcLcd::new(fsmc, lcd_pins, &timing, &timing);

        // Initialize RG61580 LCD driver
        let mut lcd = R61580::new(lcd_reset);
        lcd.init();
        let _ = lcd.set_orientation(Orientation::Portrait);

        Display { buf: fb.buf, lcd }
    }
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
        // Safety: the img format has been defined in terms of compatibility, so the conversion is ok here
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

pub fn flush(frame_buffer: &FrameBuffer) {
    #[inline]
    fn write_command(cmd: u8) {
        unsafe { core::ptr::write_volatile(0x60000000 as *mut u8, cmd) }
    }

    #[inline]
    fn write_data(data: u16) {
        unsafe { core::ptr::write_volatile(0x60020000 as *mut u16, data) };
    }

    fn write_command_and_data(cmd: u8, data: u16) {
        write_command(cmd);
        write_data(data)
    }

    for y in 0..PORTRAIT_AVAIL_HEIGHT {
        write_command_and_data(Instruction::PosX as u8, PORTRAIT_ORIGIN_X);
        write_command_and_data(Instruction::PosY as u8, y + PORTRAIT_ORIGIN_Y);
        write_command(Instruction::Gram as u8);
        let idx_y = (y * PORTRAIT_AVAIL_WIDTH) as usize;
        for x in 0..PORTRAIT_AVAIL_WIDTH as usize {
            let color = frame_buffer.buf[x + idx_y] as usize;
            write_data(RGB565_COLORS[color]);
        }
    }
}
