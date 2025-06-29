
use corelib::*;
use crate::dev_const::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
use embedded_graphics::{
    draw_target::DrawTarget, 
    geometry::{OriginDimensions, Point, Size}, 
    pixelcolor::Bgr888, 
    prelude::IntoStorage, Pixel,
    primitives::Rectangle,
};
use std::{io::Cursor, mem::transmute, path::PathBuf};
use image::{ImageBuffer, ImageReader, Rgba};

const DISPLAY_PIXELS: u32 = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub const PADDING: u32 = 10;
pub const DISPLAY_WIDTH_INC_PAD: u32 = DISPLAY_WIDTH + 2*PADDING;
pub const DISPLAY_HEIGHT_INC_PAD: u32 = DISPLAY_HEIGHT + 2*PADDING;
const DISPLAY_PIXELS_INC_PAD: u32 = (DISPLAY_WIDTH_INC_PAD)*(DISPLAY_HEIGHT_INC_PAD);
const DISPLAY_WIDTH_1: u32 = DISPLAY_WIDTH - 1;
const DISPLAY_HEIGHT_1: u32 = DISPLAY_HEIGHT - 1;

const BACKGROUND_IMAGE: &[u8] = include_bytes!("background.png");

pub struct Display {
    pub buffer: [u32; DISPLAY_PIXELS as usize],
    pub snapshot_buf: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub background: [u32; (DISPLAY_PIXELS_INC_PAD) as usize],
    rotation: Rotation,
}

impl Display {
    pub fn new() -> Display {
    let reader = ImageReader::new(Cursor::new(BACKGROUND_IMAGE))
        .with_guessed_format()
        .expect("Cursor io never fails");
    let img = reader.decode().unwrap().to_rgba8();
    let mut buf = [0u8; (DISPLAY_PIXELS_INC_PAD * 4) as usize];
    buf.copy_from_slice(&img);
    let background = unsafe { transmute(buf) };
    Display { 
        buffer: [0; DISPLAY_PIXELS as usize],
        snapshot_buf: None,
        rotation: Rotation::Rotate0,
        background,
    }
    }

    pub fn as_raw(&self) -> &[u8; (DISPLAY_PIXELS * 4) as usize] {
        unsafe { std::mem::transmute(&self.buffer) }
    }

    pub fn as_pixels(&self) -> &[Rgba<u8>; DISPLAY_PIXELS as usize] {
        unsafe { std::mem::transmute(&self.buffer) }
    }

    pub fn copy(&self) -> [u8; (DISPLAY_PIXELS * 4) as usize] {
        let mut buf = [0_u8; (DISPLAY_PIXELS * 4) as usize];
        buf.copy_from_slice(self.as_raw());
        buf
    }

    pub fn bounding_box(&self) -> Rectangle {
        Rectangle { top_left: (0, 0).into(), size: (DISPLAY_WIDTH, DISPLAY_HEIGHT).into() }
    }

    pub fn image_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::new(DISPLAY_WIDTH_INC_PAD, DISPLAY_HEIGHT_INC_PAD);
        let pixels = self.as_pixels();
        for x in 0..DISPLAY_WIDTH_INC_PAD {
            for y in 0..DISPLAY_HEIGHT_INC_PAD {
                let idx = (y * DISPLAY_WIDTH_INC_PAD + x) as usize;
                let background = self.background[idx];
                if background == 0xffff_ffff { // white
                    *imgbuf.get_pixel_mut(x, y) = pixels[((y-PADDING)*DISPLAY_WIDTH + x-PADDING) as usize];
                } else {
                    *imgbuf.get_pixel_mut(x, y) = Rgba::<u8>::from(background.to_le_bytes());
                }
            }
        }
        imgbuf
    }

    pub fn take_snapshot(&mut self) {
        self.snapshot_buf = Some(self.image_buffer());
    }

    pub fn save(&self, path: Option<PathBuf>) {
        if self.snapshot_buf.is_none() {
            eprintln!("Error, no snapshot taken!");
            return;
        }
        match path {
            Some(path) => match self.snapshot_buf.as_ref().unwrap().save(&path) {
                Ok(_) => (),
                Err(_) => eprintln!("Could not write to '{:?}'", path),
            }
            None => (),
        }
    } 
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
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
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=DISPLAY_WIDTH_1, y @ 0..=DISPLAY_HEIGHT_1)) = coord.try_into() {
                // Calculate the index in the framebuffer.
                let index: u32 = x + y * DISPLAY_WIDTH;
                self.buffer[index as usize] = 0xff00_0000 + Bgr888::from(color).into_storage();
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.

        let area = area.intersection(&self.bounding_box());
        let xs = area.top_left.x as u32;
        let ys = area.top_left.y as u32;
        let c = 0xff00_0000 + Bgr888::from(color).into_storage();
        for x in xs..xs+area.size.width {
            for y in ys..ys+area.size.height {
                self.buffer[(y*DISPLAY_WIDTH + x) as usize] = c;
            }
        }
        Ok(())
    }

}

impl DrawImage for Display {
    const DISPLAY_HEIGHT: u32 = DISPLAY_HEIGHT;
    const DISPLAY_WIDTH: u32 = DISPLAY_WIDTH;

    fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    unsafe fn draw_line_unchecked(&mut self, idx: usize, len: usize, color: Colors) {
        let x = (idx % (DISPLAY_WIDTH as usize)) as i32;
        let y = (idx / (DISPLAY_WIDTH as usize)) as i32;
        let top_left = Point::new(x, y);
        let size = Size::new(len as u32, 1);
        let area = Rectangle::new(top_left, size);
        let _ = self.fill_solid(&area, color.into());
    }
}

