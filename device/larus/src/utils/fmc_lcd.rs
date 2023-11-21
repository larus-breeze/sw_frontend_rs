use stm32h7xx_hal::{
    gpio::{self, Speed},
    pac::{FMC, fmc}, 
    pac::fmc::{BCR1, BCR2, BCR3, BCR4,},
    pac::fmc::{BTR1, BTR2, BTR3, BTR4,},
    pac::fmc::{BWTR1, BWTR2, BWTR3, BWTR4,},
};
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

#[allow(dead_code)]
pub struct LcdInterface {
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

    dc: gpio::Pin<'F', 0, gpio::Alternate<12>>,
    wr: gpio::Pin<'D', 5, gpio::Alternate<12>>,
    noe: gpio::Pin<'D', 4, gpio::Alternate<12>>,
    cs: gpio::Pin<'D', 7, gpio::Alternate<12>>,
}

impl LcdInterface {
    /// Create new parallel GPIO interface for communication with a display driver
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fmc: &FMC,
        p0: gpio::Pin<'D', 14, gpio::Analog>,
        p1: gpio::Pin<'D', 15, gpio::Analog>,
        p2: gpio::Pin<'D', 0, gpio::Analog>,
        p3: gpio::Pin<'D', 1, gpio::Analog>,
        p4: gpio::Pin<'E', 7, gpio::Analog>,
        p5: gpio::Pin<'E', 8, gpio::Analog>,
        p6: gpio::Pin<'E', 9, gpio::Analog>,
        p7: gpio::Pin<'E', 10, gpio::Analog>,
        p8: gpio::Pin<'E', 11, gpio::Analog>,
        p9: gpio::Pin<'E', 12, gpio::Analog>,
        p10: gpio::Pin<'E', 13, gpio::Analog>,
        p11: gpio::Pin<'E', 14, gpio::Analog>,
        p12: gpio::Pin<'E', 15, gpio::Analog>,
        p13: gpio::Pin<'D', 8, gpio::Analog>,
        p14: gpio::Pin<'D', 9, gpio::Analog>,
        p15: gpio::Pin<'D', 10, gpio::Analog>,

