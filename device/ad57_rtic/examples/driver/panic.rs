use corelib::CoreError;
use super::file_sys::get_filesys;
use embedded_sdmmc::{VolumeIdx, Mode};
use corelib::{Concat, DateTime};
use defmt::trace;
use defmt_rtt as _;
use core::{
    mem::MaybeUninit, panic::PanicInfo,
};

#[repr(C)]
#[derive(Debug)]
pub struct ResetWatch {
    signature: u32,
    date_time: DateTime,
    signature2: u32,
}

const SIGNATURE: u32 = 0x09d2_889f;
const SIGNATURE2: u32 = 0x1234_5678;

#[link_section = ".ccmram.BUFFERS"]
static mut RESET_WATCH: MaybeUninit<ResetWatch> = unsafe { MaybeUninit::uninit().assume_init() };

impl ResetWatch {
pub fn init() -> &'static mut Self {
        // SAFETY: The signature ensures that we are either dealing with an initialised data 
        // structure or are initialising it.
        unsafe {
            let reset_watch = 
                core::mem::transmute::<&'static mut MaybeUninit<ResetWatch>, &'static mut ResetWatch>(&mut RESET_WATCH);
            
            if (reset_watch.signature == SIGNATURE) && (reset_watch.signature2 == SIGNATURE2) {
                let _ = write_panic_msg(b"Reset");
            } else {
                trace!("Initializing panic buffer...");
                reset_watch.signature = SIGNATURE;
                reset_watch.date_time = DateTime::new();
                reset_watch.signature2 = SIGNATURE2;
            }
            reset_watch
        }
    }

    pub fn date_time(&mut self) -> &mut DateTime {
        &mut self.date_time
    }
}

fn write_panic_msg(msg: &[u8]) -> Result<(), CoreError> {
    if let Some(fs) = get_filesys() {
        let mut volume = fs.vol_mgr().open_volume(VolumeIdx(0)).map_err(|_| CoreError::SdCard)?;
        let root_dir = fs.vol_mgr().open_root_dir(volume).map_err(|_| CoreError::SdCard)?;
        let mut file = fs.vol_mgr().open_file_in_dir(root_dir, "PANIC.LOG", Mode::ReadWriteCreateOrAppend).map_err(|_| CoreError::SdCard)?;

        let rw = ResetWatch::init();
        let dt = rw.date_time().to_bytes();

        fs.vol_mgr().write(file, &dt).map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr().write(file, b" ").map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr().write(file, msg).map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr().write(file, b"\n").map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr().close_file(file).unwrap();
        Ok(())
    } else {
        Err(CoreError::SdCard)
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    trace!("PANIC!");
    cortex_m::interrupt::disable(); // Please not interrupts any more, we will reset device anyway

    let msg = Concat::<200>::new();
    let msg = if let Some(location) = info.location() {
        msg
            .push_str("Panic in '")
            .push_str(location.file())
            .push_str("' line ")
            .push_u32(location.line())
    } else {
        msg.push_str("Panic without location info")    
    };

    let _ = write_panic_msg(msg.as_bytes());

    loop {}
}
