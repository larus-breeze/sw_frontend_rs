use crate::{
    model::EditMode, CoreController, CoreModel,
    controller::KeyEvent,
};

pub fn close_edit_frame(cm: &mut CoreModel, __cc: &mut CoreController) {
    // Close Editor if open
    if cm.control.edit_mode == EditMode::Section {
        cm.control.edit_mode = EditMode::Off;
        cm.control.softkeys.to_fallback();
    }
}

pub fn key_action(
    cm: &mut CoreModel,
    __cc: &mut CoreController,
    key_event: &KeyEvent,
) {
    cm.control.softkeys.key_action(*key_event);
}