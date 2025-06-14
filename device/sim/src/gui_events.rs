
use slint::ComponentHandle;

use crate::{Adapt, AppWindow, Global, LogSettings};


pub struct GuiEvents {}

impl GuiEvents {
    pub fn new(adapt_: &Adapt, ui: &AppWindow) {
        let adapt = adapt_.clone();
        ui.on_key_pressed(move |event| {
            adapt.process_cmd(event.text.as_str());
        });
    
        let adapt = adapt_.clone();
        ui.global::<Global>().on_process_command(move |cmd| {
            adapt.process_cmd(cmd.as_str());
        });

        let adapt = adapt_.clone();
        ui.on_sec_tick(move || {
            adapt.sec_tick();
        });

        let adapt = adapt_.clone();
        ui.global::<LogSettings>().on_filter_constrain(move |src, text| {
            adapt.set_filter_constrain(&src, &text);
        })
    }
}
