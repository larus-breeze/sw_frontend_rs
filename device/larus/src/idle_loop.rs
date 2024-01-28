use core::cell::RefCell;

use defmt::trace;

use crate::driver::*;
use corelib::{CIdleEvents, CoreModel, Eeprom, IdleEvent};
use stm32h7xx_hal::{
    device::I2C1,
    i2c::{Error as I2cError, I2c},
    independent_watchdog::IndependentWatchdog,
    prelude::*,
}; //Event, DeviceEvent, SdCardCmd};

static mut I2C_REF: Option<RefCell<I2c<I2C1>>> = None;
pub struct IdleLoop {
    amplifier: Amplifier<I2cManager<'static>>,
    eeprom: Eeprom<Storage<I2cManager<'static>, I2cError>>,
    c_idle_events: CIdleEvents,
    _q_events: &'static QEvents,
    //    file_sys: FileSys,
    watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        i2c: I2c<I2C1>,
        c_idle_events: CIdleEvents,
        q_events: &'static QEvents,
        //        file_sys: FileSys,
        mut watchdog: IndependentWatchdog,
        core_model: &mut CoreModel,
    ) -> Self {
        // I found no other solution
        let (mut eeprom, amplifier) = unsafe {
            I2C_REF.replace(RefCell::new(i2c));
            (
                Storage::new(I2cManager::new(&I2C_REF.as_ref().unwrap())).unwrap(),
                Amplifier::new(I2cManager::new(&I2C_REF.as_ref().unwrap())),
            )
        };
        for item in eeprom.iter_over(corelib::EepromTopic::ConfigValues) {
            core_model.restore_persistent_item(item);
        }

        watchdog.start(1000.millis());
        trace!("Start watchdog");

        IdleLoop {
            amplifier,
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
                    IdleEvent::SetGain(gain) => {
                        self.amplifier.set_gain(gain);
                        trace!("set_gain() {}", gain);
                    }
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
