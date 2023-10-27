use crate::{
    CoreModel, core_model::EditMode,
};

const FRAME_RATE: u32 = 30;

impl CoreModel {
    pub fn activate_edit(&mut self, edit_var: Editable, edit_mode: EditMode, timeout: u32) {
        self.control.edit_mode = edit_mode;
        self.control.edit_var = edit_var;
        self.control.edit_ticks = timeout*FRAME_RATE;
    }
}

