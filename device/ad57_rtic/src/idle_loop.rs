use defmt::trace;

use vario_display::{CStorageItems, StorageItem, DeviceEvent, Event, SdCardCmd};

use crate::{
    driver::{Eeprom, QEvents, delay_ms},
    utils::FileSys,
};

pub struct IdleLoop {
    eeprom: Eeprom,
    c_pers_items: CStorageItems,
    file_sys: FileSys,
    q_events: &'static QEvents,
}

impl IdleLoop {
    pub fn new(
        eeprom: Eeprom, 
        c_pers_items: CStorageItems,
        file_sys: FileSys,
        q_events: &'static QEvents,
    ) -> Self {
        IdleLoop {
            eeprom,
            c_pers_items,
            file_sys,
            q_events,
        }
    }

    pub fn idle_loop(&mut self) -> ! {
        loop {
            while self.c_pers_items.len() > 0 {
                let storage_item = self.c_pers_items.dequeue().unwrap();
                match storage_item {
                    StorageItem::EepromItem(item) => {
                        trace!("Stored id {:?}", item.id as u32);
                        self.eeprom.write_item(item).unwrap();
                    },
                    StorageItem::SdCardItem(item) => {
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
