use defmt::trace;

use stm32f4xx_hal::{timer::monotonic::fugit::ExtU32, watchdog::IndependentWatchdog};

use crate::{
    driver::{delay_ms, QEvents, Storage},
    install_and_restart, update_available, ResetWatch,
};
use corelib::{CIdleEvents, DeviceEvent, Eeprom, Event, IdleEvent, SdCardCmd};

pub struct IdleLoop {
    eeprom: Eeprom<Storage>,
    c_idle_events: CIdleEvents,
    q_events: &'static QEvents,
    watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        eeprom: Eeprom<Storage>,
        c_idle_events: CIdleEvents,
        q_events: &'static QEvents,
        mut watchdog: IndependentWatchdog,
    ) -> Self {
        if let Some(version) = update_available() {
            // When software update is on the way, no watchdog is used
            let event = Event::DeviceItem(DeviceEvent::FwAvailable(version));
            let _ = q_events.enqueue(event);
            trace!("Update available: {}", version);
        } else {
            // Normal mode without update, activate watchdog
            watchdog.start(1000.millis());
            trace!("Start watchdog");
        }

        // Todo iter over eeprom items and restore them
        // Todo recalc_dlider()

        IdleLoop {
            eeprom,
            c_idle_events,
            q_events,
            watchdog,
        }
    }

    pub fn idle_loop(&mut self) -> ! {
        loop {
            while self.c_idle_events.len() > 0 {
                let idle_event = self.c_idle_events.dequeue().unwrap();
                match idle_event {
                    IdleEvent::SetEepromItem(item) => {
                        trace!("Stored id {:?}", item.id as u32);
                        self.eeprom.write_item(item).unwrap();
                    }
                    IdleEvent::ClearEepromItems(items_list) => {
                        self.eeprom.delete_items_list(items_list).unwrap();
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
                    IdleEvent::DateTime(date_time) => {
                        // Set date and time for PANIC.LOG
                        if let Some(reset_watch) = ResetWatch::init() {
                            reset_watch.date_time().clone_from(&date_time);
                        }
                    }
                    IdleEvent::ResetDevice(_reason) => {
                        trace!("Reset Device");
                        loop {} // Wait until watchdog reset the device
                    }
                }
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
