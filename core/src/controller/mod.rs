mod demo;
use demo::DemoController;

mod vario;
use vario::VarioController;

mod sw_update;
use sw_update::SwUpdateController;

use crate::{
    basic_config::CONTROLLER_TICK_RATE,
    can_frame_heartbeat, can_frame_sound,
    flight_physics::Polar,
    model::{DisplayActive, EditMode, TcrMode, VarioModeControl},
    system_of_units::{FloatToSpeed, Speed},
    utils::{read_can_frame, KeyEvent, Pt1},
    CoreModel, DeviceEvent, FlyMode, IdleEvent, PersistenceId, VarioMode, POLARS,
    Frame,
};

#[allow(unused_imports)]
use micromath::F32Ext;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
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
    last_vario_mode: VarioMode,
    av2_climb_rate: Pt1<Speed>,
    av_speed_to_fly: Pt1<Speed>,
}

impl CoreController {
    pub fn new(core_model: &mut CoreModel) -> Self {
        let polar_idx = core_model.config.glider_idx as usize;
        let polar = Polar::new(&POLARS[polar_idx], &mut core_model.glider_data);
        let av2_climb_rate = Pt1::new(0.0.m_s(), CONTROLLER_TICK_RATE, core_model.config.av2_climb_rate_tc);
        let av_speed_to_fly = Pt1::new(0.0.m_s(), CONTROLLER_TICK_RATE, core_model.config.av_speed_to_fly_tc);
        Self {
            demo: DemoController::new(),
            polar,
            vario: VarioController::new(),
            tick: 0,
            last_vario_mode: VarioMode::Vario,
            sw_update: SwUpdateController::new(),
            av2_climb_rate,
            av_speed_to_fly,
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

        if *key_event == KeyEvent::BtnEnc && core_model.control.edit_ticks > 1 {
            core_model.control.edit_ticks = 1; // finish edit session
        } else {
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

    }

    pub fn time_action(&mut self, core_model: &mut CoreModel) {
        core_model.pers_tick();
        if core_model.control.vario_mode == VarioMode::Vario {
            self.av2_climb_rate.tick(core_model.sensor.climb_rate);
            core_model.calculated.av2_climb_rate = self.av2_climb_rate.value();
        }

        // Count edit_ticks down, to close editor if necessary
        if core_model.control.edit_ticks > 0 {
            core_model.control.edit_ticks -= 1;
            if core_model.control.edit_ticks == 0 {
                let pers_id = match core_model.control.edit_var {
                    Editable::McCready => PersistenceId::McCready,
                    Editable::Volume => PersistenceId::Volume,
                    Editable::WaterBallast => PersistenceId::WaterBallast,
                    Editable::Glider => PersistenceId::Glider,
                    Editable::PilotWeight => PersistenceId::PilotWeight,
                    Editable::VarioModeControl => PersistenceId::VarioModeControl,
                    _ => PersistenceId::DoNotStore,
                };
                core_model.store_persistence_id(pers_id);
            }
        }

        // Calculate speed_to_fly and speed_to_fly_dif
        let climb_rate = core_model.sensor.climb_rate;
        let mc_cready = core_model.config.mc_cready;
        let sink_rate = self.polar.sink_rate(core_model.sensor.airspeed);
        core_model.calculated.speed_to_fly =
            self.polar.speed_to_fly(climb_rate - sink_rate, mc_cready);
        self.av_speed_to_fly.tick(core_model.calculated.speed_to_fly.ias());
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
        core_model.calculated.frequency = frequency;
        core_model.calculated.continuous = continuous;
        if gain != core_model.calculated.gain {
            core_model.calculated.gain = gain;
            let event = IdleEvent::SetGain(gain as u8);

            // send event to the idle loop, which handles the amplifier via i2c
            core_model.send_idle_event(event);
        }

        // create CAN frame
        let can_frame = can_frame_sound(
            frequency,
            gain as u8,
            core_model.config.snd_duty_cycle,
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
                core_model.control.vario_mode = match core_model.control.vario_mode_control {
                    VarioModeControl::Auto => {
                        if core_model.sensor.airspeed.ias() > core_model.control.speed_to_fly_limit
                        {
                            VarioMode::SpeedToFly
                        } else {
                            VarioMode::Vario
                        }
                    }
                    VarioModeControl::SpeedToFly => VarioMode::SpeedToFly,
                    VarioModeControl::Vario => VarioMode::Vario,
                };

                // Set 1-second-speed-to-fly value
                core_model.calculated.speed_to_fly_1s = core_model.calculated.av_speed_to_fly;

                if self.last_vario_mode != core_model.control.vario_mode {
                    self.last_vario_mode = core_model.control.vario_mode;
                    if core_model.control.vario_mode == VarioMode::Vario {
                        // Set average climbrate to current climbrate
                        self.av2_climb_rate.set_value(core_model.sensor.climb_rate);
                    }
                }
                match core_model.control.fly_mode {
                    FlyMode::Circling => {
                        // Start measuring thermal climb rate
                        match core_model.control.tcr_mode {
                            TcrMode::StraightFlight => {
                                core_model.control.tcr_start = core_model.sensor.gps_altitude;
                                core_model.control.tcr_1s_climb_ticks = 1;
                            }
                            TcrMode::Transition => {
                                core_model.control.tcr_1s_transient_ticks = 0;
                                core_model.control.tcr_1s_climb_ticks += 1;
                            }
                            TcrMode::Climbing => {
                                core_model.control.tcr_1s_climb_ticks += 1;
                            }
                        }
                        core_model.control.tcr_mode = TcrMode::Climbing;
                        // Calculate thermal climb rate
                        let tcr = {
                            let diff_h = (core_model.sensor.gps_altitude
                                - core_model.control.tcr_start)
                                .to_m();
                            (diff_h / core_model.control.tcr_1s_climb_ticks as f32).m_s()
                        };
                        core_model.calculated.thermal_climb_rate = tcr;
                    }
                    FlyMode::StraightFlight => {
                        match core_model.control.tcr_mode {
                            TcrMode::Climbing => {
                                core_model.control.tcr_mode = TcrMode::Transition;
                                core_model.control.tcr_1s_transient_ticks = 0;
                            }
                            TcrMode::Transition => {
                                core_model.control.tcr_1s_transient_ticks += 1;
                                if core_model.control.tcr_1s_transient_ticks > 30 {
                                    core_model.control.tcr_mode = TcrMode::StraightFlight;
                                    core_model.calculated.thermal_climb_rate = 0.0.m_s();
                                }
                            }
                            TcrMode::StraightFlight => {
                                core_model.control.tcr_start = core_model.sensor.gps_altitude
                            }
                        }
                    }
                }
            }
            3 => {
                // create CAN frame and add to queue
                let can_frame = can_frame_heartbeat(core_model.config.uuid);
                let _ = core_model.p_tx_frames.enqueue(can_frame);
            }
            _ => (),
        }
    }

    /// Interprets a Can Frame and stores the results in the CoreModel
    pub fn read_can_frame(&self, core_model: &mut CoreModel, frame: &Frame) {
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
