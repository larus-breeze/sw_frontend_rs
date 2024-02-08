use corelib::CanRng;
use corelib::stm32_crc;
use stm32h7xx_hal::rng::{self, RngCore};

pub struct DevRng {
    rng: rng::Rng,
}

impl DevRng {
    pub fn new(rng: rng::Rng) -> Self {
        DevRng { rng }
    }
}

impl CanRng for DevRng {
    fn random(&mut self, min: u32, max: u32) -> u32 {
        let r: u32 = self.rng.gen().unwrap_or(0x2ab6_537c);
        let delta = r % (max - min);
        min + delta
    }
}

// Address of timer 2 counter register
const UUID: *const [u32; 3] = 0x1FF1_E800 as *const [u32; 3];

pub fn uuid() -> u32 {
    // Safety: we just read three fixed programmed u32
    unsafe { stm32_crc(&*UUID) }
}
