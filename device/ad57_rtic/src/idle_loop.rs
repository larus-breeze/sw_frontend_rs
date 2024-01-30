use defmt::trace;

use stm32f4xx_hal::{timer::monotonic::fugit::ExtU32, watchdog::IndependentWatchdog};

use crate::{
    driver::{delay_ms, QEvents, Storage},
    utils::{FileSys, FirmwarUpadate},
};
use corelib::{CIdleEvents, DeviceEvent, Eeprom, Event, IdleEvent, SdCardCmd};

pub struct IdleLoop {
    eeprom: Eeprom<Storage>,
    c_idle_events: CIdleEvents,
    file_sys: FileSys,
    q_events: &'static QEvents,
    watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        eeprom: Eeprom<Storage>,
        c_idle_events: CIdleEvents,
        file_sys: FileSys,
        q_events: &'static QEvents,
        watchdog: IndependentWatchdog,
    ) -> Self {
        IdleLoop {
            eeprom,
            c_idle_events,
            file_sys,
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
                    IdleEvent::SdCardItem(item) => {
                        match item {
                            SdCardCmd::SwUpdateAccepted => {
                                let event = Event::DeviceItem(DeviceEvent::UploadInProgress);
                                if self.q_events.enqueue(event).is_ok() {
                                    delay_ms(200); // Give the display a chance to update
                                    self.file_sys.install_and_restart();
                                }
                            }
                            SdCardCmd::SwUpdateCanceled => {
                                self.watchdog.start(ExtU32::millis(1000));
                                trace!("Start watchdog");
                            }
                        }
                    }
                    IdleEvent::FeedTheDog => self.watchdog.feed(),
                    IdleEvent::SetGain(_) => (), // vario sound on this device not suported
                }
            }

            match self.file_sys.update_available() {
                FirmwarUpadate::Available(version) => {
                    let event = Event::DeviceItem(DeviceEvent::FwAvailable(version));
                    let _ = self.q_events.enqueue(event);
                }
                FirmwarUpadate::NotAvailable => {
                    self.watchdog.start(ExtU32::millis(1000));
                    trace!("Start watchdog");
                }
                FirmwarUpadate::ToMuchRequests => (),
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
