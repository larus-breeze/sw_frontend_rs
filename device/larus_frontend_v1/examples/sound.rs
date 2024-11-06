#![no_main]
#![no_std]

mod driver;
use driver::*;

use defmt::*;
use defmt_rtt as _;

use stm32h7xx_hal::{
    dma::dma::StreamsTuple,
    rcc::ResetEnable,
    {pac, pac::interrupt, prelude::*},
};

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

static SOUND: Mutex<RefCell<Option<Sound>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn DMA1_STR0() {
    cortex_m::interrupt::free(|cs| {
        let mut rc = SOUND.borrow(cs).borrow_mut();
        let sound = rc.as_mut().unwrap();
        sound.on_interrupt();
    });
}

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    info!("init");

    let ccdr = set_clocksys!(dp);
    let _mono = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // hopefully, this is included let _ = ccdr.peripheral.DAC12.enable();
    let dac = dp.DAC.dac(gpioa.pa4, ccdr.peripheral.DAC12);

    // Calibrate output buffer
    let dac = dac.calibrate_buffer(&mut delay);
    let scl = gpiob.pb6.into_alternate_open_drain();
    let sda = gpiob.pb7.into_alternate_open_drain();
    let i2c = dp
        .I2C1
        .i2c((scl, sda), 400.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

    let i2c_ref_cell = RefCell::new(i2c);

    let mut amplifier = Amplifier::new(I2cManager::new(&i2c_ref_cell));
    amplifier.set_gain(0);

    // hopefully, this is included let _ = ccdr.peripheral.DMA1.enable();
    let streams = StreamsTuple::new(dp.DMA1, ccdr.peripheral.DMA1);

    trace!("Init Sound");
    let _ = ccdr.peripheral.TIM6.enable();
    unsafe {
        cp.NVIC.set_priority(interrupt::DMA1_STR0, 1);
        pac::NVIC::unmask(interrupt::DMA1_STR0);
    }

    let sound = Sound::new(dac, dp.TIM6, streams.0);

    cortex_m::interrupt::free(|cs| {
        SOUND.borrow(cs).replace(Some(sound));
    });

    let mut gain = 1;
    let mut delta = -1;
    amplifier.set_gain(gain);
    let mut fq_idx = 0;
    //let dac_reg = 0x4000_7408 as *mut u16;
    loop {
        let fq = match fq_idx {
            0 => 523,
            1 => 659,
            2 => 784,
            3 => {
                fq_idx = -1;
                1047
            }
            _ => 523,
        };
        fq_idx += 1;

        gain = (gain as i8 + delta) as u8;

        amplifier.set_gain(gain);
        cortex_m::interrupt::free(|cs| {
            let mut rc = SOUND.borrow(cs).borrow_mut();
            let sound = rc.as_mut().unwrap();

            delta = match gain {
                0 => 1,
                8 => {
                    let next_wf = sound.waveform().next();
                    sound.set_waveform(next_wf);
                    -1
                }
                _ => delta,
            };

            sound.set_params(fq, false, false);
            trace!(
                "Waveform {:?}, Freqeuncy {} Hz, Gain {} dB",
                sound.waveform(),
                fq,
                gain
            );
        });

        delay_ms(1000);
    }
}
