#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac::Peripherals,
    flash::{FlashExt, LockedFlash},
    crc32::Crc32,
};
use embedded_storage::nor_flash::NorFlash;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::peripheral::SCB::sys_reset();
}

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

#[entry]
fn main() -> ! {
    copy_image();
}

fn copy_image() -> ! {
    cortex_m::interrupt::disable();

    let dp = unsafe { Peripherals::steal() };

    const STORAGE: usize = 0x0808_0000;
    let meta_data = unsafe { core::mem::transmute::<usize, &MetaData>(STORAGE) };
    let upper_flash_u32 =  unsafe { core::mem::transmute::<usize, &[u32; 0x2_0000]>(STORAGE) };
    let upper_flash_u8 =  unsafe { core::mem::transmute::<usize, &[u8; 0x8_0000]>(STORAGE) };
    let new_app_start_idx = meta_data.new_app - STORAGE;
    let new_app_end_idx = new_app_start_idx + meta_data.new_app_len;

    if meta_data.magic != 0x1c80_73ab_2085_3579 {
        cortex_m::peripheral::SCB::sys_reset();
    }

    let mut crc_stm32 = Crc32::new(dp.CRC);
    crc_stm32.init();
    let crc = crc_stm32.update(&upper_flash_u32[3..new_app_end_idx/4]);

    if crc != meta_data.crc {
        cortex_m::peripheral::SCB::sys_reset();
    }

    let mut flash = LockedFlash::new(dp.FLASH);
    let mut unlocked_flash = flash.unlocked();
    NorFlash::erase(&mut unlocked_flash, 0, meta_data.new_app_len as u32).unwrap();
    NorFlash::write(&mut unlocked_flash, 0, &upper_flash_u8[new_app_start_idx..new_app_end_idx]).unwrap();

    // Lock flash again
    drop(unlocked_flash);

    cortex_m::peripheral::SCB::sys_reset();
}

