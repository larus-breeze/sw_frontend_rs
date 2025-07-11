use defmt::trace;

use crate::{driver::*, install_and_restart, update_available, DevController};
use corelib::{persist, CIdleEvents, CoreModel, DeviceEvent, Eeprom, Event, IdleEvent, PinState, SdCardCmd};
use fugit::ExtU32;
use stm32h7xx_hal::{
    gpio::{Output, Pin, PinState::High},
    device::I2C1,
    i2c::{Error as I2cError, I2c},
    independent_watchdog::IndependentWatchdog,
};

pub struct OutputPins {
    pub o1: Pin<'G', 2, Output>,
    pub o2: Pin<'K', 2, Output>,
}

impl OutputPins {
    pub fn new(o1: Pin<'G', 2>, o2: Pin<'K', 2>) -> Self {
        OutputPins {
            o1: o1.into_push_pull_output_in_state(High),
            o2: o2.into_push_pull_output_in_state(High),
        }
    }
}

pub struct IdleLoop {
    amplifier: Amplifier<I2cManager>,
    eeprom: Eeprom<Storage<I2cManager, I2cError>>,
    c_idle_events: CIdleEvents,
    q_events: &'static QEvents,
    watchdog: IndependentWatchdog,
    output_pins: OutputPins,
}

impl IdleLoop {
    pub fn new(
        output_pins: OutputPins,
        i2c: I2c<I2C1>,
        mut watchdog: IndependentWatchdog,
        c_idle_events: CIdleEvents,
        q_events: &'static QEvents,
        cm: &mut CoreModel,
        dc: &mut DevController,
    ) -> Self {
        let i2c = I2cManager::new(i2c);
        let mut eeprom = Storage::new(i2c).unwrap();
        let amplifier = Amplifier::new(I2cManager::clone());

        for item in eeprom.iter_over(corelib::EepromTopic::ConfigValues) {
            persist::restore_item(dc.core(), cm, item);
        }
        dc.core().recalc_glider(cm);

        if let Some(version) = update_available() {
            // When software update is on the way, no watchdog is used
            let event = Event::DeviceItem(DeviceEvent::FwAvailable(version));
            let _ = q_events.enqueue(event);
            trace!("Update available: {}", version);
        } else {
            // Normal mode without update, activate watchdog
            // Watchdog starts only in release builds
            if !cfg!(debug_assertions) {
                watchdog.start(1000.millis());
                trace!("Start watchdog");
            }
        }

        IdleLoop {
            output_pins,
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
                    IdleEvent::SetEepromItem(item) => {
                        trace!("Save to EEPROM '{:?}'", item.id);
                        self.eeprom.write_item(item).unwrap();
                    }
                    IdleEvent::ClearEepromItems(items_list) => {
                        self.eeprom.delete_items_list(items_list).unwrap();
                    }
                    IdleEvent::FeedTheDog => self.watchdog.feed(),
                    IdleEvent::SetGain(gain) => {
                        self.amplifier.set_gain(gain);
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
                    IdleEvent::ResetDevice(_reason) => {
                        trace!("Reset Device");
                        loop {} // Wait until watchdog reset the device
                    }
                    IdleEvent::Output1(state) => {
                        // trace!("out1 {}", state as u32);
                        match state {
                            PinState::High => self.output_pins.o1.set_high(),
                            PinState::Low => self.output_pins.o1.set_low(),
                        }
                    }
                    IdleEvent::Output2(state) => {
                        // trace!("out2 {}", state as u32);
                        match state {
                            PinState::High => self.output_pins.o2.set_high(),
                            PinState::Low => self.output_pins.o2.set_low(),
                        }
                    }
                }
            }

            // Sleep and save power at the end
            rtic::export::wfi()
        }
    }
}
