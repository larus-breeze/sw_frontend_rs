use defmt::trace;

use stm32h7xx_hal::{
    independent_watchdog::IndependentWatchdog,
    prelude::*,
};

use crate::driver::{QEvents, Storage};
use corelib::{CIdleEvents, Eeprom, IdleEvent}; //Event, DeviceEvent, SdCardCmd};

pub struct IdleLoop {
    eeprom: Eeprom<Storage>,
    c_idle_events: CIdleEvents,
    _q_events: &'static QEvents,
    //    file_sys: FileSys,
        watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        eeprom: Eeprom<Storage>,
        c_idle_events: CIdleEvents,
        q_events: &'static QEvents,
        //        file_sys: FileSys,
        mut watchdog: IndependentWatchdog,
    ) -> Self {
        watchdog.start(1000.millis());
        trace!("Start watchdog");
        IdleLoop {
            eeprom,
            c_idle_events,
            _q_events: q_events,
            //            file_sys,
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
                    _ => (),
                    /*IdleEvent::SdCardItem(item) => {
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
                    }*/
                }
            }

            /*match self.file_sys.update_available() {
                FirmwarUpadate::Available(version) => {
                    let event = Event::DeviceItem(DeviceEvent::FwAvailable(version));
                    let _ = self.q_events.enqueue(event);
                }
                FirmwarUpadate::NotAvailable => {
                    self.watchdog.start(ExtU32::millis(1000));
                    trace!("Start watchdog");
                }
                FirmwarUpadate::ToMuchRequests => (),
            }*/

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
