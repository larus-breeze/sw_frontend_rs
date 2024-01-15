#![no_main]
#![no_std]
mod driver;

use defmt::trace;
use defmt_rtt as _;


use corelib::Event::KeyItem;
use corelib::*;
use cortex_m_rt::entry;
use heapless::mpmc::MpMcQueue;
use stm32h7xx_hal::{
    pac, prelude::*, 
    independent_watchdog::IndependentWatchdog,
};

use driver::*;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    trace!("init");

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    // This queue routes the events to the controller.
    static Q_EVENTS: QEvents = MpMcQueue::new();

    let mut keyboard = {
        let keyboard_pins = KeyboardPins::new(gpioe.pe5, gpioe.pe6, gpioe.pe4);
        let enc1_res = Enc1Res::new(ccdr.peripheral.TIM5, dp.TIM5, gpioa.pa0, gpioa.pa1);
        let enc2_res = Enc2Res::new(ccdr.peripheral.TIM3, dp.TIM3, gpiob.pb4.into(), gpioa.pa7);
        Keyboard::new(keyboard_pins, enc1_res, enc2_res, &Q_EVENTS)
    };

    let mut watchdog = IndependentWatchdog::new(dp.IWDG);
    watchdog.start(1000.millis());

    loop {
        delay.delay_ms(20_u16);
        watchdog.feed();
        keyboard.tick();
        while let Some(event) = Q_EVENTS.dequeue() {
            if let KeyItem(key_event) = event {
                match key_event {
                    KeyEvent::Btn1 => {
                        trace!("Do not feed the watchdog!");
                        loop {};
                    },
                    KeyEvent::Btn2 => {
                        trace!("Panic!");
                        panic!();
                    },
                    KeyEvent::BtnEnc => trace!("BtnEnc"),
                    KeyEvent::Rotary1Left => trace!("Rotary1Left"),
                    KeyEvent::Rotary1Right => trace!("Rotary1Right"),
                    KeyEvent::Rotary2Left => trace!("Rotary2Left"),
                    KeyEvent::Rotary2Right => trace!("Rotary2Right"),
                    _ => (),
                }
            }
        }
    }
}
