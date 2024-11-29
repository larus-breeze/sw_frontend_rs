// This data is for initializing the st7701 driver:
// 0x00xx   cmd xx
// 0x01xx   data xx

const INIT_SEQ_1: [u16; 206] = [
    0x00FF, // cmd Command2 BKx Selection
    0x0177, 0x0101, 0x0100, 0x0100, 0x0110, // CN2 = 1 BKx = 0
    0x00C0, // cmd Display Line Setting
    0x013B, 0x0100, 0x00C1, // cmd Porch Control
    0x010B, // VBP
    0x0102, 0x00C2, // cmd Inversion selection & Frame Rate Control
    0x0100, 0x0102, 0x00CC, // cmd ??? CC is a BK3 Command NVM Program Active
    0x0110, 0x00CD, // cmd Color Control
    0x0108, 0x00B0, // cmd Positive Voltage Gamma Control
    0x0102, 0x0113, 0x011B, 0x010D, 0x0110, 0x0105, 0x0108, 0x0107, 0x0107, 0x0124, 0x0104, 0x0111,
    0x010E, 0x012C, 0x0133, 0x011D, 0x00B1, // cmd Negative Voltage Gamma Control
    0x0105, 0x0113, 0x011B, 0x010D, 0x0111, 0x0105, 0x0108, 0x0107, 0x0107, 0x0124, 0x0104, 0x0111,
    0x010E, 0x012C, 0x0133, 0x011D, 0x00FF, // cmd Command2 BKx Selection
    0x0177, 0x0101, 0x0100, 0x0100, 0x0111, // CN2 = 1 BKx = 1
    0x00B0, // cmd VOP amplitude setting
    0x015d, // 5d
    0x00B1, // cmd VCOM amplitude setting
    0x0143, // 43
    0x00B2, // cmd VGH Voltage setting
    0x0181, // 12V
    0x00B3, // cmd Test Command Setting
    0x0180, 0x00B5, // cmd VGL Voltage setting
    0x0143, // -8.3V
    0x00B7, // cmd Power Control 1
    0x0185, 0x00B8, // cmd Power Control 2
    0x0120, 0x00C1, // cmd Source pre_drive timing set1
    0x0178, 0x00C2, // cmd Source pre_drive timing set2
    0x0178, 0x00D0, // cmd MIPI Setting 1
    0x0188, 0x00E0, // cmd ??? BK0 Sunlight Readable Enhancement
    0x0100, 0x0100, 0x0102, 0x00E1, // cmd ??? BK0 Noise Reduce Control
    0x0103, 0x01A0, 0x0100, 0x0100, 0x0104, 0x01A0, 0x0100, 0x0100, 0x0100, 0x0120, 0x0120,
    0x00E2, // cmd ??? BK0 Sharpnes Control
    0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100, 0x0100,
    0x0100, 0x00E3, // cmd ??? BK0 Color Calibration Control
    0x0100, 0x0100, 0x0111, 0x0100, 0x00E4, // cmd ??? BKß Skin Tone Preservation Control
    0x0122, 0x0100, 0x00E5, // cmd ??? E5 undefined
    0x0105, 0x01EC, 0x01A0, 0x01A0, 0x0107, 0x01EE, 0x01A0, 0x01A0, 0x0100, 0x0100, 0x0100, 0x0100,
    0x0100, 0x0100, 0x0100, 0x0100, 0x00E6, // cmd ??? E6 undefined
    0x0100, 0x0100, 0x0111, 0x0100, 0x00E7, // cmd ??? E7 undefined
    0x0122, 0x0100, 0x00E8, // cmd ??? E8 undefined
    0x0106, 0x01ED, 0x01A0, 0x01A0, 0x0108, 0x01EF, 0x01A0, 0x01A0, 0x0100, 0x0100, 0x0100, 0x0100,
    0x0100, 0x0100, 0x0100, 0x0100, 0x00EB, // cmd ??? EB undefined
    0x0100, 0x0100, 0x0140, 0x0140, 0x0100, 0x0100, 0x0100, 0x00ED, // cmd ??? undefined
    0x01FF, 0x01FF, 0x01FF, 0x01BA, 0x010A, 0x01BF, 0x0145, 0x01FF, 0x01FF, 0x0154, 0x01FB, 0x01A0,
    0x01AB, 0x01FF, 0x01FF, 0x01FF, 0x00EF, // cmd ??? undefined
    0x0110, 0x010D, 0x0104, 0x0108, 0x013F, 0x011F, 0x00FF, // cmd BK3 Selected
    0x0177, 0x0101, 0x0100, 0x0100, 0x0113, 0x00EF, // cmd ??? undefined
    0x0108, 0x00FF, // cmd BK0 Selected
    0x0177, 0x0101, 0x0100, 0x0100, 0x0100, 0x0011, // cmd ??? undefined
];

const INIT_SEQ_2: [u16; 5] = [
    0x0029, // cmd ??? undefined
    0x0036, // cmd ??? undefined
    0x0100, 0x003A, // cmd ??? undefined
    0x0160,
];

use stm32h7xx_hal::{
    delay::Delay,
    gpio, pac,
    prelude::*,
    rcc::{rec, CoreClocks},
    spi,
};

pub struct LcdPins(
    pub gpio::Pin<'C', 1>,  // SPI MISO
    pub gpio::Pin<'C', 2>,  // SPI MOSI
    pub gpio::Pin<'A', 12>, // SPI SLK
    pub gpio::Pin<'D', 14>, // SPI_CS
    pub gpio::Pin<'E', 8>,  // LCD Reset
);

pub struct St7701s {}

impl St7701s {
    pub fn init(
        spi2: pac::SPI2,
        prec: rec::Spi2,
        lcd_pins: LcdPins,
        clocks: &CoreClocks,
        delay: &mut Delay,
    ) {
        let (mosi, miso, sck, mut cs, mut reset) = (
            lcd_pins.0.into_alternate(),
            lcd_pins.1.into_alternate(),
            lcd_pins.2.into_alternate(),
            lcd_pins.3.into_push_pull_output(),
            lcd_pins.4.into_push_pull_output(),
        );

        // reset LCD
        reset.set_high();
        delay.delay_ms(10_u16);
        reset.set_low();
        delay.delay_ms(10_u16);
        reset.set_high();
        delay.delay_ms(10_u16);

        // init spi2 peripheral
        let mut spi_: spi::Spi<pac::SPI2, spi::Enabled, u16> =
            spi2.spi((sck, miso, mosi), spi::MODE_0, 3.MHz(), prec, clocks);

        // set to 9 bits per spi frame
        unsafe {
            let spi2_ptr = &(*pac::SPI2::ptr());
            spi2_ptr.cfg1.modify(|_, w| w.dsize().bits(8));
        }

        // send init stream to lcd part 1
        cs.set_low();
        spi_.write(&INIT_SEQ_1).unwrap();
        cs.set_high();
        delay.delay_ms(120_u16);

        // send init stream to lcd part 2
        cs.set_low();
        spi_.write(&INIT_SEQ_2).unwrap();
        cs.set_high();
        delay.delay_ms(200_u16);
    }
}
