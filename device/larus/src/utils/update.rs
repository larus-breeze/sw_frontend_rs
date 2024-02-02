use embedded_sdmmc::{VolumeIdx, ShortFileName, Mode};
use defmt::trace;
use corelib::{MetaDataV1, SwVersion, VersionCheck, SIZE_METADATA_V1};
use heapless::{String, Vec};
use core::str;
use embedded_storage::nor_flash::NorFlash;
use stm32h7xx_hal::flash::FlashExt;

use crate::{driver::*, HW_VERSION};

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

pub fn update_available(file_sys: &mut Option<FileSys>) -> Option<SwVersion> {
    if file_sys.is_none() {
        return None
    };
    // open filesystem
    let mut volume = file_sys.as_mut()?.fat().get_volume(VolumeIdx(0)).ok()?;
    let root_dir = file_sys.as_mut()?.fat().open_root_dir(&volume).ok()?;
    let fatfs = file_sys.as_mut()?.fat();

    // read root directory, look after *.bin files
    let mut files = Vec::<ShortFileName, 20>::new();
    fatfs
        .iterate_dir(&volume, &root_dir, |entry| {
            if entry.name.extension() == [66, 73, 78] && // BIN
                entry.size > SIZE_METADATA_V1 as u32 {
                    let _ = files.push(entry.name.clone());
                }
        })
        .ok()?;

    // check the *.bin files if there is something interesting there
    let mut check = VersionCheck::new(HW_VERSION);
    let mut buffer = [0_u8; SIZE_METADATA_V1];
    for name in files {
        let mut fname = String::<12>::new();
        let base = unsafe { str::from_utf8_unchecked(name.base_name()) };
        let _ = fname.push_str(base);
        let _ = fname.push('.');
        let ext = unsafe { str::from_utf8_unchecked(name.extension()) };
        let _ = fname.push_str(ext);

        let mut file = fatfs.open_file_in_dir(
            &mut volume, 
            &root_dir, 
            fname.as_str(), 
            Mode::ReadOnly).ok()?;
        let num_read = fatfs.read(
            &volume, 
            &mut file, 
            &mut buffer).ok()?;
        if num_read == SIZE_METADATA_V1 {
            check.analyse(fname.as_str(), &buffer)
        }
        let _ = fatfs.close_file(&volume, file);
    }

    let result = if let Some(image_name) = check.new_image_name()  {
        // a new image file was found
        let mut image_file = fatfs.open_file_in_dir(
            &mut volume, 
            &root_dir, 
            image_name.as_str(), 
            Mode::ReadOnly).ok()?;
    
        let dp = unsafe { stm32h7xx_hal::pac::Peripherals::steal() };
        let (_, opt_flash) = dp.FLASH.split();
        let mut flash = opt_flash?;
        let mut unlocked_flash = flash.unlocked();

        let image_size = image_file.length();

        // erase the flash region
        NorFlash::erase(
            &mut unlocked_flash,
            0,
            image_size,
        ).ok()?;

        // write image file to flash memory
        let mut buffer = [0_u8; 512];
        let mut bytes_read = 0_u32;
        loop {
            let b_read = fatfs.read(&volume, &mut image_file, &mut buffer).ok()?;
            NorFlash::write(
                &mut unlocked_flash, 
                bytes_read, 
                &buffer).ok()?;
            bytes_read += b_read as u32;
            if b_read == 0 {
                break;
            }
        }
        let _ = fatfs.close_file(&volume, image_file);
        drop(unlocked_flash);

        // Check crc
        let meta_data = meta_data();
        let upper_flash_u32 =  unsafe { core::mem::transmute::<usize, &[u32; 0x2_0000]>(STORAGE) };
        let new_app_start_idx = meta_data.new_app as usize - STORAGE;
        let new_app_end_idx = new_app_start_idx + meta_data.new_app_len as usize;
    
        // Check magic number
        if meta_data.magic != 0x1c80_73ab_2085_3579 {
            loop {}; // We should never come here
        }

        /*trace!("first {}", upper_flash_u32[3]);
        delay_ms(9_000);
        trace!("jetzt {}", new_app_end_idx/4 - 1);
        delay_ms(1_000);
        trace!("last {}", upper_flash_u32[25000]);
        trace!("last {}", upper_flash_u32[24346]);
        //trace!("last {}", upper_flash_u32[new_app_end_idx/4 - 1]);
        trace!("danach");
        loop {}*/
        
    
        // Check CRC of uploaded data
        let crc = stm32_crc(&upper_flash_u32[3..new_app_end_idx/4]);
        if crc != meta_data.crc {
            return None; // We should never come here;
        }
        trace!("CRC ok, copy image to flash finished");
        Some(check.new_sw_version())
    } else {
        None
    };
    fatfs.close_dir(&volume, root_dir);
    result
}

pub fn install_and_restart() {
    let meta_data = meta_data();
    let func = unsafe { core::mem::transmute::<u32, fn()>(meta_data.copy_func) };
    func();

    loop {}; // We should never come here;
}

const STORAGE: usize = 0x0810_0000;

fn meta_data() -> &'static MetaDataV1 {
    unsafe { core::mem::transmute::<usize, &'static MetaDataV1>(STORAGE) }
}
