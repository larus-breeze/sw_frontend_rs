use crate::SW_VERSION;
use super::file_sys::get_filesys;
use core::{mem::MaybeUninit, panic::PanicInfo};
use corelib::{Concat, CoreError, DateTime};
use defmt_rtt as _;
use embedded_sdmmc::{Mode, VolumeIdx};

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
    #[allow(clippy::new_ret_no_self)]
    pub fn new() {
        // SAFETY: The signature ensures that we are either dealing with an initialised data
        // structure or are initialising it.
        unsafe {
            let reset_watch = core::mem::transmute::<
                &'static mut MaybeUninit<ResetWatch>,
                &'static mut ResetWatch,
            >(&mut RESET_WATCH);

            if (reset_watch.signature == SIGNATURE) && (reset_watch.signature2 == SIGNATURE2) {
                let mut s = Concat::<50>::new();
                s = s.push_str("Reset - Firmware ").push_str(SW_VERSION.as_string().as_str());
                let _ = write_panic_msg(s.as_bytes());
            } else {
                reset_watch.signature = SIGNATURE;
                reset_watch.date_time = DateTime::new();
                reset_watch.signature2 = SIGNATURE2;
            }
        }
    }

    pub fn init() -> Option<&'static mut Self> {
        // SAFETY: The signature ensures that we are either dealing with an initialised data
        // structure or are initialising it.
        unsafe {
            let reset_watch = core::mem::transmute::<
                &'static mut MaybeUninit<ResetWatch>,
                &'static mut ResetWatch,
            >(&mut RESET_WATCH);

            if (reset_watch.signature == SIGNATURE) && (reset_watch.signature2 == SIGNATURE2) {
                Some(reset_watch)
            } else {
                None
            }
        }
    }

    pub fn date_time(&mut self) -> &mut DateTime {
        &mut self.date_time
    }
}

fn write_panic_msg(msg: &[u8]) -> Result<(), CoreError> {
    if let Some(fs) = get_filesys() {
        let volume = fs
            .vol_mgr()
            .open_volume(VolumeIdx(0))
            .map_err(|_| CoreError::SdCard)?;
        let root_dir = fs
            .vol_mgr()
            .open_root_dir(volume)
            .map_err(|_| CoreError::SdCard)?;
        let file = fs
            .vol_mgr()
            .open_file_in_dir(root_dir, "PANIC.LOG", Mode::ReadWriteCreateOrAppend)
            .map_err(|_| CoreError::SdCard)?;

        let dt = if let Some(rw) = ResetWatch::init() {
            rw.date_time().to_bytes()
        } else {
            DateTime::new().to_bytes()
        };

        fs.vol_mgr()
            .write(file, &dt)
            .map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr()
            .write(file, b" ")
            .map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr()
            .write(file, msg)
            .map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr()
            .write(file, b"\n")
            .map_err(|_| CoreError::SdCard)?;

        fs.vol_mgr().close_file(file).unwrap();
        fs.vol_mgr()
            .close_dir(root_dir)
            .map_err(|_| CoreError::SdCard)?;
        fs.vol_mgr()
            .close_volume(volume)
            .map_err(|_| CoreError::SdCard)?;
        Ok(())
    } else {
        Err(CoreError::SdCard)
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable(); // Please not interrupts any more, we will reset device anyway

    let msg = Concat::<200>::new();
    let msg = if let Some(location) = info.location() {
        msg.push_str("Panic in '")
            .push_str(location.file())
            .push_str("' line ")
            .push_u32(location.line())
    } else {
        msg.push_str("Panic without location info")
    };

    let _ = write_panic_msg(msg.as_bytes());

    loop {}
}
