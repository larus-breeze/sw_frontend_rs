//use defmt::trace;

use vario_display::CPersistenceItems;

use crate::driver::Eeprom;


pub struct IdleLoop {
    eeprom: Eeprom,
    c_pers_items: CPersistenceItems,
}

impl IdleLoop {
    pub fn new(eeprom: Eeprom, c_pers_items: CPersistenceItems) -> Self {
        IdleLoop { eeprom, c_pers_items }
    }

    pub fn main_loop(&mut self) -> ! {
        loop {

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}