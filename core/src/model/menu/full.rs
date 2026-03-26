use crate::{
    model::menu::{
        Menu, MenuItem, MenuItemContent, ADVANCED_SETTINGS_IDX, FLIGHT_MENU, ROOT, ROOT_IDX,
        SETTINGS_IDX, VIEW_SETTINGS_IDX,
    },
    Editable,
};

pub const DRAIN_SETTINGS_IDX: usize = 5;
pub const POLAR_SETTINGS_IDX: usize = 6;
pub const SENSOR_BOX_COMMANDS_IDX: usize = 7;
pub const SENSOR_BOX_SETTINGS_IDX: usize = 8;
pub const ADV_SPEED_TO_FLY_IDX: usize = 9;
pub const LANDING_GEAR_ALARM_IDX: usize = 10;
pub const ADV_VARIO_IDX: usize = 11;
pub const UNITS_IDX: usize = 12;
pub const VIEW_STRAIGHT_IDX: usize = 13;
pub const VIEW_CIRCLING_IDX: usize = 14;
pub const USAGE_MODE_AND_PROFILE_IDX: usize = 15;
pub const MORE_SETTINGS_IDX: usize = 16;
pub const SOUND_IDX: usize = 17;

pub const MENU_LIST: &[Menu] = &[
    ROOT,
    FLIGHT_MENU,
    SETTINGS,
    VIEW_SETTINGS,
    ADVANCED_SETTINGS,
    DRAIN_SETTINGS,
    POLAR_SETTINGS,
    SENSOR_BOX_COMMANDS,
    SENSOR_BOX_SETTINGS,
    ADV_SPEED_TO_FLY,
    LANDING_GEAR_ALARM,
    ADV_VARIO,
    UNITS,
    VIEW_STRAIGHT,
    VIEW_CIRCLING,
    USAGE_MODE_AND_PROFILE,
    MORE_SETTINGS,
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
            content: MenuItemContent::EditItem(Editable::Rotation),
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
            next_menu_idx: ADV_VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
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
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SOUND_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: MORE_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: SETTINGS_IDX,
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
            next_menu_idx: SETTINGS_IDX,
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
            next_menu_idx: ROOT_IDX,
        },
        MenuItem {
            content: MenuItemContent::MenuItem(),
            next_menu_idx: SENSOR_BOX_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: SETTINGS_IDX,
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

pub const ADV_SPEED_TO_FLY: Menu = Menu {
    name: "Speed to Fly",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcCircleHysteresis),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcSpeedToFly),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioModeControl),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::SpeedToFlyPinConfig),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::StfUpperLimit),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::StfLowerLimit),
            next_menu_idx: ADV_SPEED_TO_FLY_IDX,
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

pub const ADV_VARIO: Menu = Menu {
    name: "Vario",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::AvgClimbRateSrc),
            next_menu_idx: ADV_VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::TcClimbRate),
            next_menu_idx: ADV_VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioUpperLimit),
            next_menu_idx: ADV_VARIO_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::VarioLowerLimit),
            next_menu_idx: ADV_VARIO_IDX,
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

pub const VIEW_STRAIGHT: Menu = Menu {
    name: "Straight",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::CenterViewStraight),
            next_menu_idx: VIEW_STRAIGHT_IDX,
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
            content: MenuItemContent::EditItem(Editable::ResetConfig),
            next_menu_idx: USAGE_MODE_AND_PROFILE_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::FactoryReset),
            next_menu_idx: USAGE_MODE_AND_PROFILE_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};

pub const MORE_SETTINGS: Menu = Menu {
    name: "More Settings",
    level: 3,
    items: &[
        MenuItem {
            content: MenuItemContent::EditItem(Editable::BatteryGood),
            next_menu_idx: MORE_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::BatteryLow),
            next_menu_idx: MORE_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::FlashControl),
            next_menu_idx: MORE_SETTINGS_IDX,
        },
        MenuItem {
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
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
            content: MenuItemContent::EditItem(Editable::Return),
            next_menu_idx: ADVANCED_SETTINGS_IDX,
        },
    ],
};
