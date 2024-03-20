//! This crate provides a ST7789 driver to connect to TFT displays.

use display_interface::{
    DataFormat::{U16BEIter, U8Iter, U16BE},
    WriteOnlyDataCommand,
};
use embedded_hal::digital::v2::OutputPin;

use crate::driver::delay_ms;
use crate::utils::DevError;
use core::iter::once;

use super::frame_buffer::SetRow;

// Total LCD Dimensions
pub const PORTRAIT_TOTAL_WIDTH: u16 = 240;
pub const PORTRAIT_TOTAL_HEIGHT: u16 = 320;
pub const LANDSCAPE_TOTAL_WIDTH: u16 = PORTRAIT_TOTAL_HEIGHT;
#[allow(dead_code)]
pub const LANDSCAPE_TOTAL_HEIGHT: u16 = PORTRAIT_TOTAL_WIDTH;
//pub const TOTAL_PIXELS: u32 = PORTRAIT_TOTAL_HEIGHT as u32 * PORTRAIT_TOTAL_WIDTH as u32;

// Visible Window Portrait
pub const PORTRAIT_ORIGIN_X: u16 = 6;
pub const PORTRAIT_ORIGIN_Y: u16 = 16;
pub const PORTRAIT_AVAIL_WIDTH: u16 = 227;
pub const PORTRAIT_AVAIL_HEIGHT: u16 = 285;
pub const AVAIL_PIXELS: usize = PORTRAIT_AVAIL_HEIGHT as usize * PORTRAIT_AVAIL_WIDTH as usize;

// Visible Window Landscape
pub const LANDSCAPE_ORIGIN_X: u16 = PORTRAIT_ORIGIN_Y;
pub const LANDSCAPE_ORIGIN_Y: u16 = PORTRAIT_ORIGIN_X;
pub const LANDSCAPE_AVAIL_WIDTH: u16 = PORTRAIT_AVAIL_HEIGHT;
pub const LANDSCAPE_AVAIL_HEIGHT: u16 = PORTRAIT_AVAIL_WIDTH;

///
/// Instructions for the R61580 LCD Controller
///
#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    DriverId = 0x00, // ID of Display Chip
    PosX = 0x20,     // Cursor pos x
    PosY = 0x21,     // Cursor pos y
    Gram = 0x22,     // Start Gram

    HSA = 0x50, // x-start
    HEA = 0x51, // x-end
    VSA = 0x52, // y-start
    VEA = 0x53, // y-end
}

///
/// R61580 driver to connect to LCD displays.
///
pub struct R61580<DI> {
    di: DI
}

///
/// Display orientation.
///
#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(unused)]
pub enum Orientation {
    Portrait,  // no inverting
    Landscape, // invert column and page/column order
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Portrait
    }
}

