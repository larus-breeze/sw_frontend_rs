use super::{FrameBuffer, TBuffer, AVAIL_PIXELS};
use core::{mem, mem::transmute, ptr::addr_of};
use corelib::{Colors, CoreError, DrawImage, RGB565_COLORS, Rotation};
use embedded_graphics::{
    draw_target::DrawTarget, geometry::OriginDimensions, prelude::*, primitives::Rectangle, Pixel,
};
use embedded_hal::can::Frame;
use stm32h7xx_hal::pac;

pub const DISPLAY_HEIGHT: u32 = 480;
pub const DISPLAY_WIDTH: u32 = 480;

const PORT_AVAIL_HEI_M1: u32 = DISPLAY_HEIGHT - 1;
const PORT_AVAIL_WID_M1: u32 = DISPLAY_WIDTH - 1;

pub struct Display {
    buf: TBuffer,
    frame_buffer: FrameBuffer,
}

impl Display {
    pub fn new(mut frame_buffer: FrameBuffer) -> Self {
        let buf = frame_buffer.swap_buffers();
        Display { buf, frame_buffer }
    }

    pub fn show(&mut self) {
        self.buf = self.frame_buffer.swap_buffers();
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
                let index: u32 = x + y * DISPLAY_WIDTH as u32;
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

    fn set_rotation(&mut self, _rotation: Rotation) {
    }

    unsafe fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors) {
        for dx in 0..len {
            self.buf[idx + dx] = color.into_storage();
        }
    }
}
