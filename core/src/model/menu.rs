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
    pub fn name(&self) -> &'static str {
        match self.content {
            MenuItemContent::EditItem(editable) => editable.name(),
            MenuItemContent::MenuItem() => {
                let menu = MENU_LIST[self.next_menu_idx];
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

pub const ROOT_IDX: usize = 0;
pub const FLIGHT_MENU_IDX: usize = 1;
pub const VARIO_SETINGS_IDX: usize = 2;
pub const VIEW_SETINGS_IDX: usize = 3;
pub const ADVANCED_SETTINGS_IDX: usize = 4;
pub const DRAIN_SETTINGS_IDX: usize = 5;
pub const POLAR_SETTINGS_IDX: usize = 6;
pub const SENSOR_BOX_COMMANDS_IDX: usize = 7;
pub const SENSOR_BOX_SETTINGS_IDX: usize = 8;
pub const SPEED_TO_FLY_IDX: usize = 9;
pub const LANDING_GEAR_ALARM_IDX: usize = 10;
pub const AVERAGE_CLIMB_RATE_IDX: usize = 11;
pub const RESET_CONFIG_IDX: usize = 12;

pub const MENU_LIST: &[Menu] = &[
    ROOT,
    FLIGHT_MENU,
    VARIO_SETTINGS,
    VIEW_SETTINGS,
    ADVANCED_SETTINGS,
    DRAIN_SETTINGS,
    POLAR_SETTINGS,
    SENSOR_BOX_COMMANDS,
    SENSOR_BOX_SETTINGS,
    SPEED_TO_FLY,
    LANDING_GEAR_ALARM,
    AVERAGE_CLIMB_RATE,
    RESET_CONFIG,
];

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
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ROOT_IDX,
        },
    ],
};

pub const VARIO_SETTINGS: Menu = Menu {
    name: "Settings",
    level: 1,
    items: &[
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
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
            content: MenuItemContent::EditItem(Editable::Info1),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info2),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Info3),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterViewCircling),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterViewStraight),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::EnergyArrowMult),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Rotation),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::GliderSymbol),
            next_menu_idx: VIEW_SETINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
    ],
};

pub const ADVANCED_SETTINGS: Menu = Menu {
    name: "Advanced",
    level: 2,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::UserProfile),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterFrequency),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::BatteryGood),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::BatteryLow),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::FlashControl),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: RESET_CONFIG_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: AVERAGE_CLIMB_RATE_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: LANDING_GEAR_ALARM_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: DRAIN_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
    ],
};

pub const DRAIN_SETTINGS: Menu = Menu {
    name: "Drain Control",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::DrainPinConfig),
            next_menu_idx: DRAIN_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::FlowEmpty),
            next_menu_idx: DRAIN_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::FlowSlope),
            next_menu_idx: DRAIN_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const POLAR_SETTINGS: Menu = Menu {
    name: "Polar Settings",
    level: 2,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Glider),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::EmptyMass),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::MaxBallast),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::ReferenceWeight),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PolarValueV1),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PolarValueV2),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PolarValueV3),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PolarValueSi1),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PolarValueSi2),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PolarValueSi3),
            next_menu_idx: POLAR_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
    ],
};

pub const SENSOR_BOX_COMMANDS: Menu = Menu {
    name: "Sensorbox",
    level: 2,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CmdMeas1),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CmdMeas2),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CmdMeas3),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CmdCalcOrientation),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CmdFineTuneOrientation),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CmdResetSensorbox),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: VARIO_SETINGS_IDX,
        },
    ],
};

pub const SENSOR_BOX_SETTINGS: Menu = Menu {
    name: "Init Settings",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::SensTiltRoll),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::SensTiltPitch),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::SensTiltYaw),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PitotOffset),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::PitotSpan),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::QnhDelta),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::MagAutoCalib),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioTc),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioIntTc),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::WindTc),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::MeanWindTc),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::GnssConfig),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AntBaselen),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AntSlaveDown),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AntSlaveRight),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioPressTc),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: SENSOR_BOX_COMMANDS_IDX,
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
            content: MenuItemContent::EditItem(Editable::SpeedToFlyPinConfig),
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

pub const LANDING_GEAR_ALARM: Menu = Menu {
    name: "Gear Alarm",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AlarmVolume),
            next_menu_idx: LANDING_GEAR_ALARM_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::GearAlarmModeConfig),
            next_menu_idx: LANDING_GEAR_ALARM_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::GearPinConfig),
            next_menu_idx: LANDING_GEAR_ALARM_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AirbrakesPinConfig),
            next_menu_idx: LANDING_GEAR_ALARM_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const AVERAGE_CLIMB_RATE: Menu = Menu {
    name: "Avg Climb Rate",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AvgClimbRateSrc),
            next_menu_idx: AVERAGE_CLIMB_RATE_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcClimbRate),
            next_menu_idx: AVERAGE_CLIMB_RATE_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const RESET_CONFIG: Menu = Menu {
    name: "Config Reset",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::ResetConfig),
            next_menu_idx: RESET_CONFIG_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::FactoryReset),
            next_menu_idx: RESET_CONFIG_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};
