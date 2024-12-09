#![allow(unused)]

use crate::{Editable, TString};

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
    pub fn name(&self) -> TString<16> {
        match self.content {
            MenuItemContent::EditItem(editable) => editable.name(),
            MenuItemContent::MenuItem() => {
                let menu = MENU_LIST[self.next_menu_idx];
                TString::<16>::from_str(menu.name)
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

pub const ROOT_IDX: usize = 0;
pub const FLIGHT_MENU_IDX: usize = 1;
pub const VARIO_SETINGS_IDX: usize = 2;

pub const MENU_LIST: &'static [Menu] = &[ROOT, FLIGHT_MENU, VARIO_SETTINGS];

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
            content: MenuItemContent::EditItem(Editable::McCready),
            next_menu_idx: ROOT_IDX,
        },
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
            content: MenuItemContent::EditItem(Editable::VarioModeControl),
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ROOT_IDX,
        },
    ],
};

pub const VARIO_SETTINGS: Menu = Menu {
    name: "Vario Settings",
    level: 1,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Display),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Glider),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcClimbRate),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcSpeedToFly),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ROOT_IDX,
        },
    ],
};
