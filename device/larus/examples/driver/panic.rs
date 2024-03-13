use corelib::{Concat, DateTime};
use defmt::trace;
use defmt_rtt as _;
use embedded_storage::nor_flash::NorFlash;
use stm32h7xx_hal::{flash::FlashExt, independent_watchdog::IndependentWatchdog, pac, prelude::*};
use core::{
    mem::MaybeUninit,
    panic::PanicInfo,
    ptr::{addr_of, addr_of_mut},
    sync::atomic::{AtomicU32, AtomicU8, AtomicUsize, Ordering},
};

const ERR_MSG_LEN: usize = 200;
const BUFF_LEN: usize = ERR_MSG_LEN + 20;

#[repr(C)]
pub struct PanicBuffer {
    signature: u32,
    date_time: DateTime,
    len: usize,
    buf: [u8; BUFF_LEN],
}

const SIGNATURE: u32 = 0x09d2_889f;

#[link_section = ".axisram.AXISRAM"]
static mut PANIC_BUFFER: MaybeUninit<PanicBuffer> = MaybeUninit::uninit();


impl PanicBuffer {
    pub fn init() -> &'static mut Self {
        // SAFETY: The signature ensures that we are either dealing with an initialised data 
        // structure or are initialising it.
        unsafe {
            let uninit = &mut PANIC_BUFFER;
            let mut panic_buffer = 
                core::mem::transmute::<&'static mut MaybeUninit<PanicBuffer>, &'static mut PanicBuffer>(uninit);

            if panic_buffer.signature != SIGNATURE {
                trace!("Initializing panic buffer...");
                panic_buffer.signature = SIGNATURE;
                panic_buffer.date_time = DateTime::new();
                panic_buffer.len = 0;
            }
            panic_buffer
        }
    }

    pub fn date_time(&mut self) -> &mut DateTime {
        &mut self.date_time
    }

    fn write_slice(&mut self, slice: &[u8]) {
        let ptr = &self.buf.as_mut_ptr();
        for b in slice {
            if self.len >= BUFF_LEN {
                return
            }
            // SAFETY: We have checked before, that we write inside the array 
            unsafe { 
                ptr.add(self.len).write_volatile((*b).into());
            };
            self.len += 1;
        }
    }

    pub fn write_str(&mut self, s: &str) {
        self.write_slice(s.as_bytes());
    }

    pub fn write_date_time(&mut self) {
        let dt = self.date_time.to_bytes();
        self.write_slice(&dt);
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn has_content(&self) -> bool {
        self.len > 0
    }

    pub fn content(&self) -> &[u8] {
        &self.buf[0..self.len]
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable(); // Please not interrupts any more, we will reset device anyway

    let msg: Concat::<ERR_MSG_LEN> = if let Some(location) = info.location() {
        Concat::from_str(" panic in '")
            .push_str(location.file())
            .push_str("' line ")
            .push_u32(location.line())
            .push_str("\n")
    } else {
        Concat::from_str(" panic without location info\n")
    };

    let mut panic_buffer = PanicBuffer::init();
    panic_buffer.write_date_time();
    panic_buffer.write_str(msg.as_str());

    trace!("{}", unsafe { core::str::from_utf8_unchecked(panic_buffer.content())});

    loop {}
}
