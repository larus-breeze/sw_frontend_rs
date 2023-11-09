#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    crc32::Crc32,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::peripheral::SCB::sys_reset();
}


#[derive(Debug)]
#[repr(C)]
struct MetaData {
    magic: u64,
    crc: u32,
    meta_version: u32,
    storage_addr: usize,
    hw_version: [u8; 4],
    sw_version: [u8; 4],
    copy_func: usize,
    new_app: usize,
    new_app_len: usize,
    new_app_dest: usize, 
}

// We simulate the loading of the image by flashing it with the debugger to the upper area.
//
// $ mon program image.bin 0x08080000
//
// Then wie start this test app
#[entry]
fn main() -> ! {
    // Setup clocks
    let _cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let _clocks = rcc.cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    const STORAGE: usize = 0x0808_0000;
    let meta_data = unsafe { core::mem::transmute::<usize, &MetaData>(STORAGE) };
    let upper_flash_u32 =  unsafe { core::mem::transmute::<usize, &[u32; 0x2_0000]>(STORAGE) };
    let new_app_start_idx = meta_data.new_app - STORAGE;
    let new_app_end_idx = new_app_start_idx + meta_data.new_app_len;

    // Check magic number
    if meta_data.magic != 0x1c80_73ab_2085_3579 {
        loop {}; // We should never come here
    }

    // Check CRC of uploaded data
    let mut crc_stm32 = Crc32::new(dp.CRC);
    crc_stm32.init();
    let crc = crc_stm32.update(&upper_flash_u32[3..new_app_end_idx/4]);

    if crc != meta_data.crc {
        loop {}; // We should never come here;
    }

    // Note: You should also check whether the hardware version is correct and whether it makes 
    // sense to reflash the software (software version).

    // This call starts the update. First the consistency of the loaded data is checked, then the 
    // data from the upper flash area is copied to the lower one and then the new app is started.
    let func = unsafe { core::mem::transmute::<usize, fn()>(meta_data.copy_func) };
    func();

    loop {}; // We should never come here;
}

