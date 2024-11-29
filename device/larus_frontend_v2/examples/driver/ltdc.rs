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
    pub gpio::Pin<'C', 10>,
);

pub struct Ltdc {}

impl Ltdc {
    pub fn init(ltdc_pins: LtdcPins) {
        ltdc_pins
            .0
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc0
        ltdc_pins
            .1
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa3
        ltdc_pins
            .2
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa4
        ltdc_pins
            .3
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa6
        ltdc_pins
            .4
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb0
        ltdc_pins
            .5
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe11
        ltdc_pins
            .6
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe12
        ltdc_pins
            .7
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe13
        ltdc_pins
            .8
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe14
        ltdc_pins
            .9
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pe15
        ltdc_pins
            .10
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb10
        ltdc_pins
            .11
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb11
        ltdc_pins
            .12
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pd10
        ltdc_pins
            .13
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc6
        ltdc_pins
            .14
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc7
        ltdc_pins
            .15
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc9
        ltdc_pins
            .16
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa8
        ltdc_pins
            .17
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pa11
        ltdc_pins
            .18
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pc10
        ltdc_pins
            .19
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pd3
        ltdc_pins
            .20
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb8
        ltdc_pins
            .21
            .into_alternate::<14>()
            .speed(Speed::High)
            .into_push_pull_output(); // pb9
    }
}
