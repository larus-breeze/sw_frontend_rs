use core::str;
use corelib::{stm32_crc, MetaDataV1, SwVersion, VersionCheck, SIZE_METADATA_V1};
use embedded_sdmmc::{Mode, ShortFileName, VolumeIdx};
use embedded_storage::nor_flash::NorFlash;
use heapless::{String, Vec};
use stm32f4xx_hal::flash::{FlashExt, LockedFlash};

use crate::{driver::*, HW_VERSION};

use super::SW_VERSION;

pub fn update_available() -> Option<SwVersion> {
    let fs = get_filesys()?;
    // open filesystem
    let volume = fs.vol_mgr().open_volume(VolumeIdx(0)).ok()?;
    let root_dir = fs.vol_mgr().open_root_dir(volume).ok()?;

    // read root directory, look after *.bin files
    let mut files = Vec::<ShortFileName, 20>::new();
    fs.vol_mgr()
        .iterate_dir(root_dir, |entry| {
            if entry.name.extension() == [66, 73, 78] && // BIN
                entry.size > SIZE_METADATA_V1 as u32
            {
                let _ = files.push(entry.name.clone());
            }
        })
        .ok()?;

    // check the *.bin files if there is something interesting there
    let mut check = VersionCheck::new(HW_VERSION, SW_VERSION);
    let mut buffer = [0_u8; SIZE_METADATA_V1];
    for name in files {
        let mut fname = String::<12>::new();
        let base = unsafe { str::from_utf8_unchecked(name.base_name()) };
        let _ = fname.push_str(base);
        let _ = fname.push('.');
        let ext = unsafe { str::from_utf8_unchecked(name.extension()) };
        let _ = fname.push_str(ext);

        let file = fs
            .vol_mgr()
            .open_file_in_dir(root_dir, fname.as_str(), Mode::ReadOnly)
            .ok()?;
        let num_read = fs.vol_mgr().read(file, &mut buffer).ok()?;
        if num_read == SIZE_METADATA_V1 {
            check.analyse(fname.as_str(), &buffer)
        }
        let _ = fs.vol_mgr().close_file(file);
    }

    let result = if let Some(image_name) = check.new_image_name() {
        // a new image file was found
        let image_file = fs
            .vol_mgr()
            .open_file_in_dir(root_dir, image_name.as_str(), Mode::ReadOnly)
            .ok()?;

        let dp = unsafe { stm32f4xx_hal::pac::Peripherals::steal() };
        let mut flash = LockedFlash::new(dp.FLASH);
        let mut unlocked_flash = flash.unlocked();

        let image_size = fs.vol_mgr().file_length(image_file).ok()?;

        let flash_offset = (STORAGE - BEGIN_FLASH) as u32;

        NorFlash::erase(&mut unlocked_flash, flash_offset, flash_offset + image_size).unwrap();

        // write image file to flash memory
        let mut buffer = [0_u8; 512];
        let mut bytes_read = 0_u32;
        loop {
            let b_read = fs.vol_mgr().read(image_file, &mut buffer).ok()?;
            NorFlash::write(&mut unlocked_flash, flash_offset + bytes_read, &buffer).ok()?;
            bytes_read += b_read as u32;
            if b_read == 0 {
                break;
            }
        }
        drop(unlocked_flash);
        let _ = fs.vol_mgr().close_file(image_file);

        // Check crc
        let meta_data = meta_data();
        let upper_flash_u32 = unsafe { core::mem::transmute::<usize, &[u32; 0x2_0000]>(STORAGE) };
        let new_app_start_idx = meta_data.new_app as usize - STORAGE;
        let new_app_end_idx = new_app_start_idx + meta_data.new_app_len as usize;

        // Check magic number
        if meta_data.magic != 0x1c80_73ab_2085_3579 {
            return None; // We should never come here
        }

        // Check CRC of uploaded data
        let crc = stm32_crc(&upper_flash_u32[3..new_app_end_idx / 4]);
        if crc != meta_data.crc {
            return None; // We should never come here;
        }
        Some(check.new_sw_version())
    } else {
        None
    };
    fs.vol_mgr().close_dir(root_dir).ok()?;
    fs.vol_mgr().close_volume(volume).ok()?;
    result
}

pub fn install_and_restart() {
    let meta_data = meta_data();
    let func = unsafe { core::mem::transmute::<u32, fn()>(meta_data.copy_func) };
    func();

    #[allow(clippy::empty_loop)]
    loop {} // We should never come here;
}

const STORAGE: usize = 0x0808_0000;
const BEGIN_FLASH: usize = 0x0800_0000;

fn meta_data() -> &'static MetaDataV1 {
    unsafe { core::mem::transmute::<usize, &'static MetaDataV1>(STORAGE) }
}
