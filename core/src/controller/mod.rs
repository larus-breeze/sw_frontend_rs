mod helpers;
pub use helpers::{
    can_frame::*,
    can_ids::{audio_legacy, frontend_legacy, sensor_legacy, GenericId, SpecialId},
    CanActive, CanConfigId, IntToDuration, NmeaBuffer, RemoteConfig, Scheduler, Tim,
};
pub(crate) use helpers::{
    DrainControl, FlashControl, GearAlarmControl, InPinFunction, InTogglePinFunction, SpeedToFlyControl, GearPins,
    PIN_IN_CLOSE, PIN_IN_OPEN, PIN_IN_TOGGLE, PIN_NONE, PIN_OUT_CLOSE, PIN_OUT_OPEN, ONE_PIN_MODE, TWO_PIN_MODE,
};

mod editor;
pub use editor::{close_edit_frame, Editor};

mod menu;
pub use menu::{close_menu_display, MenuControl};

mod fw_update;
use fw_update::SwUpdateController;

mod sound;
pub(crate) use sound::SoundControl;

mod tick_1s;
use tick_1s::*;

pub mod persist;
pub use persist::{store_persistence_ids, Echo, PersistenceId};

use crate::{
    basic_config::{CONTROLLER_TICK_RATE, MAX_TX_FRAMES},
    common::PTxFrames,
    flight_physics::Polar,
    model::{DisplayActive, EditMode, VarioModeControl},
    system_of_units::{FloatToSpeed, Speed},
    utils::{KeyEvent, PIdleEvents, Pt1},
    CoreModel, DeviceEvent, Editable, Event, IdleEvent, InputPinState, PinState, SdCardCmd,
    VarioMode,
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
    pub drain_control: DrainControl,
    pub flash_control: FlashControl,
    pub speed_to_fly_control: SpeedToFlyControl,
    pub gear_alarm_control: GearAlarmControl,
    sw_update: SwUpdateController,
    sound_control: SoundControl,
    ms: u16,
    last_vario_mode: VarioMode,
    av2_climb_rate: Pt1<Speed>,
    av_speed_to_fly: Pt1<Speed>,
    av_supply_voltage: Pt1<f32>,
    pub nmea_buffer: NmeaBuffer,
    pub scheduler: Scheduler<5>,
    pub pers_vals: FnvIndexSet<PersistenceId, MAX_PERS_IDS>,
    pub nmea_vals: FnvIndexSet<PersistenceId, MAX_PERS_IDS>,
    pub remote_val: Option<(CanConfigId, RemoteConfig)>,
    p_idle_events: PIdleEvents,
    p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
}

impl CoreController {
    pub fn new(
        core_model: &mut CoreModel,
        p_idle_events: PIdleEvents,
        p_tx_frames: PTxFrames<MAX_TX_FRAMES>,
    ) -> Self {
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
        let av_supply_voltage = Pt1::new(
            12.0,
            CONTROLLER_TICK_RATE,
            core_model.config.av_supply_voltage_tc,
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
            polar: Polar::default(),
            drain_control: DrainControl::default(),
            flash_control: FlashControl::default(),
            speed_to_fly_control: SpeedToFlyControl::default(),
            gear_alarm_control: GearAlarmControl::default(),
            sound_control: SoundControl::default(),
            ms: 0,
            last_vario_mode: VarioMode::Vario,
            sw_update: SwUpdateController::new(),
            av2_climb_rate,
            av_speed_to_fly,
            av_supply_voltage,
            nmea_buffer: NmeaBuffer::new(),
            scheduler,
            nmea_vals: FnvIndexSet::new(),
            pers_vals: FnvIndexSet::new(),
            remote_val: None,
            p_idle_events,
            p_tx_frames,
        }
    }

    pub fn event_handler(&mut self, event: Event, cm: &mut CoreModel) {
        match event {
            Event::KeyItem(key_event) => self.key_action(cm, key_event),
            Event::DeviceItem(device_event) => self.device_action(cm, &device_event),
            Event::InputItem(input_event) => self.input_action(cm, input_event),
        }
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
                    }
                }
            }
        }
        recalc
    }

    pub fn set_ms(&mut self, time_ms: u16) {
        self.ms = time_ms;
    }

    pub fn recalc_glider(&mut self, cm: &mut CoreModel) {
        self.polar.recalc_glider(&cm.glider_data);
    }

    fn tick_100ms(&mut self, core_model: &mut CoreModel) {
        core_model.control.alive_ticks = core_model.control.alive_ticks.wrapping_add(1);

        if core_model.control.vario_mode == VarioMode::Vario {
            self.av2_climb_rate.tick(core_model.sensor.climb_rate);
            if core_model.control.avg_climb_slave_ticks == 0 {
                core_model.calculated.av2_climb_rate = self.av2_climb_rate.value();
            }
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

        let can_frame = core_model.can_frame_avg_climb_rates();
        let _ = self.p_tx_frames.enqueue(can_frame); // ignore when queue is full

        // calc moving average from supply voltage
        self.av_supply_voltage
            .tick(core_model.device.supply_voltage);
        core_model.calculated.av_supply_voltage = self.av_supply_voltage.value();

        // calc sound params
        if let Some(event) = self.sound_control.sound(core_model) {
            self.send_idle_event(event);
        }

        // create frame for external CAN loudspeaker
        let can_frame = core_model.can_frame_sound();
        let _ = self.p_tx_frames.enqueue(can_frame); // ignore when queue is full
    }

    pub fn send_idle_event(&mut self, idle_event: IdleEvent) {
        let _ = self.p_idle_events.enqueue(idle_event);
    }

    #[allow(unused)]
    fn set_output_pin(&mut self, pin: u8, pin_state: PinState) {
        let event = match pin {
            1 => IdleEvent::Output1(pin_state),
            2 => IdleEvent::Output2(pin_state),
            _ => return,
        };
        let _ = self.p_idle_events.enqueue(event);
    }

    // Event handler for reactions to inputs
    fn input_action(&mut self, cm: &mut CoreModel, input_event: InputPinState) {
        match input_event {
            InputPinState::Io1(state) => self.drain_control.set_state(cm, state),
            InputPinState::Io2(state) => self.speed_to_fly_control.set_state(state),
            InputPinState::Io3(state) => {
                let active = self.gear_alarm_control.set_gear_pin_state(cm, state);
                self.sound_control.set_scenario(sound::SoundScenario::GearAlarm, active);
            },
            InputPinState::Io4(state) => {
                let active = self.gear_alarm_control.set_airbrakes_pin_state(cm, state);
                self.sound_control.set_scenario(sound::SoundScenario::GearAlarm, active);
            },
        }
    }

    // Event handler for events generated by the device
    fn device_action(&mut self, core_model: &mut CoreModel, device_event: &DeviceEvent) {
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

    // Event handler for keystrokes
    fn key_action(&mut self, cm: &mut CoreModel, mut key_event: KeyEvent) {
        editor::key_action(&mut key_event, cm, self);
        menu::key_action(&mut key_event, cm, self);
    }
}
