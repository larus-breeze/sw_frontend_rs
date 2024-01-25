#![no_main]
#![no_std]

mod driver;
use driver::*;

use defmt::*;
use defmt_rtt as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{
    pac::{DMA1, DAC},
    {pac, prelude::*},
    //traits::DacOut,
    i2c::I2c, pac::I2C1,
    rcc::ResetEnable,
    dma::dma::{Stream0, StreamsTuple},
    dac::{Disabled, C1}
};

struct Amplifier {
    i2c: I2c<I2C1>,
}

const AMP_ADDR: u8 = 0x58;

impl Amplifier {
    pub fn new(i2c: I2c<I2C1>) -> Self {
        let mut amp = Amplifier { i2c };

        amp.write(7, 0b1100_0000); // disable compression, max gain 30 dB
        amp.write(6, 0b1011_1010); // disable output limiter
        amp        
    }

    pub fn set_gain(&mut self, gain: u8) {
        let gain = match gain {
            0 => {
                self.write(1, 0x83);
                return;
            },
            1..=30 => gain,
            _ => 30,
        };
        self.write(1, 0xc3);
        self.write(5, gain);
    }

    fn write(&mut self, register: u8, value: u8) {
        let bytes = [register, value];
        self.i2c.write(AMP_ADDR, &bytes).unwrap();
    }
}

const SAMPLES_CNT: usize = 20;
#[allow(unused)]
const TRIANGULAR_WAVE: [u16; SAMPLES_CNT] = [409, 819, 1228, 1638, 2047, 2457, 2866, 3276, 3685, 4095, 3685, 3276, 2866, 2457, 2047, 1638, 1228, 819, 409, 0];
#[allow(unused)]
const SAWTOOTH_WAVE: [u16; SAMPLES_CNT] = [204, 409, 614, 819, 1023, 1228, 1433, 1638, 1842, 2047, 2252, 2457, 2661, 2866, 3071, 3276, 3480, 3685, 3890, 4095];
#[allow(unused)]
const RECTANGULAR_WAVE: [u16; SAMPLES_CNT] = [4095, 4095, 4095, 4095, 4095, 4095, 4095, 4095, 4095, 4095, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[allow(unused)]
const SINE_WAVE: [u16; SAMPLES_CNT] = [2047, 2679, 3250, 3703, 3993, 4094, 3993, 3703, 3250, 2679, 2047, 1414, 843, 390, 100, 0, 100, 390, 843, 1414];#[allow(unused)]

pub struct SoundMachine {
    dac: C1<DAC, Disabled>,
    tim: pac::TIM6, 
    dma1_stream0: Stream0<DMA1>,
}

impl SoundMachine {
    pub fn new(
        dac: C1<DAC, Disabled>,
        tim: pac::TIM6, 
        dma1_stream0: Stream0<DMA1>,
    ) -> Self {
        let wave_ptr = TRIANGULAR_WAVE.as_ptr();

        unsafe {
            tim.cr2.write(|w| w.mms().bits(0b010)); // update event trigger output
            tim.arr.write(|w| w.arr().bits(11363)); // preload register
            tim.cr1.modify(|_, w| w.cen().set_bit()); // start timer

            let dac1 = &(*pac::DAC::ptr());
            dac1.mcr.modify(|_, w| w.mode1().bits(0));
            // CR 0x3    0    1    7
            //    0b0011_0000_0001_0111
            // Bit 0    en1 enable
            // Bit 1    ten1 enable trigger enable
            // Bit 5:2  tsel1 trigger select 00101 dac1_ch1_trig5 (Tim6_tgo)
            // Bit 12   dmaen1 DAC chan1 DMA enable
            // Bit 13   dmaudrie1 DAC chan1 DMA underrun ie
            dac1.cr.modify(|_, w| w.ten1().set_bit());
            dac1.cr.modify(|_, w| w.tsel1().bits(5));
            dac1.cr.modify(|_, w| w.dmaen1().set_bit());
            dac1.cr.modify(|_, w| w.dmaudrie1().set_bit());
            dac1.cr.modify(|_, w| w.en1().set_bit());

            let dmamux1 = &(*pac::DMAMUX1::ptr());
            dmamux1.ccr[0].write(|w| w.dmareq_id().bits(0x43));

            let dma1 = &(*pac::DMA1::ptr());
            dma1.st[0].ndtr.write(|w| w.bits(SAMPLES_CNT as u32)); // cnt dma
            dma1.st[0].par.write(|w| w.bits(0x4000_7408)); // dst 12 Bit DAC register
            dma1.st[0].m0ar.write(|w| w.bits(wave_ptr as u32)); // src
            // dma1.s0cr 0x2    d    5    6 
            //           0b0010_1011_0101_0110
            // Bit 1    direct mode interrupt enable
            // Bit 2    transfer error ie
            // Bit 4    transfer complete ie
            // Bit 6    memory to peripheral
            // Bit 8    circular mode
            // Bit 9    periphel increment
            // Bit 11   peripheral data size 16 Bit
            // Bit 13   memory data size 16 Bit
            dma1.st[0].cr.write(|w| w.bits(0x2d5e));
            //dma1.st[0].cr.write(|w| w.bits(0b0010_1011_0101_0000));
            dma1.st[0].cr.modify(|_, w| w.en().set_bit()); // start dma transfer
        }
        SoundMachine { dac, tim, dma1_stream0 }
    }

    pub fn set_frequency(&mut self, fq: u32) {
        let devider = (100_000_000 / SAMPLES_CNT as u32 / fq) as u16;
        self.tim.arr.write(|w| w.arr().bits(devider)); // preload register
    }

    pub fn on_interrupt(&mut self) {
        self.tim.sr.write(|w| w.uif().clear_bit());
    }
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
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

    let mut amplifier = Amplifier::new(i2c);
    amplifier.set_gain(0);

    // hopefully, this is included let _ = ccdr.peripheral.DMA1.enable();
    let streams = StreamsTuple::new(dp.DMA1, ccdr.peripheral.DMA1);

    trace!("Init Sound");
    let _ = ccdr.peripheral.TIM6.enable();
    
    let mut sm = SoundMachine::new(
        dac,
        dp.TIM6,
        streams.0,
    );

    let mut gain = 10;
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
            },
            _ => 523,
        };

        sm.set_frequency(fq);
        fq_idx += 1;


        trace!("Freqeunz {} Hz, Gain {} dB", fq, gain);
        delay_ms(1000);

        gain = (gain as i8 + delta) as u8;
        amplifier.set_gain(gain);
        
        delta = match gain {
            0 => 1,
            15 => -1,
            _ => delta,
        };
    }
}
