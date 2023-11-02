use defmt::trace;

use vario_display::CPersistenceItems;

use crate::driver::Eeprom;

pub struct IdleLoop {
    eeprom: Eeprom,
    c_pers_items: CPersistenceItems,
}

impl IdleLoop {
    pub fn new(eeprom: Eeprom, c_pers_items: CPersistenceItems) -> Self {
        IdleLoop {
            eeprom,
            c_pers_items,
        }
    }

    pub fn idle_loop(&mut self) -> ! {
        loop {
            while self.c_pers_items.len() > 0 {
                let item = self.c_pers_items.dequeue().unwrap();
                trace!("Stored id {:?}", item.id as u32);
                self.eeprom.write_item(item).unwrap();
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
