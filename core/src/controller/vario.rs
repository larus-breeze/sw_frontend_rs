use crate::{
    controller::{ControlResult, Direction, Echo, Editable},
    flight_physics::POLAR_COUNT,
    model::{CoreModel, EditMode, VarioModeControl},
    system_of_units::{FloatToMass, FloatToSpeed},
    utils::{val_manip, KeyEvent},
    CoreController,
};
use num::clamp;

pub fn key_action(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &KeyEvent,
) -> ControlResult {
    if cm.control.edit_mode == EditMode::Off {
        cc.edit_var = Editable::Volume;
    }

    let act_edit = cc.edit_var;
    cc.edit_var = if act_edit == Editable::Volume {
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

    match cc.edit_var {
        Editable::McCready => cc.persist_set_maccready(
            cm,
            val_manip(cm.config.mc_cready.to_m_s(), key_event, 0.1, 0.5, 0.0, 5.0).m_s(),
            Echo::NmeaAndCan,
        ),
        Editable::Volume => cc.persist_set_volume(
            cm,
            match key_event {
                KeyEvent::Rotary1Left => return ControlResult::NextDisplay(Direction::Backward),
                KeyEvent::Rotary1Right => return ControlResult::NextDisplay(Direction::Forward),
                KeyEvent::Rotary2Left => clamp(cm.config.volume - 1, 0, 50),
                KeyEvent::Rotary2Right => clamp(cm.config.volume + 1, 0, 50),
                _ => return ControlResult::Nothing,
            },
            Echo::NmeaAndCan,
        ),
        Editable::WaterBallast => cc.persist_set_water_ballast(
            cm,
            val_manip(
                cm.glider_data.water_ballast.to_kg(),
                key_event,
                1.0,
                10.0,
                0.0,
                250.0,
            )
            .kg(),
            Echo::NmeaAndCan,
        ),
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
        Editable::PilotWeight => cc.persist_set_pilot_weight(
            cm,
            val_manip(
                cm.glider_data.pilot_weight.to_kg(),
                key_event,
                1.0,
                10.0,
                0.0,
                250.0,
            )
            .kg(),
            Echo::NmeaAndCan,
        ),
        Editable::VarioModeControl => cc.persist_set_vario_mode_control(
            cm,
            match key_event {
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
            },
            Echo::NmeaAndCan,
        ),
        _ => (),
    }
    ControlResult::Edit(EditMode::Section, cc.edit_var)
}
