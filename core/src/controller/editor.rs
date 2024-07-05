use crate::{
    basic_config::SECTION_EDITOR_TIMEOUT,
    controller::{helpers::IntToDuration, KeyEvent, Timer},
    model::{editable::*, EditMode},
    utils::TString,
    CoreController, CoreModel, Editable, Polar, POLARS,
};
use num::clamp;

use tfmt::Convert;

fn edit_enum_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &KeyEvent,
    target: Editable,
    params: &EnumParams,
) {
    if let Content::Enum(val) = cm.control.editor.content {
        let mut idx = MAX_ENUM_VARIANTS as isize;
        let mut max = 0_isize;
        while idx > 0 {
            idx -= 1;
            if max == 0 && params.variants[idx as usize].len() > 0 {
                max = idx;
            }
            if val == params.variants[idx as usize] {
                break;
            }
        }
        match key_event {
            KeyEvent::Rotary2Left => idx -= 1,
            KeyEvent::Rotary2Right => idx += 1,
            _ => (),
        }
        let idx = clamp(idx, 0, max) as usize;
        let val = params.variants[idx];
        set_enum_content(cm, cc, &val, target);
        cm.control.editor.content = Content::Enum(val);
    }
}

fn edit_f32_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &KeyEvent,
    target: Editable,
    params: &F32Params,
) {
    if let Content::F32(mut val) = cm.control.editor.content {
        match key_event {
            KeyEvent::Rotary2Left => val -= params.small_inc,
            KeyEvent::Rotary2Right => val += params.small_inc,
            KeyEvent::Rotary1Left => val -= params.big_inc,
            KeyEvent::Rotary1Right => val += params.big_inc,
            _ => (),
        }
        let val = clamp(val, params.min, params.max);
        set_f32_content(cm, cc, val, target);
        cm.control.editor.content = Content::F32(val);
    }
}

fn edit_polar_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &KeyEvent,
    target: Editable,
    params: &PolarParams,
) {
    if let Content::Polar(mut val) = cm.control.editor.content {
        match key_event {
            KeyEvent::Rotary2Left => val -= 1,
            KeyEvent::Rotary2Right => val += 1,
            KeyEvent::Rotary1Left => val -= 10,
            KeyEvent::Rotary1Right => val += 10,
            _ => (),
        }
        let val = clamp(val, 0, params.max);
        set_polar_content(cm, cc, val, target);
        cc.polar = Polar::new(&POLARS[val as usize], &mut cm.glider_data);
        cm.control.editor.content = Content::Polar(val);
    }
}

pub fn key_action(
    key_event: &KeyEvent,
    target_changed: bool,
    cm: &mut CoreModel,
    cc: &mut CoreController,
) {
    let target = cm.control.softkeys.current();
    cm.control.editor.target = target;
    if target_changed {
        get_params(cm, target);
        get_content(cm, target);
        cm.control.editor.mode = EditMode::Section;
    }

    if *key_event == KeyEvent::BtnEnc {
        let _ = cc.scheduler.stop(Timer::CloseEditFrame, true); // finish edit session
    } else {
        cc.scheduler
            .after(crate::Timer::CloseEditFrame, SECTION_EDITOR_TIMEOUT.secs());
    }

    match cm.control.editor.params {
        Params::Enum(params) => edit_enum_content(cm, cc, key_event, target, &params),
        Params::String(_) => (),
        Params::Polar(params) => edit_polar_content(cm, cc, key_event, target, &params),
        Params::F32(params) => edit_f32_content(cm, cc, key_event, target, &params),
    }
}

pub fn close_edit_frame(cm: &mut CoreModel, _cc: &mut CoreController) {
    // Close Editor if open
    cm.control.editor.mode = EditMode::Off;
    cm.control.softkeys.to_fallback();
}

#[derive(Clone, Copy)]
pub struct Editor {
    pub target: Editable,
    pub mode: EditMode,
    pub params: Params,
    pub content: Content,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            target: Editable::None,
            mode: EditMode::Off,
            params: Params::String(StringParams {
                text: TString::new(),
            }),
            content: Content::String(TString::new()),
        }
    }

    pub fn get_head_line(&self) -> TString<16> {
        match self.params {
            Params::Enum(params) => params.text,
            Params::F32(params) => params.text,
            Params::Polar(params) => params.text,
            Params::String(params) => params.text,
        }
    }

    pub fn get_value_line(&self) -> Convert<20> {
        let mut conv = Convert::<20>::new(b' ');
        match self.params {
            Params::Enum(_params) => {
                if let Content::Enum(val) = self.content {
                    conv.write_str(val.as_str()).unwrap();
                }
            }
            Params::F32(params) => {
                if let Content::F32(val) = self.content {
                    conv.write_str(params.unit.as_str()).unwrap();
                    conv.write_u8(b' ').unwrap();
                    conv.f32(val, params.dec_places as usize).unwrap();
                }
            }
            Params::Polar(_params) => {
                if let Content::Polar(val) = self.content {
                    conv.write_str(POLARS[val as usize].name).unwrap();
                }
            }
            Params::String(_params) => {
                if let Content::String(val) = self.content {
                    conv.write_str(val.as_str()).unwrap();
                }
            }
        }
        conv
    }
}
