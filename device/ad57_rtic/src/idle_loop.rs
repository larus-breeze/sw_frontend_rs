use defmt::trace;

use vario_display::{CStorageItems, StorageItem};

use crate::driver::Eeprom;

pub struct IdleLoop {
    eeprom: Eeprom,
    c_pers_items: CStorageItems,
}

impl IdleLoop {
    pub fn new(eeprom: Eeprom, c_pers_items: CStorageItems) -> Self {
        IdleLoop {
            eeprom,
            c_pers_items,
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
                    StorageItem::SdCardItem(_) => (),
                }
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
