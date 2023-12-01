#![no_main]
#![no_std]

mod utils;

use defmt::trace;
use {defmt_rtt as _, panic_probe as _};

use cortex_m_rt::entry;
use embedded_storage::nor_flash::NorFlash;
use stm32f4xx_hal::{
    crc32::Crc32,
    flash::{FlashExt, LockedFlash},
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    sdio::{ClockFreq, SdCard, Sdio},
};

use fatfs::Read;
use utils::*;

pub fn delay_ms(millis: u32) {
    let cycles = millis * 168_000;
    cortex_m::asm::delay(cycles)
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
    // Setup clocks
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    trace!("init");

    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .require_pll48clk()
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    assert!(clocks.is_pll48clk_valid());

    let mut delay = cp.SYST.delay(&clocks);

    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let d0 = gpioc.pc8.internal_pull_up(true);
    let d1 = gpioc.pc9.internal_pull_up(true);
    let d2 = gpioc.pc10.internal_pull_up(true);
    let d3 = gpioc.pc11.internal_pull_up(true);
    let clk = gpioc.pc12;
    let cmd = gpiod.pd2.internal_pull_up(true);
    let mut sdio: Sdio<SdCard> = Sdio::new(dp.SDIO, (clk, cmd, d0, d1, d2, d3), &clocks);
    //let mut sdio: Sdio<SdCard> = Sdio::new(dp.SDIO, (clk, cmd, d0), &clocks);

    trace!("Waiting for card...");

    //  Wait for card to be ready
    loop {
        match sdio.init(ClockFreq::F24Mhz) {
            Ok(_) => break,
            Err(_err) => (),
        }

        delay.delay_ms(1000u32);
    }

    let sd_card = FileIo::new(sdio).unwrap();
    let options = fatfs::FsOptions::new();
    let fs = fatfs::FileSystem::new(sd_card, options).unwrap();

    let root_dir = fs.root_dir();
    let mut image_size: u64 = 0;
    for r in root_dir.iter() {
        let entry = r.unwrap();
        if entry.short_file_name_as_bytes() == "IMAGE.BIN".as_bytes() {
            image_size = entry.len();
        }
    }

    if image_size > 0 {
        trace!("Image 'IMAGE.BIN' found, size {}", image_size);
    } else {
        trace!("Image 'IMAGE.BIN' not found, ERROR");
    }

    const STORAGE_ADR: u32 = 0x0808_0000;
    const UPPER_FLASH_OFS: u32 = 0x0008_0000;
    let meta_data = unsafe { core::mem::transmute::<u32, &MetaData>(STORAGE_ADR) };

    let mut buf = [0_u8; 512];
    let mut bytes_read: u32 = 0;
    let mut image_file = root_dir.open_file("IMAGE.BIN").unwrap();

    let mut flash = LockedFlash::new(dp.FLASH);
    let mut unlocked_flash = flash.unlocked();
    trace!("Erase Flash");
    NorFlash::erase(
        &mut unlocked_flash,
        UPPER_FLASH_OFS,
        UPPER_FLASH_OFS + image_size as u32,
    )
    .unwrap();

    trace!("Write Image");
    loop {
        let b_read = image_file.read(&mut buf).unwrap();
        NorFlash::write(&mut unlocked_flash, UPPER_FLASH_OFS + bytes_read, &buf).unwrap();
        //delay_ms(10);
        bytes_read += b_read as u32;
        if b_read == 0 {
            break;
        }
        if bytes_read % 10240 == 0 {
            trace!("\x0d{} kBytes copied", bytes_read / 1024);
        }
    }
    drop(unlocked_flash);
    trace!("\x0dOk {} Bytes copied from SdCard to Flash", bytes_read);

    if meta_data.magic == 0x1c80_73ab_2085_3579 {
        trace!("Ok Signature {=u64:X}", meta_data.magic);
    } else {
        trace!("Signature is not correct");
        loop {}
    }

    // Check CRC of uploaded data
    let upper_flash_u32 = unsafe { core::mem::transmute::<u32, &[u32; 0x2_0000]>(STORAGE_ADR) };
    let new_app_start_idx = meta_data.new_app - STORAGE_ADR as usize;
    let new_app_end_idx = new_app_start_idx + meta_data.new_app_len;

    let mut crc_stm32 = Crc32::new(dp.CRC);
    crc_stm32.init();
    let crc = crc_stm32.update(&upper_flash_u32[3..new_app_end_idx / 4]);

    if crc == meta_data.crc {
        trace!("Ok Crc {=u32:X}", crc);
    } else {
        trace!("CrC is not correct");
        loop {} // We should never come here;
    }

    // Note: You should also check whether the hardware version is correct and whether it makes
    // sense to reflash the software (software version).

    // This call starts the update. First the consistency of the loaded data is checked, then the
    // data from the upper flash area is copied to the lower one and then the new app is started.
    trace!("Copy and start new app, please be patient");
    let func = unsafe { core::mem::transmute::<usize, fn()>(meta_data.copy_func) };
    func();

    loop {} // We should never come here;
}
