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
    continuous: bool,
    duty_cycle: u32
}

impl AudioParams {
    pub fn new(frequency: f32, volume: f32, continuous: bool, duty_cycle: u32) -> Self {
        let phase_inc = frequency / SAMPLE_FREQUENCY as f32;
        AudioParams { phase_inc, volume, continuous, duty_cycle }
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
            *x = if self.params.continuous {
                (2.0*self.phase - 1.0) * self.params.volume
            } else {
                if (self.wave_cnt % self.params.duty_cycle) < (self.params.duty_cycle / 2) {
                    (2.0*self.phase - 1.0) * self.params.volume
                } else {
                    0.0
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

