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
/// In this module, unsafe is used several times to access the controller peripherals. This is
/// unavoidable, necessary and tests did not reveal any problems.
use defmt::Format;
use stm32h7xx_hal::{
    dac::{Disabled, C1},
    dma::dma::{Stream0, StreamsTuple},
    pac,
    pac::{DAC, DMA1},
    rcc::ResetEnable,
};

const SAMPLES_CNT: usize = 20;
#[allow(unused)]
const SILENT_WAVE: [u16; SAMPLES_CNT] = [
    2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047, 2047,
    2047, 2047, 2047, 2047,
];
#[allow(unused)]
const TRIANGULAR_WAVE: [u16; SAMPLES_CNT] = [
    2251, 2456, 2661, 2865, 3070, 2865, 2661, 2456, 2251, 2046, 1842, 1637, 1432, 1228, 1023, 1228,
    1432, 1637, 1842, 2046,
];
#[allow(unused)]
const SAWTOOTH_WAVE: [u16; SAMPLES_CNT] = [
    2149, 2251, 2354, 2456, 2558, 2661, 2763, 2865, 2968, 3070, 1126, 1228, 1331, 1433, 1535, 1638,
    1740, 1842, 1945, 2047,
];
#[allow(unused)]
const RECTANGULAR_WAVE: [u16; SAMPLES_CNT] = [
    3070, 3070, 3070, 3070, 3070, 3070, 3070, 3070, 3070, 3070, 1024, 1024, 1024, 1024, 1024, 1024,
    1024, 1024, 1024, 1024,
];
#[allow(unused)]
const SINE_WAVE: [u16; SAMPLES_CNT] = [
    2363, 2648, 2875, 3020, 3070, 3020, 2875, 2648, 2363, 2047, 1730, 1445, 1218, 1073, 1023, 1073,
    1218, 1445, 1730, 2046,
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
    on: bool,
    continous: bool,
    mute: bool,
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
                                                                //dma1.st[0].cr.write(|w| w.bits(0x2d5e));
            dma1.st[0].cr.write(|w| w.bits(0b0010_1101_0101_0000));
            dma1.st[0].cr.modify(|_, w| w.en().set_bit()); // start dma transfer
        }
        Sound {
            dac,
            tim,
            dma1_stream0,
            cycle_counter: 0,
            duty_cycle: 150,
            on: true,
            continous: false,
            mute: false,
            waveform: Waveform::Triangular,
        }
    }

    pub fn set_continous(&mut self, continous: bool) {
        self.continous = continous;
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: u16) {
        self.duty_cycle = duty_cycle;
    }

    pub fn set_frequency(&mut self, fq: u32) {
        let devider = (100_000_000 / SAMPLES_CNT as u32 / fq) as u16;
        self.tim.arr.write(|w| w.arr().bits(devider)); // preload register
    }

    pub fn set_mute(&mut self, mute: bool) {
        self.mute = mute;
        self.set_wave(self.on);
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
        self.cycle_counter += 1;
        if self.cycle_counter >= self.duty_cycle {
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
        let wave_ptr = if self.mute {
            SILENT_WAVE.as_ptr()
        } else {
            if sound_on {
                match self.waveform {
                    Waveform::Triangular => TRIANGULAR_WAVE.as_ptr(),
                    Waveform::Sawtooth => SAWTOOTH_WAVE.as_ptr(),
                    Waveform::Rectangular => RECTANGULAR_WAVE.as_ptr(),
                    Waveform::SineWave => SINE_WAVE.as_ptr(),
                }
            } else {
                SILENT_WAVE.as_ptr()
            }
        };
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        dma1.st[0].cr.modify(|_, w| w.en().clear_bit()); // stop dma transfer
        dma1.st[0]
            .m0ar
            .write(|w| unsafe { w.bits(wave_ptr as u32) }); // src
        dma1.st[0].cr.modify(|_, w| w.en().set_bit()); // start dma transfer
    }
}
