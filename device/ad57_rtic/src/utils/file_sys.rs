
use heapless::{String, Vec};
use defmt::*;
use stm32f4xx_hal::{
    rcc::Clocks,
    pac::SDIO,
    gpio::Pin,
    sdio::{SdCard, Sdio},
    flash::{FlashExt, LockedFlash},
};
use fatfs::{
    NullTimeProvider, LossyOemCpConverter, FileSystem, FsOptions, Read,
};
use embedded_storage::nor_flash::NorFlash;
use crate::{
    driver::FileIo,
    utils::{stm32_crc, DevError},
};
use vario_display::{SwVersion, SW_VERSION};

const BEGIN_FLASH: usize = 0x0800_0000;
const BEGIN_UPPER_FLASH: usize = 0x0808_0000;

/// The meta data structure describes what is contained in the upload
#[repr(C)]
#[derive(Default)]
struct MetaData {
    magic: u64,
    crc: u32,
    meta_version: u32,
    storage_addr: usize,
    hw_version: [u8; 4],
    sw_version: SwVersion,
    copy_func: usize,
    new_app: usize,
    new_app_len: usize,
    new_app_dest: usize, 
}

/// Pin definition for SDIO Peripheral
pub type SdioPins = (
    Pin<'C', 12>,
    Pin<'D', 2>,
    Pin<'C', 8>,
    Pin<'C', 9>,
    Pin<'C', 10>,
    Pin<'C', 11>,
);

pub enum FirmwarUpadate {
    Available(SwVersion),
    NotAvailable,
    ToMuchRequests,
}

/// FileSys handles access to the flash file system
///
/// Note: The current solution can only be used while booting. It neither supports the change of sd
/// cards nor can it be used while the real-time system is in use. 
pub struct FileSys {
    image_name: Option<String<12>>,
    image_size: usize,
    update_available: FirmwarUpadate,
    meta_data: MetaData,
    fs: Option<fatfs::FileSystem<FileIo, NullTimeProvider, LossyOemCpConverter>>,
}

impl FileSys {
    /// Create FileSys
    pub fn new(
        dp_sdio: SDIO,
        clocks: &Clocks,
        pins: SdioPins,
    ) -> Self {
        let mut fs = None;
        let sdio: Sdio<SdCard> = Sdio::new(dp_sdio, pins, clocks);
        if let Ok(fileio) = FileIo::new(sdio) {
            if let Ok(fs_) = FileSystem::new(
                fileio, 
                FsOptions::new()) {
                fs = Some(fs_);
            }
        }

        let mut file_sys = FileSys {
            image_name: None,
            image_size: 0,
            update_available: FirmwarUpadate::NotAvailable,
            meta_data: MetaData::default(),
            fs, 
        };
        if file_sys.find_image() {
            match file_sys.copy_image() {
                Ok(()) => file_sys.update_available = FirmwarUpadate::Available(file_sys.meta_data.sw_version),
                Err(_) => file_sys.image_name = None,
            }
            }
        file_sys
    }

    /// If an update is available, its version is returned
    pub fn update_available(&mut self) -> FirmwarUpadate {
        let r = match self.update_available {
            FirmwarUpadate::Available(version) => {
                self.update_available = FirmwarUpadate::ToMuchRequests;
                FirmwarUpadate::Available(version)
            },
            FirmwarUpadate::NotAvailable => {
                self.update_available = FirmwarUpadate::ToMuchRequests;
                FirmwarUpadate::NotAvailable
            },
            FirmwarUpadate::ToMuchRequests => FirmwarUpadate::ToMuchRequests,
        };
        r
    }

    /// Copies the image to the upper flash area
    /// 
    /// Note: This routine currently only works during the boot process.
    pub fn copy_image(&mut self) -> Result<(), DevError> {
        if let Some(fs) = &self.fs {
            let mut buf = [0_u8;512];
            let mut bytes_read: u32 = 0;

            let root_dir = fs.root_dir();
            if self.image_name.is_none() {
                return Err(DevError::NoItemAvailable)
            }
            let path: &str = self.image_name.as_ref().unwrap().as_str();
            let mut image_file = root_dir.open_file(path)?;
        
            let dp = unsafe {stm32f4xx_hal::pac::Peripherals::steal()};
            let mut flash = LockedFlash::new(dp.FLASH);
            let mut unlocked_flash = flash.unlocked();
            let flash_offset = self.meta_data.storage_addr - BEGIN_FLASH;
            
            NorFlash::erase(
                &mut unlocked_flash, 
                flash_offset as u32,
                (flash_offset + self.image_size) as u32).unwrap();
        
            loop {
                let b_read = image_file.read(&mut buf)?;
                NorFlash::write(
                    &mut unlocked_flash, 
                    flash_offset as u32 + bytes_read,
                    &buf).unwrap();
                bytes_read += b_read as u32;
                if b_read == 0 {
                    break;
                }
            }
            drop(unlocked_flash);

            let upper_flash_u32 =  unsafe { core::mem::transmute::<usize, &[u32; 0x2_0000]>(BEGIN_UPPER_FLASH) };
            let new_app_end_idx = self.meta_data.new_app - BEGIN_UPPER_FLASH + self.meta_data.new_app_len;

            let crc = stm32_crc(&upper_flash_u32[3..new_app_end_idx/4]);       
            if crc != self.meta_data.crc {
                trace!("CRC Check failed");
                self.image_name = None;
                return Err(DevError::CrcError);
            }

            trace!("Image written, {} Bytes", bytes_read);
        }
        Ok((    ))
    }

    /// Install first the new software and then start the new application
    pub fn install_and_restart(&self) {
        if self.image_name.is_some() {
            // This call starts the update. First the consistency of the loaded data is checked, then the 
            // data from the upper flash area is copied to the lower one and then the new app is started.
            let func = unsafe { core::mem::transmute::<usize, fn()>(self.meta_data.copy_func) };
            func();
        }
    }

    /// Looks on the sd card if a image is available
    /// 
    /// Note: This routine currently only works during the boot process.
    fn find_image(&mut self) -> bool {
        if let Some(fs) = &self.fs {
            let root_dir = fs.root_dir();
            let mut fn_vec: Vec<u8, 12> = Vec::new();
            for dir_entry in root_dir.iter() {
                let entry = dir_entry.unwrap();
                if entry.short_file_name_as_bytes() == "IMAGE.BIN".as_bytes() {
                    fn_vec = Vec::from_slice(entry.short_file_name_as_bytes()).unwrap();
                    self.image_size = entry.len() as usize;
                }
            }
            let file_name: String<12> = String::from_utf8(fn_vec).unwrap_or(String::new());
            trace!("Image file name {}", file_name.as_str());
            const META_DATA_SIZE: usize = core::mem::size_of::<MetaData>();
            let meta_data_as_u8arr = unsafe {core::mem::transmute::<&mut MetaData, &mut [u8; META_DATA_SIZE]>(&mut self.meta_data) };
            if let Ok(mut file) = root_dir.open_file(file_name.as_str()) {
                let _ = file.read_exact(meta_data_as_u8arr); // silently ignore errors, signature will be checked anyway
            }
            if self.meta_data.magic == 0x1c80_73ab_2085_3579 && self.meta_data.sw_version != SW_VERSION {
                self.image_name = Some(file_name);
            }
        }
        self.image_name.is_some()
    }
}
