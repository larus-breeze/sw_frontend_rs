use core::cell::RefCell;

use defmt::trace;

use crate::{driver::*, install_and_restart, update_available};
use corelib::{CIdleEvents, CoreModel, DeviceEvent, Eeprom, Event, IdleEvent, SdCardCmd};
use fugit::ExtU32;
use stm32h7xx_hal::{
    device::I2C1,
    i2c::{Error as I2cError, I2c},
    independent_watchdog::IndependentWatchdog,
};

static mut I2C_REF: Option<RefCell<I2c<I2C1>>> = None;
pub struct IdleLoop {
    amplifier: Amplifier<I2cManager<'static>>,
    eeprom: Eeprom<Storage<I2cManager<'static>, I2cError>>,
    c_idle_events: CIdleEvents,
    q_events: &'static QEvents,
    watchdog: IndependentWatchdog,
}

impl IdleLoop {
    pub fn new(
        i2c: I2c<I2C1>,
        mut watchdog: IndependentWatchdog,
        c_idle_events: CIdleEvents,
        q_events: &'static QEvents,
        core_model: &mut CoreModel,
    ) -> Self {
        // I found no other solution
        let (mut eeprom, amplifier) = unsafe {
            I2C_REF.replace(RefCell::new(i2c));
            (
                Storage::new(I2cManager::new(I2C_REF.as_ref().unwrap())).unwrap(),
                Amplifier::new(I2cManager::new(I2C_REF.as_ref().unwrap())),
            )
        };
        for item in eeprom.iter_over(corelib::EepromTopic::ConfigValues) {
            core_model.restore_persistent_item(item);
        }

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

        IdleLoop {
            amplifier,
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
                    IdleEvent::EepromItem(item) => {
                        trace!("Stored id {:?}", item.id as u32);
                        self.eeprom.write_item(item).unwrap();
                    }
                    IdleEvent::FeedTheDog => self.watchdog.feed(),
                    IdleEvent::SetGain(gain) => {
                        self.amplifier.set_gain(gain);
                        trace!("set_gain() {}", gain);
                    }
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
                            // Should never happen, Updates are always accepted
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
                }
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
