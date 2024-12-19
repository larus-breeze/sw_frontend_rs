/// Sound Modul
///
/// The sound module provides the necessary components to realize the sound output of the Vario.
/// This is implemented as follows:
/// - A timer cyclically generates events that are passed on to the digital-to-analog converter.
///   The sound frequency of the output is set by adjusting the timer start value.
/// - The digital-to-analog converter outputs the currently specified amplitude value and signals
///   to the DMA that the next value is required.
/// - The DMA copies the next amplitude value from a waveform table to the DAC. The DMA works
///   cyclically and sends an interrupt request after each cycle.
/// - The interrupt service routine checks whether changes to the sound are pending. The sound
///   signal could be interrupted (continuous / not continuous), it could be switched off (mute)
///   or a new waveform could be specified.
///
/// Great importance was attached to ensuring that acoustically relevant switching operations are
/// always carried out at the zero crossing of the oscillation, so that no cracking noises are
/// generated. This applies to non-continuous tones, mute function and switching the desired
/// waveform.
///
/// In this module, unsafe is used several times to access the controller peripherals. This is
/// unavoidable, necessary and tests did not reveal any problems.
use defmt::*;
use stm32h7xx_hal::{
    dac::{Disabled, C1},
    dma::dma::Stream0,
    pac,
    pac::{DAC, DMA1},
};
use super::H7_HCLK;

