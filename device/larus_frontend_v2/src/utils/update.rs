use core::str;
use corelib::{stm32_crc, MetaDataV1, SwVersion, VersionCheck, SIZE_METADATA_V1};
use embedded_sdmmc::{Mode, ShortFileName, VolumeIdx};
use embedded_storage::nor_flash::NorFlash;
use heapless::{String, Vec};
use stm32h7xx_hal::flash::FlashExt;

use crate::{driver::*, HW_VERSION};

use super::SW_VERSION;

pub fn update_available() -> Option<SwVersion> {
    FILE_SYS.lock_during_use(|opt_fs| match opt_fs {
        Some(fs) => update_available_private(fs),
        None => None,
    })
}

fn update_available_private(fs: &mut FileSys) -> Option<SwVersion> {
    let mut volume = fs.vol_mgr().open_volume(VolumeIdx(0)).ok()?;
    let mut root_dir = volume.open_root_dir().ok()?;

    // read root directory, look after *.bin files
    let mut files = Vec::<ShortFileName, 20>::new();
    root_dir
        .iterate_dir(|entry| {
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

        let mut file = root_dir
            .open_file_in_dir(fname.as_str(), Mode::ReadOnly)
            .ok()?;
        let num_read = file.read(&mut buffer).ok()?;
        if num_read == SIZE_METADATA_V1 {
            check.analyse(fname.as_str(), &buffer)
        }
    }

    let result = if let Some(image_name) = check.new_image_name() {
        // a new image file was found
        let mut image_file = root_dir
            .open_file_in_dir(image_name.as_str(), Mode::ReadOnly)
            .ok()?;

        let dp = unsafe { stm32h7xx_hal::pac::Peripherals::steal() };
        let (_, opt_flash) = dp.FLASH.split();
        let mut flash = opt_flash?;
        let mut unlocked_flash = flash.unlocked();

        let image_size = image_file.length();

        // erase the flash region
        NorFlash::erase(&mut unlocked_flash, 0, image_size).ok()?;

        // write image file to flash memory
        let mut buffer = [0_u8; 512];
        let mut bytes_read = 0_u32;
        loop {
            let b_read = image_file.read(&mut buffer).ok()?;
            NorFlash::write(&mut unlocked_flash, bytes_read, &buffer).ok()?;
            bytes_read += b_read as u32;
            if b_read == 0 {
                break;
            }
        }
        drop(unlocked_flash);

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
    result
}

pub fn install_and_restart() {
    let meta_data = meta_data();
    let func = unsafe { core::mem::transmute::<u32, fn()>(meta_data.copy_func) };
    func();

    #[allow(clippy::empty_loop)]
    loop {} // We should never come here;
}

const STORAGE: usize = 0x0810_0000;

fn meta_data() -> &'static MetaDataV1 {
    unsafe { core::mem::transmute::<usize, &'static MetaDataV1>(STORAGE) }
}
