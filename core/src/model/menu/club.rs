use crate::{
    model::menu::{
        Menu, MenuItem, MenuItemContent, ADVANCED_SETTINGS_IDX, FLIGHT_MENU, ROOT, ROOT_IDX,
        SETTINGS_IDX, VIEW_SETTINGS_IDX,
    },
    Editable,
};

pub const SPEED_TO_FLY_IDX: usize = 5;
pub const VARIO_IDX: usize = 6;
pub const UNITS_IDX: usize = 7;
pub const USAGE_MODE_AND_PROFILE_IDX: usize = 8;
pub const VIEW_STRAIGHT_IDX: usize = 9;
pub const VIEW_CIRCLING_IDX: usize = 10;
pub const SOUND_IDX: usize = 11;

pub const MENU_LIST: &[Menu] = &[
    ROOT,
    FLIGHT_MENU,
    SETTINGS,
    VIEW_SETTINGS,
    ADVANCED_SETTINGS,
    SPEED_TO_FLY,
    VARIO,
    UNITS,
    USAGE_MODE_AND_PROFILE,
    VIEW_STRAIGHT,
    VIEW_CIRCLING,
    SOUND,
];

pub const SETTINGS: Menu = Menu {
    name: "Settings",
    level: 1,
    items: &[
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ROOT_IDX,
        },
    ],
};

pub const VIEW_SETTINGS: Menu = Menu {
    name: "Views",
    level: 2,
    items: &[
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: VIEW_CIRCLING_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: VIEW_STRAIGHT_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: UNITS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::EnergyArrowMult),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::GliderSymbol),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: SETTINGS_IDX,
        },
    ],
};

pub const ADVANCED_SETTINGS: Menu = Menu {
    name: "Advanced",
    level: 2,
    items: &[
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: USAGE_MODE_AND_PROFILE_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SOUND_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: SETTINGS_IDX,
        },
    ],
};

pub const SPEED_TO_FLY: Menu = Menu {
    name: "Speed to Fly",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcCircleHysteresis),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcSpeedToFly),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioModeControl),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::StfUpperLimit),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::StfLowerLimit),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const VARIO: Menu = Menu {
    name: "Vario",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AvgClimbRateSrc),
            next_menu_idx: VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcClimbRate),
            next_menu_idx: VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioUpperLimit),
            next_menu_idx: VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioLowerLimit),
            next_menu_idx: VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const UNITS: Menu = Menu {
    name: "Units",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UnitHorizontalSpeed),
            next_menu_idx: UNITS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UnitVerticalSpeed),
            next_menu_idx: UNITS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UnitHeight),
            next_menu_idx: UNITS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
    ],
};

pub const USAGE_MODE_AND_PROFILE: Menu = Menu {
    name: "User Profiles",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UsageMode),
            next_menu_idx: USAGE_MODE_AND_PROFILE_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UsageCode),
            next_menu_idx: USAGE_MODE_AND_PROFILE_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const VIEW_STRAIGHT: Menu = Menu {
    name: "Straight",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterViewStraight),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info1Stf),
            next_menu_idx: VIEW_STRAIGHT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info2Stf),
            next_menu_idx: VIEW_STRAIGHT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info3Stf),
            next_menu_idx: VIEW_STRAIGHT_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
    ],
};

pub const VIEW_CIRCLING: Menu = Menu {
    name: "Circling",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterViewCircling),
            next_menu_idx: VIEW_CIRCLING_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info1Vario),
            next_menu_idx: VIEW_CIRCLING_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info2Vario),
            next_menu_idx: VIEW_CIRCLING_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info3Vario),
            next_menu_idx: VIEW_CIRCLING_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VIEW_SETTINGS_IDX,
        },
    ],
};

pub const SOUND: Menu = Menu {
    name: "Sound",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterFrequency),
            next_menu_idx: SOUND_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Waveform),
            next_menu_idx: SOUND_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::SoundSpreading),
            next_menu_idx: SOUND_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};
