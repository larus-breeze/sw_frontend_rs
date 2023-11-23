mod timing;

use stm32h7xx_hal::{
    gpio,
    pac::{FMC as FSMC, fmc as fsmc},
//    rcc::{Enable, Reset},
};
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use timing::*;
pub use timing::{AccessMode, Timing};

macro_rules! config_pin {
    ($pin:expr) => {
        {
            let mut p = $pin.into_pull_up_input().into_alternate::<12>();
            p.set_speed(gpio::Speed::VeryHigh);
            p
        }
    };
}

#[allow(dead_code)]
pub struct DataPins16 {
    p0: gpio::Pin<'D', 14, gpio::Alternate<12>>,
    p1: gpio::Pin<'D', 15, gpio::Alternate<12>>,
    p2: gpio::Pin<'D', 0, gpio::Alternate<12>>,
    p3: gpio::Pin<'D', 1, gpio::Alternate<12>>,
    p4: gpio::Pin<'E', 7, gpio::Alternate<12>>,
    p5: gpio::Pin<'E', 8, gpio::Alternate<12>>,
    p6: gpio::Pin<'E', 9, gpio::Alternate<12>>,
    p7: gpio::Pin<'E', 10, gpio::Alternate<12>>,
    p8: gpio::Pin<'E', 11, gpio::Alternate<12>>,
    p9: gpio::Pin<'E', 12, gpio::Alternate<12>>,
    p10: gpio::Pin<'E', 13, gpio::Alternate<12>>,
    p11: gpio::Pin<'E', 14, gpio::Alternate<12>>,
    p12: gpio::Pin<'E', 15, gpio::Alternate<12>>,
    p13: gpio::Pin<'D', 8, gpio::Alternate<12>>,
    p14: gpio::Pin<'D', 9, gpio::Alternate<12>>,
    p15: gpio::Pin<'D', 10, gpio::Alternate<12>>,
}
impl DataPins16 {
    pub fn new(
        p0: gpio::Pin<'D', 14>,
        p1: gpio::Pin<'D', 15>,
        p2: gpio::Pin<'D', 0>,
        p3: gpio::Pin<'D', 1>,
        p4: gpio::Pin<'E', 7>,
        p5: gpio::Pin<'E', 8>,
        p6: gpio::Pin<'E', 9>,
        p7: gpio::Pin<'E', 10>,
        p8: gpio::Pin<'E', 11>,
        p9: gpio::Pin<'E', 12>,
        p10: gpio::Pin<'E', 13>,
        p11: gpio::Pin<'E', 14>,
        p12: gpio::Pin<'E', 15>,
        p13: gpio::Pin<'D', 8>,
        p14: gpio::Pin<'D', 9>,
        p15: gpio::Pin<'D', 10>,
    ) -> Self {
        let p0 = config_pin!(p0);
        let p1 = config_pin!(p1);
        let p2 = config_pin!(p2);
        let p3 = config_pin!(p3);
        let p4 = config_pin!(p4);
        let p5 = config_pin!(p5);
        let p6 = config_pin!(p6);
        let p7 = config_pin!(p7);
        let p8 = config_pin!(p8);
        let p9 = config_pin!(p9);
        let p10 = config_pin!(p10);
        let p11 = config_pin!(p11);
        let p12 = config_pin!(p12);
        let p13 = config_pin!(p13);
        let p14 = config_pin!(p14);
        let p15 = config_pin!(p15);
        DataPins16 { 
            p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15 }
    }
}

#[allow(dead_code)]
pub struct LcdPins {
    data_pins: DataPins16,
    address: Address,
    read_enable: ReadEnable,
    write_enable: WriteEnable,
    chip_select: ChipSelect,
}

impl LcdPins {
    pub fn new(
        data_pins: DataPins16,
        address: gpio::Pin<'D', 11>,
        read_enable: gpio::Pin<'D', 4>,
        write_enable: gpio::Pin<'D', 5>,
        chip_select: gpio::Pin<'D', 7>,
    ) -> Self {
        let address = config_pin!(address);
        let read_enable = config_pin!(read_enable);
        let write_enable = config_pin!(write_enable);
        let chip_select = config_pin!(chip_select);
        LcdPins {data_pins, address, read_enable, write_enable, chip_select}
    }
}


type Address = gpio::Pin<'D', 11, gpio::Alternate<12>>;
type ReadEnable = gpio::Pin<'D', 4, gpio::Alternate<12>>;
type WriteEnable = gpio::Pin<'D', 5, gpio::Alternate<12>>;
type ChipSelect = gpio::Pin<'D', 7, gpio::Alternate<12>>;

#[allow(dead_code)]
pub struct LcdInterface {
    lcd_pins: LcdPins,
}

