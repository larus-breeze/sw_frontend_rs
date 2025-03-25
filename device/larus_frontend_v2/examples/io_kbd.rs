#![no_main]
#![no_std]

mod driver;

use defmt::*;
use defmt_rtt as _;

use heapless::mpmc::MpMcQueue;
use cortex_m_rt::entry;

use stm32h7xx_hal::{pac, prelude::*};

use corelib::{Event, InputPinState};
use driver::*;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    info!("init");

    let ccdr = set_clocksys!(dp, cp);
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);

    let mut delay = cp.SYST.delay(ccdr.clocks);

    static Q_EVENTS: QEvents = MpMcQueue::new();

    let mut keyboard = {
        let keyboard_pins = KeyboardPins::new(gpioa.pa3);
        let input_pins = InputPins::new(gpiob.pb12, gpiob.pb13, gpiob.pb14, gpioh.ph7);
        let enc1_res = Enc1Res::new(ccdr.peripheral.TIM5, dp.TIM5, gpioa.pa0, gpioa.pa1);
        let enc2_res = Enc2Res::new(ccdr.peripheral.TIM3, dp.TIM3, gpiob.pb4.into(), gpioc.pc7);
        Keyboard::new(
            keyboard_pins,
            input_pins, 
            enc1_res, 
            enc2_res, 
            &Q_EVENTS
        )
    };

    loop {
        delay.delay_ms(20_u16);
        keyboard.tick();

        while let Some(event) = &Q_EVENTS.dequeue() {
            match event {
                Event::KeyItem(key_event) => info!("KeyItem {}", *key_event as u32),
                Event::DeviceItem(_) => info!("DeviceItem"),
                Event::InputItem(io) => match io {
                    InputPinState::Io1(state) => info!("Input 1 state {}", state),
                    InputPinState::Io2(state) => info!("Input 2 state {}", state),
                    InputPinState::Io3(state) => info!("Input 3 state {}", state),
                    InputPinState::Io4(state) => info!("Input 4 state {}", state),
                }
            }
        }
    }
}
