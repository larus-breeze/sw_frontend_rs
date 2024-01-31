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
        amp.write(2, 0b0000_0001); // minimize time between gain decrease
        amp.write(3, 0b0000_0001); // minimize time between gain increase
        amp.write(4, 0b0000_0000); // minimize hold between gain change
        amp.write(1, 0xc3); // activate amplifier
        amp
    }

    pub fn set_gain(&mut self, gain: u8) {
        // Note: gain 0 is not mute for the amplifier, you always hear something. Volume 0
        // is handled by the sound modul, which mutes the sound in this case.
        if gain <= 30 {
            self.write(5, gain);
        } else {
            self.write(5, 30);
        }
    }

    fn write(&mut self, register: u8, value: u8) {
        let bytes = [register, value];
        let _ = self.i2c.write(AMP_ADDR, &bytes);
    }
}
