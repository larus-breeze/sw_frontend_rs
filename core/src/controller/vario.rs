use crate::{
    basic_config::SECTION_EDITOR_TIMEOUT,
    controller::{Direction, Editable, Result},
    flight_physics::POLAR_COUNT,
    model::{CoreModel, EditMode, VarioModeControl},
    system_of_units::{FloatToMass, FloatToSpeed},
    utils::{val_manip, KeyEvent},
    SysConfigId,
};
use num::clamp;

pub struct VarioController {
    edit_var: Editable,
}

impl VarioController {
    pub fn new() -> Self {
        VarioController {
            edit_var: Editable::Volume,
        }
    }

    pub fn key_action(&mut self, cm: &mut CoreModel, key_event: &KeyEvent) -> Result {
        if cm.control.edit_ticks == 0 {
            self.edit_var = Editable::Volume;
        }

        let act_edit = self.edit_var;
        self.edit_var = if cm.control.edit_ticks == 0 || act_edit == Editable::Volume {
            // Activate Edit Mode
            match key_event {
                KeyEvent::Btn1 => Editable::McCready,
                KeyEvent::Btn2 => Editable::WaterBallast,
                KeyEvent::Btn3 => Editable::PilotWeight,
                KeyEvent::BtnEsc => Editable::VarioModeControl,
                KeyEvent::Btn1S3 => Editable::Glider,
                _ => act_edit,
            }
        } else {
            match key_event {
                KeyEvent::Btn1 => match act_edit {
                    Editable::McCready => Editable::VarioModeControl,
                    Editable::WaterBallast => Editable::McCready,
                    Editable::PilotWeight => Editable::WaterBallast,
                    Editable::VarioModeControl => Editable::PilotWeight,
                    _ => act_edit,
                },
                KeyEvent::Btn2 => match act_edit {
                    Editable::McCready => Editable::WaterBallast,
                    Editable::WaterBallast => Editable::PilotWeight,
                    Editable::PilotWeight => Editable::VarioModeControl,
                    Editable::VarioModeControl => Editable::McCready,
                    _ => act_edit,
                },
                _ => act_edit,
            }
        };

        match self.edit_var {
            Editable::McCready => {
                cm.config.mc_cready =
                    val_manip(cm.config.mc_cready.to_m_s(), key_event, 0.1, 0.5, 0.0, 5.0).m_s();
                if let Some(frame) = cm.can_frame_sys_config(SysConfigId::MacCready) {
                    let _ = cm.p_tx_frames.enqueue(frame);
                }
            }
            Editable::Volume => {
                cm.config.volume = match key_event {
                    KeyEvent::Rotary1Left => return Result::NextDisplay(Direction::Backward),
                    KeyEvent::Rotary1Right => return Result::NextDisplay(Direction::Forward),
                    KeyEvent::Rotary2Left => clamp(cm.config.volume - 1, 0, 50),
                    KeyEvent::Rotary2Right => clamp(cm.config.volume + 1, 0, 50),
                    _ => return Result::Nothing,
                };
                if let Some(frame) = cm.can_frame_sys_config(SysConfigId::VolumeVario) {
                    let _ = cm.p_tx_frames.enqueue(frame);
                }
            }
            Editable::WaterBallast => {
                cm.glider_data.water_ballast = val_manip(
                    cm.glider_data.water_ballast.to_kg(),
                    key_event,
                    1.0,
                    10.0,
                    0.0,
                    250.0,
                )
                .kg();
                if let Some(frame) = cm.can_frame_sys_config(SysConfigId::WaterBallast) {
                    let _ = cm.p_tx_frames.enqueue(frame);
                }
            }
            Editable::Glider => {
                cm.config.glider_idx = val_manip(
                    cm.config.glider_idx,
                    key_event,
                    1,
                    20,
                    0,
                    POLAR_COUNT as i32 - 1,
                )
            }
            Editable::PilotWeight => {
                cm.glider_data.pilot_weight = val_manip(
                    cm.glider_data.pilot_weight.to_kg(),
                    key_event,
                    1.0,
                    10.0,
                    0.0,
                    250.0,
                )
                .kg();
                if let Some(frame) = cm.can_frame_sys_config(SysConfigId::PilotWeight) {
                    let _ = cm.p_tx_frames.enqueue(frame);
                }
            }
            Editable::VarioModeControl => {
                cm.control.vario_mode_control = match key_event {
                    KeyEvent::Rotary2Left => match cm.control.vario_mode_control {
                        VarioModeControl::Auto => VarioModeControl::SpeedToFly,
                        VarioModeControl::SpeedToFly => VarioModeControl::Vario,
                        VarioModeControl::Vario => VarioModeControl::Auto,
                    },
                    KeyEvent::Rotary2Right => match cm.control.vario_mode_control {
                        VarioModeControl::Auto => VarioModeControl::Vario,
                        VarioModeControl::SpeedToFly => VarioModeControl::Auto,
                        VarioModeControl::Vario => VarioModeControl::SpeedToFly,
                    },
                    _ => cm.control.vario_mode_control,
                };
                if let Some(frame) = cm.can_frame_sys_config(SysConfigId::VarioModeControl) {
                    let _ = cm.p_tx_frames.enqueue(frame);
                }
            }
            _ => (),
        }
        Result::Edit(EditMode::Section, self.edit_var, SECTION_EDITOR_TIMEOUT)
    }
}
