use corelib::Concat;
use defmt::trace;
use defmt_rtt as _;
use embedded_storage::nor_flash::NorFlash;
use stm32f4xx_hal::flash::{FlashExt, LockedFlash};

const FLASH_START: usize = 0x0800_0000;
const PANIC_BUF: usize = 0x0807_c000;
const ERR_MSG_LEN: usize = 128;
const PANIC_BUF_END: usize = 0x0808_0000 - ERR_MSG_LEN;

fn get_ptr_end() -> usize {
    let mut ptr = PANIC_BUF;
    while ptr < PANIC_BUF_END {
        let b = unsafe { *(ptr as *const u8) };
        if b == 0xff {
            break;
        }
        ptr += 1;
    }
    ptr
}

fn write_to_flash(msg: &str) {
    let ptr = get_ptr_end();
    let dp = unsafe { stm32f4xx_hal::pac::Peripherals::steal() };
    let mut flash = LockedFlash::new(dp.FLASH);
    let mut uflash = flash.unlocked();
    let _ = NorFlash::write(&mut uflash, (ptr - FLASH_START) as u32, msg.as_bytes());
    drop(uflash);
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable(); // Please not interrupts any more, we will reset device anyway

    let msg: Concat<ERR_MSG_LEN> = if let Some(location) = info.location() {
        Concat::from_str("panic in '")
            .push_str(location.file())
            .push_str("' line ")
            .push_u32(location.line())
            .push_str("\n")
    } else {
        Concat::from_str("panic without location info\n")
    };
    trace!("{}", msg.as_str());
    write_to_flash(msg.as_str());
    loop {}
}

#[allow(unused)]
pub fn get_error_log() -> &'static [u8] {
    let ptr = get_ptr_end();
    let upper_flash_u8 =
        unsafe { core::mem::transmute::<usize, &[u8; PANIC_BUF_END - PANIC_BUF]>(PANIC_BUF) };
    &upper_flash_u8[0..ptr - PANIC_BUF]
}
