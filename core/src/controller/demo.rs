use crate::model::EditMode;
use crate::utils::val_manip;
use crate::CoreModel;
use crate::{
    controller::Result,
    flight_physics::AirSpeed,
    system_of_units::{AngleUnit, FloatToSpeed},
    utils::KeyEvent,
};

use super::Editable;
use crate::model::{FlyMode, VarioMode};

/// The Demo Mode allows to preset data normally measured by the Larus sensor by keys and rotary
/// encoders. This function allows on the one hand to get familiar with the displays and on the
/// other hand to check functions like the speed command or polar curve of the glider.
#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum DemoKind {
    Vario = 1,
    Speed,
    WindDirection,
    WindSpeed,
    Last,
}

/// Switch through demo variables
impl DemoKind {
    pub fn inc(&mut self) {
        let val = *self as u8 + 1;
        self.fit_to_range(val)
    }
    pub fn dec(&mut self) {
        let val = *self as u8 - 1;
        self.fit_to_range(val)
    }

    fn fit_to_range(&mut self, val: u8) {
        let mut val = val;
        if val >= DemoKind::Last as u8 {
            val = 1
        }
        if val == 0 {
            val = DemoKind::Last as u8 - 1
        }
        // Safety: we have checked, that val is in range, so unsafe is ok here
        *self = unsafe { core::mem::transmute::<u8, DemoKind>(val) };
    }
}

/// Control component for demo mode
pub struct DemoController {
    demo_kind: DemoKind,
}

impl DemoController {
    pub fn new() -> Self {
        DemoController {
            demo_kind: DemoKind::Vario,
        }
    }

    pub fn key_action(&mut self, cm: &mut CoreModel, key_event: &KeyEvent) -> Result {
        match key_event {
            KeyEvent::Btn1 => self.demo_kind.inc(),
            KeyEvent::Btn2 => self.demo_kind.dec(),
            KeyEvent::Btn3 => match cm.control.vario_mode {
                VarioMode::Vario => {
                    cm.control.vario_mode = VarioMode::SpeedToFly;
                    cm.control.fly_mode = FlyMode::StraightFlight;
                }
                VarioMode::SpeedToFly => {
                    cm.control.vario_mode = VarioMode::Vario;
                    cm.control.fly_mode = FlyMode::Circling;
                }
            },
            _ => (),
        }

        match self.demo_kind {
            DemoKind::Vario => {
                cm.sensor.climb_rate = val_manip(
                    cm.sensor.climb_rate.to_m_s(),
                    key_event,
                    0.1,
                    0.5,
                    -5.0,
                    5.0,
                )
                .m_s()
            }
            DemoKind::Speed => {
                let tas = val_manip(
                    cm.sensor.airspeed.tas().to_km_h(),
                    key_event,
                    1.0,
                    10.0,
                    0.0,
                    270.0,
                )
                .km_h();
                cm.sensor.airspeed = AirSpeed::from_tas_at_nn(tas);
            }
            DemoKind::WindDirection => {
                cm.sensor.wind_angle = val_manip(
                    cm.sensor.wind_angle.to_degrees(),
                    key_event,
                    5.0,
                    30.0,
                    0.0,
                    359.0,
                )
                .deg()
            }
            DemoKind::WindSpeed => {
                cm.sensor.wind_speed = val_manip(
                    cm.sensor.wind_speed.to_km_h(),
                    key_event,
                    1.0,
                    5.0,
                    0.0,
                    99.0,
                )
                .km_h()
            }
            _ => (), // should never happen
        }
        if !((*key_event == KeyEvent::NoEvent) || (*key_event == KeyEvent::Btn3)) {
            let edit_var = match self.demo_kind {
                DemoKind::Vario => Editable::ClimbRate,
                DemoKind::Speed => Editable::Speed,
                DemoKind::WindSpeed => Editable::WindSpeed,
                DemoKind::WindDirection => Editable::WindDirection,
                _ => Editable::ClimbRate,
            };
            Result::Edit(EditMode::Section, edit_var, 2)
        } else {
            Result::Nothing
        }
    }
}
