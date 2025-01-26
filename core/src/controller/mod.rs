mod helpers;

pub use helpers::{
    can_frame::*,
    can_ids::{audio_legacy, frontend_legacy, sensor_legacy, GenericId, SpecialId},
    CanActive, IntToDuration, NmeaBuffer, Scheduler, Tim,
};
use num::clamp;

mod editor;
pub use editor::{close_edit_frame, Editor};

mod menu;
pub use menu::{close_menu_display, MenuControl};

mod fw_update;
use fw_update::SwUpdateController;

mod tick_1s;
use tick_1s::*;

mod persistence;
pub use persistence::{store_persistence_ids, Echo, PersistenceId};

use crate::{
    basic_config::{CONTROLLER_TICK_RATE, MAX_TX_FRAMES},
    common::PTxFrames,
    flight_physics::Polar,
    model::{DisplayActive, EditMode, VarioModeControl},
    system_of_units::{FloatToSpeed, Speed},
    utils::{KeyEvent, PIdleEvents, Pt1},
    CoreModel, DeviceEvent, Editable, IdleEvent, SdCardCmd, VarioMode, polar_store,
};
use helpers::nmea_cyclic_200ms;

#[allow(unused_imports)]
use micromath::F32Ext;

use heapless::FnvIndexSet;

pub enum Direction {
    Forward,
    Backward,
}

pub enum ControlResult {
    Nothing,
    Edit(EditMode, Editable),
    NextDisplay(Direction),
}

pub type Callback = fn(cm: &mut CoreModel, cc: &mut CoreController);

pub enum Timer {
    Ticker1Hz,
    NmeaFast,
    PersistSetting,
    CloseEditFrame,
    CloseMenu,
}

pub const MAX_PERS_IDS: usize = 8;

pub struct CoreController {
    pub polar: Polar,
    sw_update: SwUpdateController,
    ms: u16,
    last_vario_mode: VarioMode,
    av2_climb_rate: Pt1<Speed>,
    av_speed_to_fly: Pt1<Speed>,
    pub nmea_buffer: NmeaBuffer,
    pub scheduler: Scheduler<5>,
    pub pers_vals: FnvIndexSet<PersistenceId, MAX_PERS_IDS>,
    pub nmea_vals: FnvIndexSet<PersistenceId, MAX_PERS_IDS>,
    p_idle_events: PIdleEvents,
    p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
}

impl CoreController {
    pub fn new(
        core_model: &mut CoreModel,
        p_idle_events: PIdleEvents,
        p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
    ) -> Self {
        let polar_idx = core_model.config.glider_idx as usize;
        let polar = Polar::new(&polar_store::POLARS[polar_idx], &mut core_model.glider_data);
        let av2_climb_rate = Pt1::new(
            0.0.m_s(),
            CONTROLLER_TICK_RATE,
            core_model.config.av2_climb_rate_tc,
        );
        let av_speed_to_fly = Pt1::new(
            0.0.m_s(),
            CONTROLLER_TICK_RATE,
            core_model.config.av_speed_to_fly_tc,
        );
        let mut scheduler = Scheduler::new([
            Tim::new(recalc_polar),
            Tim::new(nmea_cyclic_200ms),
            Tim::new(store_persistence_ids),
            Tim::new(close_edit_frame),
            Tim::new(close_menu_display),
        ]);
        scheduler.every(Timer::Ticker1Hz, 1.secs());
        scheduler.every(Timer::NmeaFast, 200.millis());
        Self {
            polar,
            ms: 0,
            last_vario_mode: VarioMode::Vario,
            sw_update: SwUpdateController::new(),
            av2_climb_rate,
            av_speed_to_fly,
            nmea_buffer: NmeaBuffer::new(),
            scheduler,
            nmea_vals: FnvIndexSet::new(),
            pers_vals: FnvIndexSet::new(),
            p_idle_events,
            p_tx_frames,
        }
    }

    pub fn device_action(&mut self, core_model: &mut CoreModel, device_event: &DeviceEvent) {
        match device_event {
            DeviceEvent::FwAvailable(_) => {
                core_model.config.last_display_active = core_model.config.display_active;
                core_model.config.display_active = DisplayActive::FirmwareUpdate;
            }
            DeviceEvent::UploadFinished => {
                core_model.config.display_active = core_model.config.last_display_active
            }

            _ => (),
        }
        if core_model.config.display_active == DisplayActive::FirmwareUpdate {
            self.sw_update.device_action(core_model, device_event);
            self.send_idle_event(IdleEvent::SdCardItem(SdCardCmd::SwUpdateAccepted))
        }
    }

