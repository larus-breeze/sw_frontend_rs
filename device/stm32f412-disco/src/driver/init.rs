
use cortex_m_rt::entry;

use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::fsmc_lcd::{ChipSelect1, FsmcLcd, LcdPins, Timing};
use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;
use {defmt_rtt as _, panic_probe as _};

use crate::driver;
use crate::main;

#[entry]
fn init() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    // Make HCLK faster to allow updating the display more quickly
    let clocks = rcc.cfgr
        .use_hse(16.mhz())
        .sysclk(100.mhz())
        .hclk(100.mhz())
        .freeze();

    let mut cp = pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, &clocks);

    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();
    let gpiof = dp.GPIOF.split();

    // Pins connected to the LCD on the 32F412GDISCOVERY board
    let lcd_pins = LcdPins {
        data: (
            gpiod.pd14.into_alternate(),
            gpiod.pd15.into_alternate(),
            gpiod.pd0.into_alternate(),
            gpiod.pd1.into_alternate(),
            gpioe.pe7.into_alternate(),
            gpioe.pe8.into_alternate(),
            gpioe.pe9.into_alternate(),
            gpioe.pe10.into_alternate(),
            gpioe.pe11.into_alternate(),
            gpioe.pe12.into_alternate(),
            gpioe.pe13.into_alternate(),
            gpioe.pe14.into_alternate(),
            gpioe.pe15.into_alternate(),
            gpiod.pd8.into_alternate(),
            gpiod.pd9.into_alternate(),
            gpiod.pd10.into_alternate(),
        ),
        address: gpiof.pf0.into_alternate(),
        read_enable: gpiod.pd4.into_alternate(),
        write_enable: gpiod.pd5.into_alternate(),
        chip_select: ChipSelect1(gpiod.pd7.into_alternate()),
    };
    let lcd_reset = gpiod.pd11.into_push_pull_output();
    let mut backlight_control = gpiof.pf5.into_push_pull_output();

    // Speed up timing settings, assuming HCLK is 100 MHz (1 cycle = 10 nanoseconds)
    // These read timings work to read settings, but slower timings are needed to read from the
    // frame memory.
    // Read timing: RD can go low at the same time as D/C changes and CS goes low.
    // RD must be low for at least 45 ns -> DATAST=8
    // Also, a read cycle must take at least 160 nanoseconds, so set ADDSET=8. This means
    // that a whole read takes 16 HCLK cycles (160 nanoseconds).
    // Bus turnaround time is zero, because no particular interval is required between transactions.
    let read_timing = Timing::default().data(8).address_setup(8).bus_turnaround(0);
    // Write timing: Minimum 10 nanoseconds from when WR goes high to CS goes high, so
    // HCLK can't be faster than 100 MHz.
    // NWE must be low for at least 15 ns -> DATAST=3
    // A write cycle must take at least 66 nanoseconds, so ADDSET=3. This means that a whole
    // write cycle takes 7 HCLK cycles (70 nanoseconds) (an extra HCLK cycle is added after NWE
    // goes high).
    // Bus turnaround time is zero, because no particular interval is required between transactions.
    let write_timing = Timing::default().data(3).address_setup(3).bus_turnaround(0);

    let (_fsmc, interface) = FsmcLcd::new(dp.FSMC, lcd_pins, &read_timing, &write_timing);

    // The 32F412GDISCOVERY board has an FRD154BP2902-CTP LCD. There is no easily available
    // datasheet, so the behavior of this code is based on the working demonstration C code:
    // https://github.com/STMicroelectronics/STM32CubeF4/blob/e084518f363e04344dc37822210a75e87377b200/Drivers/BSP/STM32412G-Discovery/stm32412g_discovery_lcd.c
    // https://github.com/STMicroelectronics/STM32CubeF4/blob/e084518f363e04344dc37822210a75e87377b200/Drivers/BSP/Components/st7789h2/st7789h2.c

    let mut lcd = driver::ST7789::new(interface, lcd_reset);
    // Initialise the display, clear the screen and turn on the backlight
    lcd.init(&mut delay).unwrap();
    let mut display = driver::Display::new(&mut lcd);
    backlight_control.set_high();

    main(&mut display, &mut delay);

    // Should never come here
    loop {}
}
