use corelib::Concat;
use defmt::trace;
use defmt_rtt as _;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable(); // Please not interrupts any more, we will reset device anyway

    let msg: Concat<128> = if let Some(location) = info.location() {
        Concat::from_str("panic in '")
            .push_str(location.file())
            .push_str("' line ")
            .push_u32(location.line())
            .push_str("\n")
    } else {
        Concat::from_str("panic without location info\n")
    };
    trace!("{}", msg.as_str());
    loop {}
}