    pub fn key_action(&mut self, cm: &mut CoreModel, mut key_event: KeyEvent) {
        editor::key_action(&mut key_event, cm, self);
        menu::key_action(&mut key_event, cm, self);
    }

    /// Call this latest after 1 ms
    ///
    /// time_ms is the absolute time. The internal counter is updated tick by tick until the time
    /// is caught up. A maximum of one callback routine is started in one call.
    pub fn tick_1ms(&mut self, time_ms: u16, cm: &mut CoreModel) -> bool {
        let mut recalc = false;
        while self.ms != time_ms {
            self.ms = self.ms.wrapping_add(1);
            match self.ms % 100 {
                0 => self.scheduler.tick_100ms().unwrap(), // call scheduler every 100ms
                1 => {
                    self.tick_100ms(cm); // call 100ms tick routine
                    recalc = true;
                }
                _ => {
                    // alternatively: execute a callback every ms as long as available
                    if let Some(callback) = self.scheduler.next_callback() {
                        callback(cm, self);
                        break; // max one call per ms
                    }
                }
            }
        }
        recalc
    }

    pub fn set_ms(&mut self, time_ms: u16) {
        self.ms = time_ms;
    }

    fn tick_100ms(&mut self, core_model: &mut CoreModel) {
        core_model.control.alive_ticks = core_model.control.alive_ticks.wrapping_add(1);

        if core_model.control.vario_mode == VarioMode::Vario {
            self.av2_climb_rate.tick(core_model.sensor.climb_rate);
            core_model.calculated.av2_climb_rate = self.av2_climb_rate.value();
        }

        // Calculate speed_to_fly and speed_to_fly_dif
        let climb_rate = core_model.sensor.climb_rate;
        let mc_cready = core_model.config.mc_cready;
        let sink_rate = self.polar.sink_rate(core_model.sensor.airspeed);
        core_model.calculated.speed_to_fly =
            self.polar.speed_to_fly(climb_rate - sink_rate, mc_cready);
        self.av_speed_to_fly
            .tick(core_model.calculated.speed_to_fly.ias());
        core_model.calculated.av_speed_to_fly = self.av_speed_to_fly.value();
        core_model.calculated.speed_to_fly_dif =
            core_model.calculated.av_speed_to_fly - core_model.sensor.airspeed.ias();

        // calculate sound parameters and push can frame to queue
        let cms = &core_model.sensor;
        let cmc = &core_model.config;
        let (frequency, continuous, gain) = match core_model.control.vario_mode {
            VarioMode::Vario => (
                (cmc.snd_center_freq * (cmc.snd_exp_mul * cms.climb_rate.to_m_s()).exp()) as u16,
                cms.climb_rate.to_m_s() < 0.0,
                cmc.volume,
            ),
            VarioMode::SpeedToFly => {
                let sped_to_fly_val = core_model.calculated.speed_to_fly_dif.to_km_h() / -10.0;
                if sped_to_fly_val < 1.0 && sped_to_fly_val > -1.0 {
                    (500, true, 0) // speed to fly is ok, so be quiet
                } else {
                    (
                        (cmc.snd_center_freq * (cmc.snd_exp_mul * sped_to_fly_val).exp()) as u16,
                        sped_to_fly_val < 0.0,
                        cmc.volume,
                    )
                }
            }
        };
        core_model.calculated.frequency =
            clamp(frequency, cmc.snd_min_freq as u16, cmc.snd_max_freq as u16);
        core_model.calculated.continuous = continuous;
        if gain != core_model.calculated.gain {
            core_model.calculated.gain = gain;
            let event = IdleEvent::SetGain(gain as u8);

            // send event to the idle loop, which handles the amplifier via i2c
            self.send_idle_event(event);
        }

        // create CAN frame
        let can_frame = core_model.can_frame_sound();
        // add CAN frame to queue, ignore if the queue is full
        let _ = self.p_tx_frames.enqueue(can_frame);
    }

    pub fn send_idle_event(&mut self, idle_event: IdleEvent) {
        let _ = self.p_idle_events.enqueue(idle_event);
    }
}
