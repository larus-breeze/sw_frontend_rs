mod demo;
use demo::DemoController;

mod vario;
use vario::VarioController;

mod sw_update;
use sw_update::SwUpdateController;

use crate::{
    basic_config::CONTROLLER_TICK_RATE,
    can_frame_sound,
    flight_physics::Polar,
    model::{DisplayActive, EditMode, VarioModeControl},
    system_of_units::FloatToSpeed,
    utils::{read_can_frame, KeyEvent},
    CoreModel, DeviceEvent, PersistenceId, PersistenceItem, VarioMode, POLARS,
};
use can_dispatch::CanFrame;

#[allow(unused_imports)]
use micromath::F32Ext;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Editable {
    ClimbRate,
    Glider,
    McCready,
    PilotWeight,
    VarioModeControl,
    Speed,
    Volume,
    WaterBallast,
    WindDirection,
    WindSpeed,
}

pub enum Direction {
    Forward,
    Backward,
}

pub enum Result {
    Nothing,
    Edit(EditMode, Editable, u32),
    NextDisplay(Direction),
}

pub struct CoreController {
    demo: DemoController,
    polar: Polar,
    vario: VarioController,
    sw_update: SwUpdateController,
    tick: u32,
}

impl CoreController {
    pub fn new(core_model: &mut CoreModel) -> Self {
        let polar_idx = core_model.config.glider_idx as usize;
        let polar = Polar::new(&POLARS[polar_idx], &mut core_model.glider_data);
        Self {
            demo: DemoController::new(),
            polar,
            vario: VarioController::new(),
            tick: 0,
            sw_update: SwUpdateController::new(),
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
        }
    }

    pub fn key_action(&mut self, core_model: &mut CoreModel, key_event: &KeyEvent) {
        if *key_event == KeyEvent::Btn1EscS3 {
            if core_model.control.demo_acitve {
                core_model.control.demo_acitve = false;
                return;
            } else {
                core_model.control.demo_acitve = true;
                return;
            }
        }
        let result = if core_model.control.demo_acitve {
            self.demo.key_action(core_model, key_event)
        } else {
            match core_model.config.display_active {
                DisplayActive::Vario => self.vario.key_action(core_model, key_event),
                DisplayActive::FirmwareUpdate => self.sw_update.key_action(core_model, key_event),
            }
        };

        // activate editor, if desired
        match result {
            Result::Edit(mode, var, timeout) => {
                core_model.control.edit_mode = mode;
                core_model.control.edit_var = var;
                core_model.control.edit_ticks = timeout * CONTROLLER_TICK_RATE;
                self.check_edit_results(core_model)
            }
            Result::Nothing => (),
            Result::NextDisplay(_) => (),
        }
    }