#[allow(unused)]
impl <DI>R61580<DI> 
where
    DI: WriteOnlyDataCommand,
{
    ///
    /// Creates a new ST7789 driver instance
    ///
    /// # Arguments
    ///
    /// * `di` - a display interface for talking with the display
    /// * `rst` - display hard reset pin
    /// * `size_x` - x axis resolution of the display in pixels
    /// * `size_y` - y axis resolution of the display in pixels
    ///
    pub fn new<RST, PinE>(di: DI, mut rst: RST) -> Self
    where
        RST: OutputPin<Error = PinE>,
        DI: WriteOnlyDataCommand,
    {
        // Performs a hard reset using the RST pin sequence
        let _ = rst.set_high();
        delay_ms(1); // ensure the pin change will get registered
        let _ = rst.set_low();
        delay_ms(2); // ensure the pin change will get registered
        let _ = rst.set_high();
        delay_ms(2); // ensure the pin change will get registered

        R61580 { di }
    }

    pub fn init(&mut self) {
        // Check, if display answers
        self.write_command(0);
        let id = self.read_data();

        // 4x RS=0
        self.write_command_and_data(0x0000, 0x0000);
        self.write_command_and_data(0x0000, 0x0000);
        self.write_command_and_data(0x0000, 0x0000);
        self.write_command_and_data(0x0000, 0x0000);
        delay_ms(200);
        // Setup display
        self.write_command_and_data(0x00A4, 0x0001); // NVW Calibration: CALB=1
        delay_ms(1);

        self.write_command_and_data(0x0060, 0xA700); // Driver Output Control 2: GS=1, NL=0x27, SCN=0
        self.write_command_and_data(0x0008, 0x0503); // Display Control 2: BP=3, FP=5

        self.write_command_and_data(0x0030, 0x0500); // y control
        self.write_command_and_data(0x0031, 0x3711); // y control
        self.write_command_and_data(0x0032, 0x0605); // y control
        self.write_command_and_data(0x0033, 0x120D); // y control
        self.write_command_and_data(0x0034, 0x1202); // y control
        self.write_command_and_data(0x0035, 0x0D0A); // y control
        self.write_command_and_data(0x0036, 0x3506); // y control
        self.write_command_and_data(0x0037, 0x1107); // y control
        self.write_command_and_data(0x0038, 0x0005); // y control
        self.write_command_and_data(0x0039, 0x0212); // y control

        self.write_command_and_data(0x0090, 0x001D); // Panel I/F Control 1: DIVI=0, RTNI=0x1D (80Hz??)

        self.write_command_and_data(0x009C, 0x0043);

        self.write_command_and_data(0x0010, 0x0310); // Power Control 1: BT=2, AP=1, DSTB=0
        self.write_command_and_data(0x0011, 0x0231); // Power Control 2: DC1=2, DC0=3, VC=1
        self.write_command_and_data(0x0012, 0x01BC); // Power Control 3: VRH=0, VCMR=1, PSON=0, PON=0, VRH=0x0C
        self.write_command_and_data(0x0013, 0x1400); // Power Control 4: VDV=0x14,

        delay_ms(100);

        self.write_command_and_data(0x0001, 0x0500); // Driver Output Control 1: SM=1, SS=1
        self.write_command_and_data(0x0002, 0x0200); // LCD Driving Control: BC0=1, NW0=0
        self.write_command_and_data(0x0003, 0x1030); // Entry Mode: TRIREG=0, DFM=0, BGR=1, ORG=0, I/D=3, AM=0

        delay_ms(1);

        self.write_command_and_data(0x000A, 0x0008); // Display Control 4: FMARK0=1, FM=0

        self.write_command_and_data(0x0091, 0x0003); // Panel I/F Control 1-1: SPCWI=3
        self.write_command_and_data(0x0093, 0x0201); // Panel I/F Control 3: VEQWI=2, MCPI=1

        self.write_command_and_data(0x0007, 0x0100); // Display Control 1: BASEE=1
        delay_ms(35);
    }

    ///
    /// Sets display orientation
    ///
    #[allow(unused)]
    pub fn set_orientation(
        &mut self,
        orientation: Orientation,
    ) -> Result<(), DevError>
    where
        DI: WriteOnlyDataCommand,
    {
        match orientation {
            Orientation::Portrait => {
                //write_command_and_data(di, 0x03, 0x1030);
                delay_ms(30);
                self.write_command_and_data(Instruction::PosX as u8, 0);
                self.write_command_and_data(Instruction::PosY as u8, 0);
                self.write_command_and_data(Instruction::HSA as u8, PORTRAIT_ORIGIN_X);
                self.write_command_and_data(
                    Instruction::HEA as u8,
                    PORTRAIT_ORIGIN_X + PORTRAIT_AVAIL_WIDTH - 1,
                );
                self.write_command_and_data(
                    Instruction::VSA as u8,
                    PORTRAIT_TOTAL_HEIGHT - PORTRAIT_AVAIL_HEIGHT - PORTRAIT_ORIGIN_Y + 1,
                );
                self.write_command_and_data(
                    Instruction::VEA as u8,
                    PORTRAIT_TOTAL_HEIGHT - PORTRAIT_ORIGIN_Y,
                );
                self.write_command_and_data(0x03, 0x1030);
            }
            Orientation::Landscape => {
                self.write_command_and_data(0x03, 0x1098);
                self.write_command_and_data(Instruction::PosX as u8, 0);
                self.write_command_and_data(Instruction::PosY as u8, 0);
                self.write_command_and_data(Instruction::HSA as u8, LANDSCAPE_ORIGIN_Y);
                self.write_command_and_data(
                    Instruction::HEA as u8,
                    LANDSCAPE_ORIGIN_Y + LANDSCAPE_AVAIL_HEIGHT - 1,
                );
                self.write_command_and_data(
                    Instruction::VSA as u8,
                    LANDSCAPE_TOTAL_WIDTH - LANDSCAPE_AVAIL_WIDTH - LANDSCAPE_ORIGIN_X + 1,
                );
                self.write_command_and_data(
                    Instruction::VEA as u8,
                    LANDSCAPE_TOTAL_WIDTH - LANDSCAPE_ORIGIN_X,
                );
            }
        }
        Ok(())
    }

    #[inline]
    fn write_command(&mut self, cmd: u8) {
        let _ = self.di.send_commands(U8Iter(&mut once(cmd)));
    }
    
    #[inline]
    fn write_data(&mut self, data: u16) {
        let _ = self.di.send_data(U16BEIter(&mut once(data)));
    }
    
    #[inline]
    fn read_data(&self) -> u16 {
        // Safety: Reading u16 is atomic, so unsafe is ok
        unsafe { core::ptr::read_volatile(0x6002_0000 as *const u16) }
    }
    
    #[inline]
    fn write_command_and_data(&mut self, cmd: u8, data: u16) {
        self.write_command(cmd);
        self.write_data(data)
    }
}

impl <DI>SetRow for R61580<DI> 
where
    DI: WriteOnlyDataCommand
{
    fn set_row(&mut self, pos_x: u16, pos_y: u16, buf: &mut [u16]) {
        self.write_command_and_data(Instruction::PosX as u8, pos_x);
        self.write_command_and_data(Instruction::PosY as u8, pos_y);
        self.write_command(Instruction::Gram as u8);
        let _ = self.di.send_data(U16BE(buf));
    }
}
