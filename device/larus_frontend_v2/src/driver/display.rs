use super::{FrameBuffer, TBuffer, AVAIL_PIXELS};
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use corelib::{Colors, CoreError, DrawImage, Rotation};
use embedded_graphics::{
    draw_target::DrawTarget, geometry::OriginDimensions, prelude::*, primitives::Rectangle, Pixel,
};

const WIDTH_M1: u32 = DISPLAY_WIDTH - 1;

pub struct Display {
    buf: TBuffer,
    frame_buffer: FrameBuffer,
    rotation: Rotation,
}

impl Display {
    pub fn new(mut frame_buffer: FrameBuffer) -> Self {
        let buf = frame_buffer.swap_buffers();
        Display {
            buf,
            frame_buffer,
            rotation: Rotation::Rotate0,
        }
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
        match self.rotation {
            Rotation::Rotate0 => {
                for Pixel(coord, color) in pixels.into_iter() {
                    // Check if the pixel coordinates are out of bounds. `DrawTarget` implementation are required
                    // to discard any out of bounds pixels without returning an error or causing a panic.
                    if let Ok((x @ 0..=WIDTH_M1, y @ 0..=WIDTH_M1)) = coord.try_into() {
                        let idx: u32 = x + y * DISPLAY_WIDTH as u32;
                        self.buf[idx as usize] = color.into_storage();
                    }
                }
            }
            Rotation::Rotate90 => {
                for Pixel(coord, color) in pixels.into_iter() {
                    if let Ok((x @ 0..=WIDTH_M1, y @ 0..=WIDTH_M1)) = coord.try_into() {
                        let idx: u32 = ((x + 1) * DISPLAY_WIDTH) as u32 - 1 - y;
                        self.buf[idx as usize] = color.into_storage();
                    }
                }
            }
            Rotation::Rotate180 => {
                for Pixel(coord, color) in pixels.into_iter() {
                    if let Ok((x @ 0..=WIDTH_M1, y @ 0..=WIDTH_M1)) = coord.try_into() {
                        let idx: u32 = WIDTH_M1 - x + (WIDTH_M1 - y) * DISPLAY_WIDTH;
                        self.buf[idx as usize] = color.into_storage();
                    }
                }
            }
            Rotation::Rotate270 => {
                for Pixel(coord, color) in pixels.into_iter() {
                    if let Ok((x @ 0..=WIDTH_M1, y @ 0..=WIDTH_M1)) = coord.try_into() {
                        let idx: u32 = (WIDTH_M1 - x) * DISPLAY_WIDTH + y;
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
            Rotation::Rotate0 => {
                let mut row_start_idx =
                    (area.top_left.y as u32) * DISPLAY_WIDTH + area.top_left.x as u32;
                for _ in 0..area.size.height {
                    for idx in row_start_idx..(row_start_idx + area.size.width) {
                        self.buf[idx as usize] = color.into_storage();
                    }
                    row_start_idx += DISPLAY_WIDTH;
                }
            }
            Rotation::Rotate90 => {
                let mut row_start_idx =
                    (area.top_left.x as u32 + 1) * DISPLAY_WIDTH - area.top_left.y as u32 - 1;
                for _x in 0..area.size.width {
                    for y in 0..area.size.height {
                        let idx = row_start_idx - y;
                        self.buf[idx as usize] = color.into_storage();
                    }
                    row_start_idx += DISPLAY_WIDTH;
                }
            }
            Rotation::Rotate180 => {
                let mut row_start_idx = WIDTH_M1 - area.top_left.x as u32
                    + (WIDTH_M1 - area.top_left.y as u32) * DISPLAY_WIDTH;
                for _y in 0..area.size.height {
                    for x in 0..area.size.width {
                        let idx = row_start_idx - x;
                        self.buf[idx as usize] = color.into_storage();
                    }
                    row_start_idx -= DISPLAY_WIDTH;
                }
            }
            Rotation::Rotate270 => {
                let mut row_start_idx =
                    (WIDTH_M1 - area.top_left.x as u32) * DISPLAY_WIDTH + area.top_left.y as u32;
                for _ in 0..area.size.width {
                    for y in 0..area.size.height {
                        let idx = row_start_idx + y;
                        self.buf[idx as usize] = color.into_storage();
                    }
                    row_start_idx -= DISPLAY_WIDTH;
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
        Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
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
            Rotation::Rotate0 => {
                for dx in 0..len {
                    self.buf[idx + dx] = color.into_storage();
                }
            }
            Rotation::Rotate90 => {
                let x = idx as u32 % DISPLAY_WIDTH;
                let y = idx as u32 / DISPLAY_WIDTH;
                let mut idx = (x + 1) * DISPLAY_WIDTH - y - 1;
                for _ in 0..len {
                    self.buf[idx as usize] = color.into_storage();
                    idx += DISPLAY_WIDTH;
                }
            }
            Rotation::Rotate180 => {
                let idx = AVAIL_PIXELS - idx - 1;
                for dx in 0..len {
                    self.buf[idx - dx] = color.into_storage();
                }
            }
            Rotation::Rotate270 => {
                let x = idx as u32 % DISPLAY_WIDTH;
                let y = idx as u32 / DISPLAY_WIDTH;
                let mut idx = (WIDTH_M1 - x) * DISPLAY_WIDTH + y;
                for _ in 0..len {
                    self.buf[idx as usize] = color.into_storage();
                    idx -= DISPLAY_WIDTH;
                }
            }
        }
    }
}
