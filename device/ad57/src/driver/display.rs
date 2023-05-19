use core::convert::TryInto;
use embedded_graphics::{
    prelude::*,
};
use crate::{Colors, RGB565_COLORS};
use crate::driver::st7789::ST7789;

use display_interface::WriteOnlyDataCommand;
use embedded_hal::digital::v2::OutputPin;

use crate::utils::Error;

pub struct Display<'a, DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    buffer: [u8; 240 * 240],
    lcd: &'a mut ST7789<DI, RST>
}

impl<'a, DI, RST, PinE> Display<'a, DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    pub fn new(lcd: &'a mut ST7789<DI, RST>)  -> Self
    where
        DI: WriteOnlyDataCommand,
        RST: OutputPin<Error = PinE>,
    {
        Display { buffer: [0; 240*240], lcd }
    }

    /// Updates the display from the Display.
    pub fn flush(&mut self) -> Result<(), Error<PinE>> {
        let mut row: [u16; 240] = [0; 240];
        for y in 0..240 {
            for x in 0..240 {
                row[x] = RGB565_COLORS[self.buffer[y*240 + x] as usize].into_storage();
            }
            self.lcd.set_pixels(0, y as u16, 239, y as u16, row)?;
        }
        Ok(())
    }
}

impl<DI, RST, PinE> DrawTarget for Display<'_, DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    type Color = Colors;
    // `Display` uses a Display and doesn't need to communicate with the display
    // controller to draw pixel, which means that drawing operations can never fail. To reflect
    // this the type `Infallible` was chosen as the `Error` type.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds. `DrawTarget` implementation are required
            // to discard any out of bounds pixels without returning an error or causing a panic.
            if let Ok((x  @ 0..=239, y @ 0..=239)) = coord.try_into() {
                let index: u32 = x + y * 240;
                self.buffer[index as usize] = color.into_storage();
            }
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buffer[0..240*240].fill(color.into_storage());
        Ok(())
    }
}

impl<DI, RST, PinE> OriginDimensions for Display<'_, DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    fn size(&self) -> Size {
        Size::new(240, 240)
    }
}

