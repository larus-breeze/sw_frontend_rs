use stm32h7xx_hal::gpio::{self, Speed};

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

pub struct Ltdc {}

impl Ltdc {
    pub fn init(ltdc_pins: LtdcPins) {
        ltdc_pins
            .0
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc0  -> LTDC R5
        ltdc_pins
            .1
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa3  -> LTDC B5
        ltdc_pins
            .2
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa4  -> LTDC VSYNC
        ltdc_pins
            .3
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa6  -> LTDC G2
        ltdc_pins
            .4
            .into_alternate::<9>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb0  -> LTDC R3
        ltdc_pins
            .5
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe11 -> LTDC G3
        ltdc_pins
            .6
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe12 -> LTDC B4
        ltdc_pins
            .7
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe13 -> LTDC DE
        ltdc_pins
            .8
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe14 -> LTDC CLK
        ltdc_pins
            .9
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe15 -> LTDC R7
        ltdc_pins
            .10
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb10 -> LTDC G4
        ltdc_pins
            .11
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb11 -> LTDC G5
        ltdc_pins
            .12
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pd10 -> LTDC B3
        ltdc_pins
            .13
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc6  -> LTDC HSYNC
        ltdc_pins
            .14
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc7  -> LTDC G6
        ltdc_pins
            .15
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc9  -> LTDC B2
        ltdc_pins
            .16
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa8  -> LTDC R6
        ltdc_pins
            .17
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa11 -> LTDC R4
        ltdc_pins
            .18
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc10 -> LTDC R2
        ltdc_pins
            .19
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pd3  -> LTDC G7
        ltdc_pins
            .20
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb8  -> LTDC B6
        ltdc_pins
            .21
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb9  -> LTDC B7
    }
}
