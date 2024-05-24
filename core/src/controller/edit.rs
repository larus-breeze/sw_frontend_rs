use crate::{model::EditMode, CoreController, CoreModel};

pub fn close_edit_frame(cm: &mut CoreModel, __cc: &mut CoreController) {
    // Close Editor if open
    if cm.control.edit_mode == EditMode::Section {
        cm.control.edit_mode = EditMode::Off;
    }
}
