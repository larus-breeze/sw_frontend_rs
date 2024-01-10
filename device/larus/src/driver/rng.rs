use stm32h7xx_hal::rng::{self, RngCore};
use can_dispatch::CanRng;

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