const SAMPLES_CNT: usize = 20;
#[allow(unused)]
const SILENT_WAVE: [u16; SAMPLES_CNT] = [
    2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047,
    2047, 2047, 2047, 2047,
];
#[allow(unused)]
const TRIANGULAR_WAVE: [u16; SAMPLES_CNT] = [
    2456, 2866, 3275, 3685, 4094, 3685, 3275, 2866, 2456, 2047, 1637, 1228, 818, 409, 0, 409, 818,
    1228, 1637, 2047,
];
#[allow(unused)]
const TRIANGULAR_20DB_WAVE: [u16; SAMPLES_CNT] = [
    2087, 2128, 2169, 2210, 2251, 2210, 2169, 2128, 2087, 2046, 2006, 1965, 1924, 1883, 1842, 1883,
    1924, 1965, 2006, 2046,
];
#[allow(unused)]
const SAWTOOTH_WAVE: [u16; SAMPLES_CNT] = [
    2251, 2456, 2661, 2866, 3070, 3275, 3480, 3685, 3889, 4094, 204, 409, 614, 819, 1023, 1228,
    1433, 1638, 1842, 2047,
];
#[allow(unused)]
const SAWTOOTH_20DB_WAVE: [u16; SAMPLES_CNT] = [
    2067, 2087, 2108, 2128, 2149, 2169, 2190, 2210, 2231, 2251, 1863, 1883, 1904, 1924, 1945, 1965,
    1986, 2006, 2027, 2047,
];
#[allow(unused)]
const RECTANGULAR_WAVE: [u16; SAMPLES_CNT] = [
    4094, 4094, 4094, 4094, 4094, 4094, 4094, 4094, 4094, 4094, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[allow(unused)]
const RECTANGULAR_20DB_WAVE: [u16; SAMPLES_CNT] = [
    2251, 2251, 2251, 2251, 2251, 2251, 2251, 2251, 2251, 2251, 1843, 1843, 1843, 1843, 1843, 1843,
    1843, 1843, 1843, 1843,
];
#[allow(unused)]
const SINE_WAVE: [u16; SAMPLES_CNT] = [
    2680, 3251, 3704, 3995, 4095, 3995, 3704, 3251, 2680, 2048, 1415, 844, 391, 100, 0, 100, 391,
    844, 1415, 2047,
];
#[allow(unused)]
const SINE_20DB_WAVE: [u16; SAMPLES_CNT] = [
    2111, 2168, 2213, 2242, 2252, 2242, 2213, 2168, 2111, 2048, 1984, 1927, 1882, 1853, 1843, 1853,
    1882, 1927, 1984, 2047,
];

#[derive(Clone, Copy, Format)]
pub enum Waveform {
    Triangular,
    Sawtooth,
    Rectangular,
    SineWave,
}

impl Waveform {
    pub fn next(&self) -> Self {
        match self {
            Waveform::Triangular => Waveform::Sawtooth,
            Waveform::Sawtooth => Waveform::Rectangular,
            Waveform::Rectangular => Waveform::SineWave,
            Waveform::SineWave => Waveform::Triangular,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Waveform::Triangular => Waveform::SineWave,
            Waveform::Sawtooth => Waveform::Triangular,
            Waveform::Rectangular => Waveform::Sawtooth,
            Waveform::SineWave => Waveform::Rectangular,
        }
    }
}

#[allow(unused)]
pub struct Sound {
    dac: C1<DAC, Disabled>,
    tim: pac::TIM6,
    dma1_stream0: Stream0<DMA1>,
    cycle_counter: u16,
    duty_cycle: u16,
    wave_ctr: u32,
    next_f: f32,
    curr_f: f32,
    delta_f: f32,
    on: bool,
    continous: bool,
    gain: i8,
    waveform: Waveform,
}

impl Sound {
    pub fn new(dac: C1<DAC, Disabled>, tim: pac::TIM6, dma1_stream0: Stream0<DMA1>) -> Self {
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
            dma1.st[0].m0ar.write(|w| w.bits(wave_ptr as u32));
            // Bit 4     transfer complete ie
            // Bit 7:6   0b01 memory to peripheral
            // Bit 8     circular mode
            // Bit 10    memory increment after transfer
            // Bit 12:11 0b01 peripheral data size 16 Bit
            // Bit 14:13 0b01 memory data size 16 Bit
            dma1.st[0].cr.write(|w| w.bits(0b0010_1101_0101_0000));
            dma1.st[0].cr.modify(|_, w| w.en().set_bit()); // start dma transfer
        }
        Sound {
            dac,
            tim,
            dma1_stream0,
            cycle_counter: 0,
            duty_cycle: 120,
            wave_ctr: 0,
            next_f: 1000.0,
            curr_f: 1000.0,
            delta_f: 0.0,
            on: true,
            continous: false,
            gain: 0,
            waveform: Waveform::Triangular,
        }
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: u16) {
        self.duty_cycle = duty_cycle;
    }

    pub fn set_params(&mut self, fq: u16, continous: bool, gain: i8) {
        let devider = self.tim.arr.read().bits();
        self.wave_ctr = 0;
        self.curr_f = H7_HCLK as f32 / SAMPLES_CNT as f32 / devider as f32;
        // Calculate delta frequency asume 10 Hz tick rate
        self.next_f = fq as f32;
        self.delta_f = (self.next_f - self.curr_f) / self.curr_f * 10.0;
        // let devider = (100_000_000 / SAMPLES_CNT as u32 / fq as u32) as u16;
        // self.tim.arr.write(|w| w.arr().bits(devider)); // preload register
        self.continous = continous;
        self.gain = gain;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
        self.set_wave(self.on);
    }

    pub fn waveform(&self) -> Waveform {
        self.waveform
    }

    pub fn on_interrupt(&mut self) {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        dma1.lifcr.write(|w| w.ctcif0().set_bit());

        // calc next frequency
        self.curr_f += self.delta_f;
        self.wave_ctr += 1;
        if self.curr_f > 200.0 && self.curr_f < 2000.0 {
            let devider = (H7_HCLK / SAMPLES_CNT as u32 / self.curr_f as u32) as u16;
            self.tim.arr.write(|w| w.arr().bits(devider)); // preload register
        }

        self.cycle_counter += 1;

        if self.gain == 0 {
            if self.on {
                self.set_wave(false);
            }
        } else if self.cycle_counter >= self.duty_cycle {
            self.cycle_counter = 0;
            match self.continous {
                true => {
                    if !self.on {
                        self.set_wave(true);
                    }
                }
                false => match self.on {
                    true => self.set_wave(false),
                    false => self.set_wave(true),
                },
            }
        }
    }

    fn set_wave(&mut self, sound_on: bool) {
        self.on = sound_on;
        let wave_ptr = if sound_on {
            if self.gain > 30 {
                match self.waveform {
                    Waveform::Triangular => TRIANGULAR_WAVE.as_ptr(),
                    Waveform::Sawtooth => SAWTOOTH_WAVE.as_ptr(),
                    Waveform::Rectangular => RECTANGULAR_WAVE.as_ptr(),
                    Waveform::SineWave => SINE_WAVE.as_ptr(),
                }
            } else {
                match self.waveform {
                    Waveform::Triangular => TRIANGULAR_20DB_WAVE.as_ptr(),
                    Waveform::Sawtooth => SAWTOOTH_20DB_WAVE.as_ptr(),
                    Waveform::Rectangular => RECTANGULAR_20DB_WAVE.as_ptr(),
                    Waveform::SineWave => SINE_20DB_WAVE.as_ptr(),
                }
            }
        } else {
            SILENT_WAVE.as_ptr()
        };
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        dma1.st[0].cr.modify(|_, w| w.en().clear_bit()); // stop dma transfer
        dma1.st[0]
            .m0ar
            .write(|w| unsafe { w.bits(wave_ptr as u32) }); // src
        dma1.st[0].cr.modify(|_, w| w.en().set_bit()); // start dma transfer
    }
}
