use corelib::*;
use byteorder::{ByteOrder, LittleEndian as LE};
use tinyaudio::{OutputDeviceParameters, run_output_device, OutputDevice};
use std::{
    f32::consts::PI,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Clone, Copy)]
struct AudioData {
    pub frequency: u16,
    pub duty_cycle: u16,
    pub volume: u8,
    pub continuous: bool
}

impl AudioData {
    pub fn new() -> Self {
        AudioData { frequency: 440, duty_cycle: 120, volume: 0, continuous: true }
    }
    
}

const VOL_BASE: f32 = 1.1481536214968828; // 1000.0**(1/50)
const AUDIO_DATA_PER_SECOND: usize = 10; // coming fromt frontend firmware
const CHANNEL_COUNT: usize = 2;
const CHUNKS_PER_SECOND: usize = 100; // are send to the sound card
const SAMPLE_RATE: usize = 44100;
const PARAMS: OutputDeviceParameters = OutputDeviceParameters {
    channels_count: 2,
    sample_rate: SAMPLE_RATE,
    channel_sample_count: SAMPLE_RATE / CHUNKS_PER_SECOND,
};


pub struct Sound {
    _device: OutputDevice,
    audio_tx: Sender<AudioData>,
}

impl Sound {
    pub fn new() -> Self {
        let (audio_tx, audio_rx) = channel::<AudioData>(); 

        // This function is given to the tinyaudio driver. It contains a closure, which collects
        // the sound data. The communication to the closure is done via a mpsc channel. Frequencies
        // between two sound commands from frontend are interpolated.
        //
        //  This driver is unable to switch at zero amplitude, resulting in occasional crackling noises.
        fn data_callback(audio_rx: Receiver<AudioData>) -> impl FnMut(&mut [f32]) + Send + 'static {
            let mut clock = 0f32;
            let mut audio_data = AudioData::new();
            let mut frequency = audio_data.frequency as i16;
            let mut delta_frequency = 0_i16;
            let mut delta_count = 0;
            let mut duty_cyle: u16 = 0;
            let mut toggle = true;
    
            move |data| {
                match audio_rx.try_recv() {
                    Ok(data) => {
                        audio_data = data;
                        delta_frequency = (audio_data.frequency as i16 - frequency) / AUDIO_DATA_PER_SECOND as i16;
                        delta_count = CHUNKS_PER_SECOND / AUDIO_DATA_PER_SECOND;
                    },
                    Err(_) => (),
                }

                if delta_count > 0 {
                    delta_count -= 1;
                } else {
                    delta_frequency = 0;
                }

                frequency += delta_frequency;
    
                if duty_cyle == 0 {
                    duty_cyle = (CHUNKS_PER_SECOND * audio_data.duty_cycle as usize / audio_data.frequency as usize ) as u16;
                    toggle = !toggle;
                }
                duty_cyle -= 1;
    
                let mut vol_mult = if audio_data.volume == 0 {
                    0.0
                } else {
                    VOL_BASE.powf(audio_data.volume as f32) / 1000.0
                };
    
                if toggle & !audio_data.continuous {
                    vol_mult = 0.0
                }
    
                let delta_clock = frequency as f32 * 2.0 * PI / SAMPLE_RATE as f32;
    
                for samples in data.chunks_mut(CHANNEL_COUNT) {
                    clock += delta_clock;
                    if clock > PI {
                        clock -= 2.0 * PI;
                    }
                    let value = clock.sin()*vol_mult;
                    for sample in samples {
                        *sample = value;
                    }
                }
            }
        }
    
        let device = run_output_device(PARAMS, data_callback(audio_rx)).unwrap();

        Sound { _device: device, audio_tx }
    }

    pub fn from_can_datagram(&self, frame: Frame) {
        if let Frame::Specific(specific_frame) = frame {
            if specific_frame.specific_id == 0 {
                let data = specific_frame.can_frame.data();

                let mut audio_data = AudioData::new();
                audio_data.frequency = LE::read_u16(&data[..2]);
                audio_data.duty_cycle = LE::read_u16(&data[2..4]);
                audio_data.volume = data[4];
                audio_data.continuous = data[5] == 1;

                self.audio_tx.send(audio_data).unwrap();
            }
        }
    }
}


