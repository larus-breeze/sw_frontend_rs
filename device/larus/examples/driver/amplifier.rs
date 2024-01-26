use stm32h7xx_hal::{
    {pac, prelude::*, pac::interrupt},
    i2c::I2c, pac::I2C1,
};

pub struct Amplifier {
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