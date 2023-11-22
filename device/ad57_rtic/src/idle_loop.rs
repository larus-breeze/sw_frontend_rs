use defmt::trace;

use stm32f4xx_hal::watchdog::IndependentWatchdog;

use vario_display::{CIdleEvents, IdleEvent, DeviceEvent, Event, SdCardCmd};
use crate::{
    driver::{Eeprom, QEvents, delay_ms},
    utils::FileSys,
};


pub struct IdleLoop {
    eeprom: Eeprom,
    c_pers_items: CIdleEvents,
    file_sys: FileSys,
    q_events: &'static QEvents,
    watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        eeprom: Eeprom, 
        c_pers_items: CIdleEvents,
        file_sys: FileSys,
        q_events: &'static QEvents,
        watchdog: IndependentWatchdog,
    ) -> Self {
        IdleLoop {
            eeprom,
            c_pers_items,
            file_sys,
            q_events,
            watchdog
        }
    }

    pub fn idle_loop(&mut self) -> ! {
        loop {
            while self.c_pers_items.len() > 0 {
                let idle_event = self.c_pers_items.dequeue().unwrap();
                match idle_event {
                    IdleEvent::EepromItem(item) => {
                        trace!("Stored id {:?}", item.id as u32);
                        self.eeprom.write_item(item).unwrap();
                    },
                    IdleEvent::SdCardItem(item) => {
                        match item {
                            SdCardCmd::SwUpdateAccepted => {
                                let event = Event::DeviceItem(DeviceEvent::UploadInProgress);
                                if self.q_events.enqueue(event).is_ok() {
                                    delay_ms(200); // Give the display a chance to update
                                    self.file_sys.install_and_restart();
                                }
                            },
                            _ => (),
                        }
                    },
                    IdleEvent::FeedTheDog => self.watchdog.feed(),
                }
            }

            if let Some(version) = self.file_sys.update_available() {
                let event = Event::DeviceItem(DeviceEvent::FwAvailable(version));
                let _ = self.q_events.enqueue(event);
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
