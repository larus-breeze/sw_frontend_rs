use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
};
use crate::{Colors, CoreError, DrawImage};

pub struct Image {
    img: &'static [u8],
}

impl Image {
    pub fn new(img: &'static [u8]) -> Self {
        Image { img }
    }

    pub fn width(&self) -> u32 {
        assert!(self.img[0] == 4);
        self.img[4] as u32 + (self.img[5] as u32) * 256
    }

    pub fn height(&self) -> u32 {
        assert!(self.img[0] == 4);
        self.img[6] as u32 + (self.img[7] as u32) * 256
    }

    pub fn size(&self) -> Size {
        Size { width: self.width(), height: self.height() }
    }

    pub fn draw<D>(
        &self,
        display: &mut D,
        offset: Point,
        cover_up: Option<Colors>,
    ) -> Result<(), CoreError>
    where
        D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
        Self: Sized,
    {
        display.draw_img(self.img, offset, cover_up)
    }
}