    pub fn time_action(&mut self, core_model: &mut CoreModel) {
        // Count edit_ticks down, to close editor if necessary
        if core_model.control.edit_ticks > 0 {
            core_model.control.edit_ticks -= 1;
            if core_model.control.edit_ticks == 0 {
                let p_item = match core_model.control.edit_var {
                    Editable::McCready => PersistenceItem::from_f32(
                        PersistenceId::McCready,
                        core_model.config.mc_cready.to_m_s(),
                    ),
                    Editable::Volume => {
                        PersistenceItem::from_i8(PersistenceId::Volume, core_model.config.volume)
                    }
                    Editable::WaterBallast => PersistenceItem::from_f32(
                        PersistenceId::WaterBallast,
                        core_model.glider_data.water_ballast.to_kg(),
                    ),
                    Editable::Glider => PersistenceItem::from_i32(
                        PersistenceId::Glider,
                        core_model.config.glider_idx,
                    ),
                    Editable::PilotWeight => PersistenceItem::from_f32(
                        PersistenceId::PilotWeight,
                        core_model.glider_data.pilot_weight.to_kg(),
                    ),
                    _ => PersistenceItem::do_not_store(),
                };
                core_model.send_idle_event(crate::IdleEvent::EepromItem(p_item));
            }
        }

        // Calculate speed_to_fly and speed_to_fly_dif
        let climb_rate = core_model.sensor.climb_rate;
        let mc_cready = core_model.config.mc_cready;
        let sink_rate = self.polar.sink_rate(core_model.sensor.airspeed);
        core_model.calculated.speed_to_fly =
            self.polar.speed_to_fly(climb_rate - sink_rate, mc_cready);
        core_model.calculated.speed_to_fly_dif =
            core_model.calculated.speed_to_fly.ias() - core_model.sensor.airspeed.ias();

        // calculate sound parameters and push can frame to queue
        let cms = &core_model.sensor;
        let cmc = &core_model.config;
        let (frequency, continuous, volume) = match core_model.control.vario_mode {
            VarioMode::Vario => (
                cmc.snd_center_freq * (cmc.snd_exp_mul * cms.climb_rate.to_m_s()).exp(),
                cms.climb_rate.to_m_s() < 0.0,
                cmc.volume,
            ),
            VarioMode::SpeedToFly => {
                let sped_to_fly_val = core_model.calculated.speed_to_fly_dif.to_km_h() / -10.0;
                if sped_to_fly_val.abs() < 1.0 {
                    (500.0, true, 0) // speed to fly is ok, so be quiet
                } else {
                    (
                        cmc.snd_center_freq * (cmc.snd_exp_mul * sped_to_fly_val).exp(),
                        sped_to_fly_val < 0.0,
                        cmc.volume,
                    )
                }
            }
        };

        // create CAN frame
        let can_frame = can_frame_sound(
            frequency as u16,
            volume as u8,
            cmc.snd_duty_cycle,
            continuous,
        );
        // add CAN frame to queue, ignore if the queue is full
        let _ = core_model.p_tx_frames.enqueue(can_frame);

        // The following actions are performed infrequently and alternately
        self.tick = (self.tick + 1) % CONTROLLER_TICK_RATE; // every second from beginning
        match self.tick {
            // Recalculate the polar coefficients based on the current data
            1 => self
                .polar
                .recalc(&core_model.glider_data, core_model.sensor.density),

            // Calculate the SpeedToFly/Vario limit and set the mode accordingly if necessary.
            2 => {
                let stf = self.polar.speed_to_fly(0.0.m_s(), 0.0.m_s());
                core_model.control.speed_to_fly_limit =
                    stf.ias() * core_model.control.vario_mode_switch_ratio;

                // In auto mode switch between Vario and SpeedToFly
                match core_model.control.vario_mode_control {
                    VarioModeControl::Auto => {
                        if core_model.sensor.airspeed.ias() > core_model.control.speed_to_fly_limit
                        {
                            core_model.control.vario_mode = VarioMode::SpeedToFly;
                        } else {
                            core_model.control.vario_mode = VarioMode::Vario;
                        }
                    }
                    VarioModeControl::SpeedToFly => {
                        core_model.control.vario_mode = VarioMode::SpeedToFly
                    }
                    VarioModeControl::Vario => core_model.control.vario_mode = VarioMode::Vario,
                }

                // Set 1-second-speed-to-fly value
                core_model.calculated.speed_to_fly_1s = core_model.calculated.speed_to_fly.ias();
            }
            _ => (),
        }
    }

    /// Interprets a Can Frame and stores the results in the CoreModel
    pub fn read_can_frame(&self, core_model: &mut CoreModel, frame: &CanFrame) {
        read_can_frame(core_model, frame)
    }

    /// Executes instructions based on the user's input
    fn check_edit_results(&mut self, core_model: &mut CoreModel) {
        #[allow(clippy::single_match)]
        match core_model.control.edit_var {
            Editable::Glider => {
                let polar_idx = core_model.config.glider_idx as usize;
                self.polar = Polar::new(&POLARS[polar_idx], &mut core_model.glider_data)
            }
            _ => (),
        }
    }
}