        dc: gpio::Pin<'F', 0, gpio::Analog>,
        wr: gpio::Pin<'D', 5, gpio::Analog>,
        noe: gpio::Pin<'D', 4, gpio::Analog>,
        cs: gpio::Pin<'D', 7, gpio::Analog>,
    ) -> Self {
        let mut p0 = p0
            .into_pull_up_input()
            .into_alternate::<12>();
        p0.set_speed(Speed::VeryHigh);
        let mut p1 = p1
            .into_pull_up_input()
            .into_alternate::<12>();
        p1.set_speed(Speed::VeryHigh);
        let mut p2 = p2
            .into_pull_up_input()
            .into_alternate::<12>();
        p2.set_speed(Speed::VeryHigh);
        let mut p3 = p3
            .into_pull_up_input()
            .into_alternate::<12>();
        p3.set_speed(Speed::VeryHigh);
        let mut p4 = p4
            .into_pull_up_input()
            .into_alternate::<12>();
        p4.set_speed(Speed::VeryHigh);
        let mut p5 = p5
            .into_pull_up_input()
            .into_alternate::<12>();
        p5.set_speed(Speed::VeryHigh);
        let mut p6 = p6
            .into_pull_up_input()
            .into_alternate::<12>();
        p6.set_speed(Speed::VeryHigh);
        let mut p7 = p7
            .into_pull_up_input()
            .into_alternate::<12>();
        p7.set_speed(Speed::VeryHigh);
        let mut p8 = p8
            .into_pull_up_input()
            .into_alternate::<12>();
        p8.set_speed(Speed::VeryHigh);
        let mut p9 = p9
            .into_pull_up_input()
            .into_alternate::<12>();
        p9.set_speed(Speed::VeryHigh);
        let mut p10 = p10
            .into_pull_up_input()
            .into_alternate::<12>();
        p10.set_speed(Speed::VeryHigh);
        let mut p11 = p11
            .into_pull_up_input()
            .into_alternate::<12>();
        p11.set_speed(Speed::VeryHigh);
        let mut p12 = p12
            .into_pull_up_input()
            .into_alternate::<12>();
        p12.set_speed(Speed::VeryHigh);
        let mut p13 = p13
            .into_pull_up_input()
            .into_alternate::<12>();
        p13.set_speed(Speed::VeryHigh);
        let mut p14 = p14
            .into_pull_up_input()
            .into_alternate::<12>();
        p14.set_speed(Speed::VeryHigh);
        let mut p15 = p15
            .into_pull_up_input()
            .into_alternate::<12>();
        p15.set_speed(Speed::VeryHigh);

        let mut dc = dc
            .into_pull_up_input()
            .into_alternate::<12>();
        dc.set_speed(Speed::VeryHigh);
        let mut wr = wr
            .into_pull_up_input()
            .into_alternate::<12>();
        wr.set_speed(Speed::VeryHigh);
        let mut noe = noe
            .into_pull_up_input()
            .into_alternate::<12>();
        noe.set_speed(Speed::VeryHigh);
        let mut cs = cs
            .into_pull_up_input()
            .into_alternate::<12>();
        cs.set_speed(Speed::VeryHigh);

        fn config_bcr(bcr: &fmc::BCR1) {
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
        let bcr1 = &fmc.bcr1;
        let bcr2 = unsafe { core::mem::transmute::<&BCR2, &BCR1>(&fmc.bcr2) };
        let bcr3 = unsafe { core::mem::transmute::<&BCR3, &BCR1>(&fmc.bcr3) };
        let bcr4 = unsafe { core::mem::transmute::<&BCR4, &BCR1>(&fmc.bcr4) };
        config_bcr(bcr1); // unsafe: bcrxx are identical
        config_bcr(bcr2);
        config_bcr(bcr3);
        config_bcr(bcr4);

        fn config_btr(btr: &fmc::BTR1) {
            btr.modify(|_, w| unsafe { w
                .accmod()
                .bits(00) // Mode A
                .datlat()
                .bits(2)
                .clkdiv()
                .bits(2)
                .busturn()
                .bits(1)
                .addhld()
                .bits(1)
                .addset()
                .bits(3)
            });
        }
        let btr1 = &fmc.btr1;
        let btr2 = unsafe { core::mem::transmute::<&BTR2, &BTR1>(&fmc.btr2)};
        let btr3 = unsafe { core::mem::transmute::<&BTR3, &BTR1>(&fmc.btr3)};
        let btr4 = unsafe { core::mem::transmute::<&BTR4, &BTR1>(&fmc.btr4)};
        config_btr(btr1);
        config_btr(btr2);
        config_btr(btr3);
        config_btr(btr4);

        fn config_bwtr(bwtr: &fmc::BWTR1) {
            bwtr.modify(|_, w| unsafe { w
                    .accmod()
                    .bits(00) // Mode A
                    .datast()
                    .bits(4)
                    .busturn()
                    .bits(1)
                    .addhld()
                    .bits(1)
                    .addset()
                    .bits(3)
            });
        }
        let bwtr1 = &fmc.bwtr1;
        let bwtr2 = unsafe { core::mem::transmute::<&BWTR2, &BWTR1>(&fmc.bwtr2)};
        let bwtr3 = unsafe { core::mem::transmute::<&BWTR3, &BWTR1>(&fmc.bwtr3)};
        let bwtr4 = unsafe { core::mem::transmute::<&BWTR4, &BWTR1>(&fmc.bwtr4)};
        config_bwtr(bwtr1);
        config_bwtr(bwtr2);
        config_bwtr(bwtr3);
        config_bwtr(bwtr4);

        Self {
            p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15,
            dc, wr, noe, cs,
        }
    }

    fn send_command_u8(&mut self, command: u8) {
        unsafe {
            core::ptr::write_volatile(0x6800_0000 as *mut u8, command);
        }
    }
    fn send_command_u16(&mut self, command: u16) {
        unsafe {
            core::ptr::write_volatile(0x6800_0000 as *mut u16, command);
        }
    }
    fn send_data_u8(&mut self, data: u8) {
        unsafe {
            core::ptr::write_volatile(0x6800_0002 as *mut u8, data);
        }
    }
    fn send_data_u16(&mut self, data: u16) {
        unsafe {
            core::ptr::write_volatile(0x6800_0002 as *mut u16, data);
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