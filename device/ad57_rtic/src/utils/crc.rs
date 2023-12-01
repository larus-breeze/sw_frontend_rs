/// Calculate CRC like the stm32 controller hardware
///
/// The CRC here is calculated using the same algorithm as that integrated in hardware. The reason
/// for providing this function a second time is that no thread protection measures are required
/// when using this solution, as is the case when using singular hardware.
pub fn stm32_crc(data: &[u32]) -> u32 {
    let mut crc: u32 = 0xffffffff;
    for w in data {
        for val in w.to_be_bytes() {
            crc ^= (val as u32) << 24;
            for _ in 0..8 {
                if (crc & 0x8000_0000) == 0 {
                    crc <<= 1;
                } else {
                    crc = crc.wrapping_shl(1) ^ 0x04c1_1db7;
                }
            }
        }
    }
    crc
}
