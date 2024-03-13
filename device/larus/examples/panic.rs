#![no_main]
#![no_std]
mod driver;

use defmt::trace;
use defmt_rtt as _;

use corelib::Event::KeyItem;
use corelib::*;
use cortex_m_rt::entry;
use heapless::mpmc::MpMcQueue;
use stm32h7xx_hal::{independent_watchdog::IndependentWatchdog, pac, prelude::*};
use embedded_sdmmc::{VolumeIdx, Mode};

use driver::*;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    trace!("init");

    // Constrain and freeze power, save a little bit power, optimum is at vos3 / 200 MHz
    let pwrcfg = dp.PWR.constrain().vos3().freeze();
    let ccdr = dp
        .RCC
        .constrain()
        .use_hse(16.MHz())
        .sys_ck(200.MHz())
        .hclk(100.MHz())
        .pll1_q_ck(50.MHz()) // CAN
        .pll2_p_ck(100.MHz()) // ?
        .pll2_r_ck(50.MHz()) // LCD
        .freeze(pwrcfg, &dp.SYSCFG);

    // Enable cortex m7 cache and cyclecounter
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();


    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
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

    let pins = SdcardPins::new(
        gpioc.pc12, gpiod.pd2, gpioc.pc8, gpioc.pc9, gpioc.pc10, gpioc.pc11, gpioe.pe3,
    );

    let mut fs = FileSys::new(pins, dp.SDMMC1, ccdr.peripheral.SDMMC1, &ccdr.clocks).unwrap();
    let mut volume = fs.fat().get_volume(VolumeIdx(0)).unwrap();
    let root_dir = fs.fat().open_root_dir(&volume).unwrap();


    let pb = PanicBuffer::init();
    if pb.has_content() {
        let mut file = fs.fat()
        .open_file_in_dir(
            &mut volume,
            &root_dir,
            "PANIC.LOG",
            Mode::ReadWriteCreateOrAppend,
        )
        .unwrap();
    
        let content = pb.content();
        fs.fat().write(&mut volume, &mut file, &content).unwrap();
        fs.fat().close_file(&mut volume, file).unwrap();
        pb.clear();
    }
    fs.fat().close_dir(&volume, root_dir);

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
                        loop {}
                    }
                    KeyEvent::Btn2 => {
                        trace!("Panic!");
                        panic!();
                    }
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
