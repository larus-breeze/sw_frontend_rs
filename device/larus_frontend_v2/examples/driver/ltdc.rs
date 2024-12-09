use super::{TBuffer, AVAIL_PIXELS, CLUT_COLORS};
use core::ptr::addr_of;
use defmt::info;
use stm32h7xx_hal::{
    delay::Delay,
    gpio::{self, Speed},
    pac,
    rcc::{rec, CoreClocks, ResetEnable},
};

pub struct LtdcPins(
    pub gpio::Pin<'C', 0>,
    pub gpio::Pin<'A', 3>,
    pub gpio::Pin<'A', 4>,
    pub gpio::Pin<'A', 6>,
    pub gpio::Pin<'B', 0>,
    pub gpio::Pin<'E', 11>,
    pub gpio::Pin<'E', 12>,
    pub gpio::Pin<'E', 13>,
    pub gpio::Pin<'E', 14>,
    pub gpio::Pin<'E', 15>,
    pub gpio::Pin<'B', 10>,
    pub gpio::Pin<'B', 11>,
    pub gpio::Pin<'D', 10>,
    pub gpio::Pin<'C', 6>,
    pub gpio::Pin<'C', 7>,
    pub gpio::Pin<'C', 9>,
    pub gpio::Pin<'A', 8>,
    pub gpio::Pin<'A', 11>,
    pub gpio::Pin<'C', 10>,
    pub gpio::Pin<'D', 3>,
    pub gpio::Pin<'B', 8>,
    pub gpio::Pin<'B', 9>,
);

pub const DISPLAY_HEIGHT: usize = 480;
pub const DISPLAY_WIDTH: usize = 480;
pub const PIX_AVAIL: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;

pub struct Ltdc {}

impl Ltdc {
    pub fn init(
        ltdc: pac::LTDC,
        ltdc_pins: LtdcPins,
        prec: rec::Ltdc,
        clocks: &CoreClocks,
        delay: &mut Delay,
    ) -> Self {
        // The pll3_r_ck (== pixel clock) of 9 Mhz leads to a frame rate of slightly more
        // than 30 Hz
        let _ = clocks.pll3_r_ck().unwrap(); // pll3 must run
        prec.enable().reset(); // enable peripheral

        ltdc_pins
            .0
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pc0  -> LTDC R5
        ltdc_pins
            .1
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pa3  -> LTDC B5
        ltdc_pins
            .2
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pa4  -> LTDC VSYNC
        ltdc_pins
            .3
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pa6  -> LTDC G2
        ltdc_pins
            .4
            .into_alternate::<9>()
            .speed(Speed::High)
            .internal_pull_up(true); // pb0  -> LTDC R3
        ltdc_pins
            .5
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pe11 -> LTDC G3
        ltdc_pins
            .6
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pe12 -> LTDC B4
        ltdc_pins
            .7
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pe13 -> LTDC DE
        ltdc_pins
            .8
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pe14 -> LTDC CLK
        ltdc_pins
            .9
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pe15 -> LTDC R7
        ltdc_pins
            .10
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pb10 -> LTDC G4
        ltdc_pins
            .11
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pb11 -> LTDC G5
        ltdc_pins
            .12
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pd10 -> LTDC B3
        ltdc_pins
            .13
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pc6  -> LTDC HSYNC
        ltdc_pins
            .14
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pc7  -> LTDC G6
        ltdc_pins
            .15
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pc9  -> LTDC B2
        ltdc_pins
            .16
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pa8  -> LTDC R6
        ltdc_pins
            .17
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pa11 -> LTDC R4
        ltdc_pins
            .18
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pc10 -> LTDC R2
        ltdc_pins
            .19
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pd3  -> LTDC G7
        ltdc_pins
            .20
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pb8  -> LTDC B6
        ltdc_pins
            .21
            .into_alternate::<14>()
            .speed(Speed::High)
            .internal_pull_up(true); // pb9  -> LTDC B7

        // unsafe is unavoidable and ok during initialization of the hardware
        //
        // the details are taken from
        // https://github.com/larus-breeze/sw_frontend/tree/hw_frontend_stm32h743_hal_lqfp100_rgb18_st7701
        unsafe {
            // configure ltdc peripheral
            let ltdc = &(*pac::LTDC::ptr());
            ltdc.sscr.write(|w| w.vsh().bits(3));
            ltdc.sscr.modify(|_, w| w.hsw().bits(5));

            ltdc.bpcr.write(|w| w.avbp().bits(0x0d));
            ltdc.bpcr.modify(|_, w| w.ahbp().bits(0x17));

            ltdc.awcr.write(|w| w.aah().bits(0x01ed));
            ltdc.awcr.modify(|_, w| w.aaw().bits(0x01f7));

            ltdc.twcr.write(|w| w.totalh().bits(0x01fd));
            ltdc.twcr.modify(|_, w| w.totalw().bits(0x020f));

            ltdc.ier
                .modify(|_, w| w.terrie().set_bit().fuie().set_bit());

            ltdc.gcr.write(|w| w.pcpol().set_bit());

            Ltdc {}
        }
    }

    pub fn init_layer(&mut self, frame_bauffer: *const u8) {
        unsafe {
            // write color lookup table
            let ltdc = &(*pac::LTDC::ptr());
            for clut_entry in CLUT_COLORS {
                ltdc.layer1.clutwr.write(|w| w.bits(clut_entry));
            }
            ltdc.layer1.cr.modify(|_, w| w.cluten().enabled()); // enable clut
            ltdc.srcr.write(|w| w.imr().set_bit());

            // configure the layer used
            ltdc.layer1.whpcr.write(|w| w.bits(0x01f7_0018));
            ltdc.layer1.wvpcr.write(|w| w.bits(0x01ed_000e));
            ltdc.layer1.pfcr.write(|w| w.bits(0x05));
            ltdc.layer1.bfcr.write(|w| w.bits(0x0000_0405));
            ltdc.layer1.cfblr.write(|w| w.bits(0x01e0_01e7));
            ltdc.layer1.cfblnr.write(|w| w.bits(0x0000_01e0));

            ltdc.layer1
                .cfbar
                .write(|w| w.cfbadd().bits(frame_bauffer as u32));
            ltdc.layer1.cr.modify(|_, w| w.len().enabled()); // enable layer 1
            ltdc.srcr.write(|w| w.imr().set_bit());

            // activate ltdc peripheral
            ltdc.gcr.modify(|_, w| w.ltdcen().set_bit()); // enable ltdc
        }
    }

    pub fn set_frame_buffer(&mut self, frame_bauffer: *const u8) {
        unsafe {
            let ltdc = &(*pac::LTDC::ptr());
            ltdc.layer1
                .cfbar
                .write(|w| w.cfbadd().bits(frame_bauffer as u32));
            ltdc.srcr.write(|w| w.vbr().set_bit());
        }
    }
}
