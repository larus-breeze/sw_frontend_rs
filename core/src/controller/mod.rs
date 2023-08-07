mod demo;
use demo::DemoController;

mod vario;
use vario::VarioController;

use crate::{
    flight_physics::Polar, 
    model::EditMode, 
    utils::{KeyEvent, read_can_frame}, 
    CoreModel, 
    POLARS
};
use embedded_can::Frame;

const UPDATE_RATE: u32 = 10;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Editable {
    ClimbRate,
    Glider,
    McCready,
    PilotWeight,
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
            self.vario.key_action(core_model, key_event)
        };

        // activate editor, if desired
        match result {
            Result::Edit(mode, var, timeout) => {
                core_model.control.edit_mode = mode;
                core_model.control.edit_var = var;
                core_model.control.edit_ticks = timeout * UPDATE_RATE;
                self.check_edit_results(core_model)
            }
            Result::Nothing => (),
            Result::NextDisplay(_) => (),
        }
    }

    pub fn time_action(&mut self, core_model: &mut CoreModel) {
        if core_model.control.edit_ticks > 0 {
            core_model.control.edit_ticks -= 1;
        }
        let climb_rate = core_model.sensor.climb_rate;
        let mc_cready = core_model.calculated.mc_cready;
        let sink_rate = self.polar.sink_rate(core_model.sensor.airspeed);
        core_model.calculated.speed_to_fly =
            self.polar.speed_to_fly(climb_rate - sink_rate, mc_cready);
        core_model.calculated.speed_to_fly_dif =
            core_model.calculated.speed_to_fly.ias() - core_model.sensor.airspeed.ias();

        // The following actions are performed infrequently and alternately
        self.tick = (self.tick + 1) % 10; // 10 Hz -> every second from beginning
        match self.tick {
            1 => self
                .polar
                .recalc(&core_model.glider_data, core_model.sensor.density),
            _ => (),
        }
    }

    pub fn read_can_frame<F: Frame>(&self, core_model: &mut CoreModel, frame: &F) {
        read_can_frame(core_model, frame)
    }

    fn check_edit_results(&mut self, core_model: &mut CoreModel) {
        match core_model.control.edit_var {
            Editable::Glider => {
                let polar_idx = core_model.config.glider_idx as usize;
                self.polar = Polar::new(&POLARS[polar_idx], &mut core_model.glider_data)
            }
            _ => (),
        }
    }
}
