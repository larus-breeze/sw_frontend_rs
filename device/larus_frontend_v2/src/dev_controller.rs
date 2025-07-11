use crate::{driver::QEvents, timestamp_ms, CoreController};
use corelib::{
    basic_config::{MAX_RX_FRAMES, MAX_TX_FRAMES},
    CRxFrames, CoreModel, PIdleEvents, PTxFrames,
};
use stm32h7xx_hal::{adc, gpio::Pin, pac::ADC1, prelude::*};

type DevAdc = adc::Adc<ADC1, adc::Enabled>;
type SupplyPin = Pin<'A', 6>;
type TemperutrePin = Pin<'B', 1>;

#[cfg(feature = "test-panic")]
use corelib::FloatToMass;
#[cfg(feature = "test-panic")]
use defmt::trace;

pub struct DevController {
    core_controller: CoreController,
    q_events: &'static QEvents,            // key event queue
    c_rx_frames: CRxFrames<MAX_RX_FRAMES>, // can bus rx queue
    supply_pin: SupplyPin,
    adc: DevAdc,
    temperature_pin: TemperutrePin,
    tick_cnt: u32,
}

impl DevController {
    pub fn new(
        core_model: &mut CoreModel,
        q_events: &'static QEvents,
        p_idle_events: PIdleEvents,
        p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
        c_rx_frames: CRxFrames<MAX_RX_FRAMES>,
        mut adc: DevAdc,
        supply_pin: SupplyPin,
        temperature_pin: TemperutrePin,
    ) -> Self {
        let core_controller = CoreController::new(core_model, p_idle_events, p_tx_frames);
        let supply_pin = supply_pin.into_analog();
        let temperature_pin = temperature_pin.into_analog();
        adc.set_resolution(adc::Resolution::SixteenBit);
        DevController {
            core_controller,
            q_events,
            c_rx_frames,
            adc,
            supply_pin,
            temperature_pin,
            tick_cnt: 0,
        }
    }

    pub fn core(&mut self) -> &mut CoreController {
        &mut self.core_controller
    }

    pub fn set_ms(&mut self, time_ms: u16) {
        self.core_controller.set_ms(time_ms)
    }

    pub fn tick_1ms(&mut self, core_model: &mut CoreModel) -> bool {
        while let Some(event) = self.q_events.dequeue() {
            self.core_controller.event_handler(event, core_model);
        }
        while let Some(frame) = self.c_rx_frames.dequeue() {
            self.core_controller.read_can_frame(core_model, &frame);
        }

        let recalc = self.core_controller.tick_1ms(timestamp_ms(), core_model);

        self.tick_cnt = (self.tick_cnt + 1) % 100;
        match self.tick_cnt {
            1 => {
                let adc_value: u32 = self.adc.read(&mut self.supply_pin).unwrap();
                core_model.device.supply_voltage = adc_value as f32 * 0.00050354004;
            }
            2 => {
                let adc_value: u32 = self.adc.read(&mut self.temperature_pin).unwrap();
                core_model.device.temperature_pcb = adc_value as f32 * 0.000045776367 - 0.5;
            }
            _ => (),
        }

        #[cfg(feature = "test-panic")]
        {
            let cm = core_model;
            if cm.glider_data.pilot_weight > 100.9.kg() && cm.glider_data.pilot_weight < 101.1.kg()
            {
                trace!("panic!");
                panic!();
            }
            if cm.glider_data.pilot_weight > 101.9.kg() && cm.glider_data.pilot_weight < 102.1.kg()
            {
                trace!("watchdog reset");
                loop {}
            }
        }
        recalc
    }
}
