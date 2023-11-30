use concurrent_queue::ConcurrentQueue;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

const SAMPLE_FREQUENCY: i32 = 32768;

pub fn desired_spec() -> AudioSpecDesired {
    AudioSpecDesired {
        freq: Some(SAMPLE_FREQUENCY),
        channels: Some(1),  // mono
        samples: None       // default sample size
    }
}

#[derive(Debug)]
pub struct AudioParams {
    phase_inc: f32,
    volume: f32,
    continous: bool,
}

impl AudioParams {
    pub fn new(frequency: f32, volume: f32, continous: bool) -> Self {
        let phase_inc = frequency / SAMPLE_FREQUENCY as f32;
        AudioParams { phase_inc, volume, continous }
    }
}


pub struct SquareWave {
    queue: &'static ConcurrentQueue<AudioParams>,
    params: AudioParams,
    phase: f32,
    wave_cnt: u32,
}

impl SquareWave {
    pub fn new(queue: &'static ConcurrentQueue<AudioParams>, params: AudioParams) -> Self {
        SquareWave { queue, params, phase: 0.0, wave_cnt: 0 }
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        if self.queue.len() > 0 {
            let ap = self.queue.pop().unwrap();
            self.params = ap;
        }
        // Generate triangular oscillation
        for x in out.iter_mut() {
            *x = if self.params.continous {
                (2.0*self.phase - 1.0) * self.params.volume
            } else { 
                match self.wave_cnt % 80 {
                    0..=50 => (2.0*self.phase - 1.0) * self.params.volume,
                    _ => 0.0,
                }
            };
            
            let last_phase = self.phase;
            self.phase = (self.phase + self.params.phase_inc) % 1.0;
            if last_phase > self.phase {
                self.wave_cnt = self.wave_cnt.wrapping_add(1);                
            }
        }
    }
}

