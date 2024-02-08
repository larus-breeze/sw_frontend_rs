use defmt::trace;

use stm32f4xx_hal::{timer::monotonic::fugit::ExtU32, watchdog::IndependentWatchdog};

use crate::{
    driver::{delay_ms, FileSys, QEvents, Storage},
    install_and_restart, update_available,
};
use corelib::{CIdleEvents, DeviceEvent, Eeprom, Event, IdleEvent, SdCardCmd};

pub struct IdleLoop {
    eeprom: Eeprom<Storage>,
    c_idle_events: CIdleEvents,
    _file_sys: Option<FileSys>,
    q_events: &'static QEvents,
    watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        eeprom: Eeprom<Storage>,
        c_idle_events: CIdleEvents,
        mut file_sys: Option<FileSys>,
        q_events: &'static QEvents,
        mut watchdog: IndependentWatchdog,
    ) -> Self {
        if let Some(version) = update_available(&mut file_sys) {
            // When software update is on the way, no watchdog is used
            let event = Event::DeviceItem(DeviceEvent::FwAvailable(version));
            let _ = q_events.enqueue(event);
            trace!("Update available: {}", version);
        } else {
            // Normal mode without update, activate watchdog
            watchdog.start(1000.millis());
            trace!("Start watchdog");
        }

        IdleLoop {
            eeprom,
            c_idle_events,
            _file_sys: file_sys,
            q_events,
            watchdog,
        }
    }

    pub fn idle_loop(&mut self) -> ! {
        loop {
            while self.c_idle_events.len() > 0 {
                let idle_event = self.c_idle_events.dequeue().unwrap();
                match idle_event {
                    IdleEvent::EepromItem(item) => {
                        trace!("Stored id {:?}", item.id as u32);
                        self.eeprom.write_item(item).unwrap();
                    }
                    IdleEvent::FeedTheDog => self.watchdog.feed(),
                    IdleEvent::SetGain(_) => (), // vario sound on this device not suported
                    IdleEvent::SdCardItem(item) => {
                        match item {
                            SdCardCmd::SwUpdateAccepted => {
                                let event = Event::DeviceItem(DeviceEvent::UploadInProgress);
                                if self.q_events.enqueue(event).is_ok() {
                                    delay_ms(200); // Give the display a chance to update
                                    trace!("Sw update is accepted");
                                    install_and_restart();
                                }
                            }
                            SdCardCmd::SwUpdateCanceled => {
                                self.watchdog.start(ExtU32::millis(1000));
                                trace!("Start watchdog");
                            }
                        }
                    }
                }
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
