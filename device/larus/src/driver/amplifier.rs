use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
/// Amplifier module for controlling the TPA2028D1 amplifier
///
/// This simple component initializes the amplifier and can continuously change the volume.
/// Communication via the i2c bus is designed in such a way that any errors are quietly
/// ignored.
///
/// The hardware of the front end does not support the amplifier when supplied with more than
/// 5V, so that communication with the amplifier module is not possible in this case.  

pub struct Amplifier<I2C>
where
    I2C: Read + Write + WriteRead,
{
    i2c: I2C,
}

const AMP_ADDR: u8 = 0x58;

impl<I2C> Amplifier<I2C>
where
    I2C: Read + Write + WriteRead,
{
    pub fn new(i2c: I2C) -> Self {
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
            }
            1..=30 => gain,
            _ => 30,
        };
        self.write(1, 0xc3);
        self.write(5, gain);
    }

    fn write(&mut self, register: u8, value: u8) {
        let bytes = [register, value];
        let _ = self.i2c.write(AMP_ADDR, &bytes);
    }
}
