use crate::{
    controller::KeyEvent, model::EditMode, CoreController, CoreModel, Editable
};
use heapless::String;
use core::str::FromStr;


pub type TString = String<16>;

pub struct F32Params {
    pub text: TString,
    pub min: f32,
    pub max: f32,
    pub small_inc: f32,
    pub big_inc: f32,
    pub dec_places: u8,
    pub unit: String<5>,
}

pub struct EnumParams {
    pub text: TString,
    pub variants: [TString; 10],
}

pub struct StringParams {
    pub text: TString,
}

pub enum Content {
    F32(f32, F32Params),
    Enum(TString, EnumParams),
    String(TString, StringParams),
}

struct Editor {
    target: Editable,
    mode: EditMode,
    content: Content,
}

fn get_content(cm: &CoreModel) -> Content {
    Content::F32 (
        cm.config.mc_cready.to_m_s(),
        F32Params {
            text: TString::from_str("Mac Cready").unwrap(),
            min: 0.0,
            max: 5.0,
            small_inc: 0.1,
            big_inc: 0.5,
            dec_places: 1,
            unit: String::<5>::from_str("m/s").unwrap(),
        }
    )
}


impl Editor {
    pub fn new() -> Self {
        Editor { 
            target: Editable::None,
            mode: EditMode::Off,
            content: Content::String(TString::new(), StringParams { text: TString::new()}),
        }
    }

    pub fn activate(&mut self, cm: &mut CoreModel, target: Editable) {
        if target != self.target {
            self.target = target;
            self.content = get_content(cm);

        }
    }
}


pub fn close_edit_frame(cm: &mut CoreModel, __cc: &mut CoreController) {
    // Close Editor if open
    if cm.control.edit_mode == EditMode::Section {
        cm.control.edit_mode = EditMode::Off;
        cm.control.softkeys.to_fallback();
    }
}

use libc_print::std_name::println;

pub fn key_action(
    cm: &mut CoreModel,
    __cc: &mut CoreController,
    key_event: &KeyEvent,
) {
    let target = cm.control.softkeys.key_action(*key_event);
    println!("target {:?}, current {:?}", target, cm.control.softkeys.current());
}