impl LcdInterface {
    /// Create new parallel GPIO interface for communication with a display driver
    #[allow(clippy::too_many_arguments)]
    pub fn new<'a>(
        fsmc: FSMC,
        lcd_pins: LcdPins,
        read_timing: &'a Timing,
        write_timing: &'a Timing,
    ) -> Self {
        config_bcr(&fsmc.bcr1);
        let bcr = unsafe {core::mem::transmute::<&fsmc::BCR2, &fsmc::BCR1>(&fsmc.bcr2)};
        config_bcr(bcr);
        let bcr = unsafe {core::mem::transmute::<&fsmc::BCR3, &fsmc::BCR1>(&fsmc.bcr3)};
        config_bcr(bcr);
        let bcr = unsafe {core::mem::transmute::<&fsmc::BCR4, &fsmc::BCR1>(&fsmc.bcr4)};
        config_bcr(bcr);

        config_btr(&fsmc.btr1, read_timing);
        let btr = unsafe {core::mem::transmute::<&fsmc::BTR2, &fsmc::BTR1>(&fsmc.btr2)};
        config_btr(btr, read_timing);
        let btr = unsafe {core::mem::transmute::<&fsmc::BTR3, &fsmc::BTR1>(&fsmc.btr3)};
        config_btr(btr, read_timing);
        let btr = unsafe {core::mem::transmute::<&fsmc::BTR4, &fsmc::BTR1>(&fsmc.btr4)};
        config_btr(btr, read_timing);

        config_bwtr(&fsmc.bwtr1, write_timing);
        let bwtr = unsafe {core::mem::transmute::<&fsmc::BWTR2, &fsmc::BWTR1>(&fsmc.bwtr2)};
        config_bwtr(bwtr, write_timing);
        let bwtr = unsafe {core::mem::transmute::<&fsmc::BWTR3, &fsmc::BWTR1>(&fsmc.bwtr3)};
        config_bwtr(bwtr, write_timing);
        let bwtr = unsafe {core::mem::transmute::<&fsmc::BWTR4, &fsmc::BWTR1>(&fsmc.bwtr4)};
        config_bwtr(bwtr, write_timing);

        Self { lcd_pins }
    }

    fn send_command_u8(&mut self, command: u8) {
        unsafe {
            core::ptr::write_volatile(0x6000_0000 as *mut u8, command);
        }
    }
    fn send_command_u16(&mut self, command: u16) {
        unsafe {
            core::ptr::write_volatile(0x6000_0000 as *mut u16, command);
        }
    }
    fn send_data_u8(&mut self, data: u8) {
        unsafe {
            core::ptr::write_volatile(0x6002_0000 as *mut u8, data);
        }
    }
    fn send_data_u16(&mut self, data: u16) {
        unsafe {
            core::ptr::write_volatile(0x6002_0000 as *mut u16, data);
        }
    }
}

impl WriteOnlyDataCommand for LcdInterface {
    fn send_commands(&mut self, cmds: DataFormat<'_>) -> Result<(), DisplayError> {
        match cmds {
            DataFormat::U8(slice) => {
                for cmd in slice {
                    self.send_command_u8(*cmd)
                }
            }
            DataFormat::U8Iter(iter) => {
                for cmd in iter {
                    self.send_command_u8(cmd)
                }
            }
            DataFormat::U16(slice) => {
                for cmd in slice {
                    self.send_command_u16(*cmd)
                }
            }
            DataFormat::U16BE(slice) | DataFormat::U16LE(slice) => {
                for cmd in slice {
                    self.send_command_u16(*cmd)
                }
            }
            DataFormat::U16BEIter(iter) | DataFormat::U16LEIter(iter) => {
                for cmd in iter {
                    self.send_command_u16(cmd)
                }
            }
            _ => Err(display_interface::DisplayError::DataFormatNotImplemented)?,
        }
        Ok(())
    }

    fn send_data(&mut self, buf: DataFormat) -> Result<(), DisplayError> {
        match buf {
            DataFormat::U8(slice) => {
                for d in slice {
                    self.send_data_u8(*d)
                }
            }
            DataFormat::U8Iter(iter) => {
                for d in iter {
                    self.send_data_u8(d)
                }
            }
            DataFormat::U16(slice) => {
                for d in slice {
                    self.send_data_u16(*d)
                }
            }
            DataFormat::U16BE(slice) | DataFormat::U16LE(slice) => {
                for d in slice {
                    self.send_data_u16(*d)
                }
            }
            DataFormat::U16BEIter(iter) | DataFormat::U16LEIter(iter) => {
                for d in iter {
                    self.send_data_u16(d)
                }
            }
            _ => Err(display_interface::DisplayError::DataFormatNotImplemented)?,
        }
        Ok(())
    }
}

fn config_bcr(bcr: &fsmc::BCR1) {
    bcr.write(|w| unsafe { w
        // The write fifo and WFDIS bit are missing from some models.
        // Where present, the FIFO is enabled by default.
        // ------------
        // Disable synchronous writes
        .cburstrw()
        .clear_bit()
        // .disabled()
        // Don't split burst transactions (doesn't matter for LCD mode)
        .cpsize()
        .bits(0)
        //                .no_burst_split()
        // Ignore wait signal (asynchronous mode)
        .asyncwait()
        .clear_bit()
        //.disabled()
        // Enable extended mode, for different read and write timings
        .extmod()
        .set_bit()
        //.enabled()
        // Ignore wait signal (synchronous mode)
        .waiten()
        .clear_bit()
        //.disabled()
        // Allow write operations
        .wren()
        .set_bit()
        //.enabled()
        // Default wait timing
        .waitcfg()
        .clear_bit()
        //.before_wait_state()
        // Default wait polarity
        .waitpol()
        .clear_bit()
        //.active_low()
        // Disable burst reads
        .bursten()
        .clear_bit()
        //.disabled()
        // Enable NOR flash operations
        .faccen()
        .set_bit()
        //.enabled()
        // 16-bit bus width
        .mwid()
        .bits(1)
        //.bits16()
        // NOR flash mode (compatible with LCD controllers)
        .mtyp()
        .bits(2)
        //.flash()
        // Address and data not multiplexed
        .muxen()
        .clear_bit()
        //.disabled()
        // Enable this memory bank
        .mbken()
        .set_bit()
        //.enabled()
    });            
}
