#![no_main]
#![no_std]

mod utils;

use defmt::trace;
use defmt_rtt as _;
use embedded_storage::nor_flash::NorFlash;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    flash::{FlashExt, LockedFlash},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    watchdog::IndependentWatchdog,
};

use utils::*;

pub fn delay_ms(millis: u32) {
    let cycles = millis * 168_000;
    cortex_m::asm::delay(cycles)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    EepromOrI2c1,
    NoItemAvailable,
}

const FLASH_START: u32 = 0x0800_0000;
const PANIC_BUF: u32 = 0x0807_c000;
const ERR_MSG_LEN: usize = 128;
const PANIC_BUF_END: u32 = 0x0808_0000 - ERR_MSG_LEN as u32;

fn write_to_flash<'a>(msg: &'a str) {
    let mut ptr = PANIC_BUF;
    while ptr < PANIC_BUF_END {
        let b = unsafe { *(ptr as *const u8) };
        if b == 0xff {
            break;
        }
        ptr += 1;
    }
    let dp = unsafe { stm32f4xx_hal::pac::Peripherals::steal() };
    let mut flash = LockedFlash::new(dp.FLASH);
    let mut uflash = flash.unlocked();
    let _ = NorFlash::write(&mut uflash, ptr - FLASH_START, msg.as_bytes());
    drop(uflash);
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable(); // Please not interrupts any more, we will reset device anyway

    let msg: Concat<ERR_MSG_LEN> = if let Some(location) = info.location() {
        Concat::from_str("panic in '")
            .push_str(location.file())
            .push_str("' line ")
            .push_u32(location.line())
            .push_str("\n")
    } else {
        Concat::from_str("panic without location info\n")
    };
    trace!("{}", msg.as_str());
    write_to_flash(msg.as_str());
    loop {}
}

#[entry]
fn main() -> ! {
    // Setup clocks
    let _cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    let keyboard_pins = KeyboardPins::new(gpioa.pa7, gpioc.pc5, gpiob.pb0, gpiob.pb1, gpioa.pa6);
    let mut keyboard = Keyboard::new(keyboard_pins);

    let mut watchdog = IndependentWatchdog::new(dp.IWDG);
    watchdog.start(1000.millis());

    trace!("reset");

    loop {
        delay_ms(20);
        watchdog.feed();
        let key = keyboard.tick();
        if key != Key::None {
            trace!("{}", key as u32);
        }
        match key {
            Key::Button4 => loop {},  // Watchdog test
            Key::Button3 => panic!(), // Panic test
            _ => (),
        }
    }
}
