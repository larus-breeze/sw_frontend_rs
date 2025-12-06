#![allow(unused)]
pub mod club;
pub mod full;

use crate::{CoreModel, Editable, TString};

#[derive(Clone, Copy, PartialEq)]
pub struct Menu {
    pub name: &'static str,
    pub level: usize,
    pub items: &'static [MenuItem],
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuItemContent {
    EditItem(Editable),
    MenuItem(),
}

#[derive(Clone, Copy, PartialEq)]
pub struct MenuItem {
    pub content: MenuItemContent,
    pub next_menu_idx: usize,
}

impl MenuItem {
    pub fn name(&self, cm: &CoreModel) -> &'static str {
        match self.content {
            MenuItemContent::EditItem(editable) => editable.name(),
            MenuItemContent::MenuItem() => {
                let menu = menu_list(cm, self.next_menu_idx);
                menu.name
            }
        }
    }

    pub fn is_menu(&self) -> bool {
        match self.content {
            MenuItemContent::EditItem(_) => false,
            MenuItemContent::MenuItem() => true,
        }
    }
}

pub fn menu_list(cm: &CoreModel, idx: usize) -> &'static Menu {
    if cm.config.club_mode {
        &club::MENU_LIST[idx]
    } else {
        &full::MENU_LIST[idx]
    }
}

pub fn settings_menu(cm: &CoreModel) -> &'static Menu {
    if cm.config.club_mode {
        &club::MENU_LIST[SETTINGS_IDX]
    } else {
        &full::MENU_LIST[SETTINGS_IDX]
    }
}

pub const ROOT_IDX: usize = 0;
pub const FLIGHT_MENU_IDX: usize = 1;
pub const SETTINGS_IDX: usize = 2;
pub const VIEW_SETTINGS_IDX: usize = 3;
pub const ADVANCED_SETTINGS_IDX: usize = 4;

pub const ROOT: Menu = Menu {
    name: "Root",
    level: 0,
    items: &[],
};

pub const FLIGHT_MENU: Menu = Menu {
    name: "Flight Menu",
    level: 1,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::WaterBallast),
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Bugs),
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PilotWeight),
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Display),
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UserProfile),
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ROOT_IDX,
        },
    ],
};
