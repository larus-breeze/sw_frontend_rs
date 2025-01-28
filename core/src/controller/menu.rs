use crate::{
    basic_config::MENU_TIMEOUT,
    controller::{editor, helpers::IntToDuration, EditMode, KeyEvent, Timer},
    model::{
        menu::{Menu, MenuItemContent, FLIGHT_MENU, MENU_LIST, ROOT, VARIO_SETTINGS},
        DisplayActive, OverlayActive,
    },
    CoreController, CoreModel, Editable,
};

#[derive(Clone, Copy)]
pub struct MenuControl {
    pub pos: [usize; 5],
    pub return_idx: usize,
    pub menu: &'static Menu,
}

impl Default for MenuControl {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuControl {
    pub fn new() -> Self {
        MenuControl {
            pos: [0; 5],
            return_idx: 0,
            menu: &FLIGHT_MENU,
        }
    }
}

pub fn key_action(key_event: &mut KeyEvent, cm: &mut CoreModel, cc: &mut CoreController) {
    if cm.control.editor.mode == EditMode::Window {
        return;
    }
    if cm.config.display_active == DisplayActive::Menu
        || cm.config.overlay_active == OverlayActive::Menu
    {
        cc.scheduler
            .after(crate::Timer::CloseMenu, MENU_TIMEOUT.secs());
        let level = cm.control.menu_control.menu.level;
        match key_event {
            KeyEvent::Rotary1Left | KeyEvent::Rotary2Left => {
                if cm.control.menu_control.pos[level] > 0 {
                    cm.control.menu_control.pos[level] -= 1;
                }
                *key_event = KeyEvent::NoEvent
            }
            KeyEvent::Rotary1Right | KeyEvent::Rotary2Right => {
                if (cm.control.menu_control.pos[level] + 1)
                    < cm.control.menu_control.menu.items.len()
                {
                    cm.control.menu_control.pos[level] += 1;
                }
                *key_event = KeyEvent::NoEvent
            }
            KeyEvent::BtnEnc => {
                // get next menu and level
                let pos = cm.control.menu_control.pos[level];
                let menu_item = cm.control.menu_control.menu.items[pos];
                let next_menu = &MENU_LIST[menu_item.next_menu_idx];

                // set menu pos to 0, if next level is higher than current
                if next_menu.level > level {
                    cm.control.menu_control.pos[next_menu.level] = 0;
                }

                // activate next_menu
                cm.control.menu_control.menu = next_menu;
                if next_menu == &ROOT {
                    close_menu_display(cm, cc);
                }

                // open editor if menu item contains something to edit
                if let MenuItemContent::EditItem(editable) = menu_item.content {
                    if editable != Editable::Return {
                        editor::activate_editable(editable, cm, cc);
                    }
                }
            }
            _ => (),
        };
    } else {
        match key_event {
            KeyEvent::BtnEnc => {
                activate_menu(&FLIGHT_MENU, cm, cc);
                *key_event = KeyEvent::NoEvent;
            }
            KeyEvent::BtnEncS3 => {
                activate_menu(&VARIO_SETTINGS, cm, cc);
                *key_event = KeyEvent::NoEvent;
            }
            _ => (),
        };
    }
}

pub fn activate_menu(menu: &'static Menu, cm: &mut CoreModel, cc: &mut CoreController) {
    cm.control.menu_control.pos[menu.level] = 0;
    cm.control.menu_control.menu = menu;

    if cm.config.display_active != DisplayActive::Menu {
        cm.config.last_display_active = cm.config.display_active;
    }

    if menu == &FLIGHT_MENU {
        cm.config.overlay_active = OverlayActive::Menu;
    } else {
        cm.config.display_active = DisplayActive::Menu;
    }
    cc.scheduler
        .after(crate::Timer::CloseMenu, MENU_TIMEOUT.secs());
}

pub fn close_menu_display(cm: &mut CoreModel, cc: &mut CoreController) {
    if cm.config.display_active == DisplayActive::Menu {
        cm.config.display_active = cm.config.last_display_active;
    }
    if cm.config.overlay_active == OverlayActive::Menu {
        cm.config.overlay_active = OverlayActive::None;
    }
    cm.control.menu_control.menu = &ROOT;
    let _ = cc.scheduler.stop(Timer::CloseMenu, false);
}
