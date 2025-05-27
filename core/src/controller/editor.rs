use crate::{
    basic_config::SECTION_EDITOR_TIMEOUT,
    controller::{helpers::IntToDuration, KeyEvent, Timer},
    model::{editable::*, DisplayActive, EditMode, OverlayActive},
    utils::TString,
    CoreController, CoreModel, Editable,
};
use num::clamp;

fn edit_enum_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &mut KeyEvent,
    target: Editable,
    params: &EnumParams,
) {
    if let Content::Enum(val) = cm.control.editor.content {
        let mut idx = MAX_ENUM_VARIANTS as isize;
        let mut max = 0_isize;
        while idx > 0 {
            idx -= 1;
            if max == 0 && !params.variants[idx as usize].is_empty() {
                max = idx;
            }
            if val.as_str() == params.variants[idx as usize] {
                break;
            }
        }
        match key_event {
            KeyEvent::Rotary2Left => idx -= 1,
            KeyEvent::Rotary2Right => idx += 1,
            KeyEvent::BtnEnc => (),
            _ => return,
        }
        let idx = clamp(idx, 0, max) as usize;
        let val = TString::<16>::from_str(params.variants[idx]);
        let content = Content::Enum(val);
        cm.control.editor.content = content;
        target.set_content(cm, cc, content);
        *key_event = KeyEvent::NoEvent
    }
}

fn edit_f32_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &mut KeyEvent,
    target: Editable,
    params: &F32Params,
) {
    if let Content::F32(opt_val) = cm.control.editor.content {
        if let Some(mut val) = opt_val {
            match key_event {
                KeyEvent::Rotary2Left => val -= params.small_inc,
                KeyEvent::Rotary2Right => val += params.small_inc,
                KeyEvent::Rotary1Left => val -= params.big_inc,
                KeyEvent::Rotary1Right => val += params.big_inc,
                KeyEvent::BtnEnc => (),
                _ => return,
            }
            let val = clamp(val, params.min, params.max);
            let content = Content::F32(Some(val));
            cm.control.editor.content = content;
            target.set_content(cm, cc, content);
        }
        *key_event = KeyEvent::NoEvent
    }
}

fn edit_list_content(
    cm: &mut CoreModel,
    cc: &mut CoreController,
    key_event: &mut KeyEvent,
    target: Editable,
    params: &ListParams,
) {
    if let Content::List(mut val) = cm.control.editor.content {
        match key_event {
            KeyEvent::Rotary2Left => val -= 1,
            KeyEvent::Rotary2Right => val += 1,
            KeyEvent::Rotary1Left => val -= 10,
            KeyEvent::Rotary1Right => val += 10,
            KeyEvent::BtnEnc => (),
            _ => return,
        }
        let val = clamp(val, 0, params.max);
        let content = Content::List(val);
        cm.control.editor.content = content;
        target.set_content(cm, cc, content);
        *key_event = KeyEvent::NoEvent
    }
}

fn edit_cmd_content(
    _cm: &mut CoreModel,
    _cc: &mut CoreController,
    key_event: &mut KeyEvent,
    _target: Editable,
    _params: &CmdParams,
) {
    // There is nothing to do here, Cmd is sent when activating
    *key_event = KeyEvent::NoEvent
}

pub fn key_action(key_event: &mut KeyEvent, cm: &mut CoreModel, cc: &mut CoreController) {
    if cm.control.editor.mode != EditMode::Off {
        match key_event {
            KeyEvent::BtnEnc => {
                cm.control.editor.enter_pushed = true;
                let _ = cc.scheduler.stop(Timer::CloseEditFrame, true); // finish edit session
            }
            KeyEvent::BtnEncS3 => *key_event = KeyEvent::NoEvent,
            _ => cc
                .scheduler
                .after(crate::Timer::CloseEditFrame, SECTION_EDITOR_TIMEOUT.secs()),
        }

        let target = cm.control.editor.target;

        // change volume and mc cready independently of the selection
        match target {
            Editable::Volume => {
                if *key_event == KeyEvent::Rotary1Left || *key_event == KeyEvent::Rotary1Right {
                    activate_editable(Editable::McCready, cm, cc);
                    *key_event = KeyEvent::NoEvent;
                }
            }
            Editable::McCready => {
                if *key_event == KeyEvent::Rotary2Left || *key_event == KeyEvent::Rotary2Right {
                    activate_editable(Editable::Volume, cm, cc);
                    *key_event = KeyEvent::NoEvent;
                }
            }
            _ => (),
        }

        match cm.control.editor.params {
            Params::Enum(params) => edit_enum_content(cm, cc, key_event, target, &params),
            Params::String(_) => (),
            Params::List(params) => edit_list_content(cm, cc, key_event, target, &params),
            Params::F32(params) => edit_f32_content(cm, cc, key_event, target, &params),
            Params::Cmd(params) => edit_cmd_content(cm, cc, key_event, target, &params),
        }
    }
    if cm.config.display_active != DisplayActive::Menu
        && cm.config.overlay_active != OverlayActive::Menu
    {
        match key_event {
            KeyEvent::Rotary1Left | KeyEvent::Rotary1Right => {
                activate_editable(Editable::McCready, cm, cc)
            }
            KeyEvent::Rotary2Left | KeyEvent::Rotary2Right => {
                activate_editable(Editable::Volume, cm, cc)
            }
            KeyEvent::Btn1 => activate_editable(Editable::McCready, cm, cc),
            KeyEvent::Btn2 => activate_editable(Editable::WaterBallast, cm, cc),
            KeyEvent::Btn3 => activate_editable(Editable::PilotWeight, cm, cc),
            KeyEvent::BtnEsc => activate_editable(Editable::VarioModeControl, cm, cc),
            _ => return,
        }
        *key_event = KeyEvent::NoEvent;
    }
}

pub fn activate_editable(editable: Editable, cm: &mut CoreModel, cc: &mut CoreController) {
    cm.control.editor.target = editable;
    cm.control.editor.params = editable.params();
    cm.control.editor.content = editable.content(cm, cc);
    cm.control.editor.enter_pushed = false;
    if cm.config.display_active == DisplayActive::Menu {
        cm.control.editor.mode = EditMode::Fullscreen
    } else {
        cm.control.editor.mode = cm.device_const.misc.edit_mode;
    }
    cm.config.overlay_active = OverlayActive::Editor;
    cc.scheduler
        .after(crate::Timer::CloseEditFrame, SECTION_EDITOR_TIMEOUT.secs());

    // a command is executed when activating it - not during edit session
    if let Params::Cmd(_content) = cm.control.editor.params {
        editable.set_content(cm, cc, cm.control.editor.content);
    }
}

pub fn close_edit_frame(cm: &mut CoreModel, _cc: &mut CoreController) {
    // Close Editor if open
    cm.control.editor.mode = EditMode::Off;
    cm.config.overlay_active = OverlayActive::None;
}

#[derive(Clone, Copy)]
pub struct Editor {
    pub target: Editable,
    pub mode: EditMode,
    pub params: Params,
    pub content: Content,
    pub enter_pushed: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            target: Editable::None,
            mode: EditMode::Off,
            params: Editable::None.params(),
            content: Content::String(TString::new()),
            enter_pushed: false,
        }
    }

    pub fn get_head_line(&self) -> &'static str {
        self.target.name()
    }

    pub fn get_value_line(&self) -> TString<20> {
        self.target.content_as_str(self.content)
    }
}
