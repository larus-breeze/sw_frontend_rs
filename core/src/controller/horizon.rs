use crate::{
    model::{CoreModel, DisplayActive},
    Editable,
};

pub fn set_horizon_active(cm: &mut CoreModel) {
    cm.control.softkeys.set_editables(
        Editable::McCready,
        Editable::WaterBallast,
        Editable::PilotWeight,
        Editable::VarioModeControl,
    );
    cm.control
        .softkeys
        .set_3s_keys(Editable::Glider, Editable::Theme);
    cm.config.display_active = DisplayActive::Horizon;
}
