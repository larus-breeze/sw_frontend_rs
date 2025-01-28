use super::{
    DisplayActive, Editable, VarioModeControl, DEFAULT_CONFIG, DO_NOT_CHANGE, FACTORY_RESET,
    USER_1, USER_2, USER_3, USER_4,
};
use crate::{
    controller::persist,
    flight_physics::polar_store,
    utils::{TString, Variant},
    view::viewable::{
        centerview::{CenterType, CenterView},
        lineview::{LineView, Placement},
    },
    CoreController, CoreModel, Echo, FloatToMass, FloatToSpeed, PersistenceId, Rotation,
};

impl Editable {
    pub fn set_enum_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: &TString<16>) {
        match self {
            Editable::Display => match val.as_str() {
                "Horizon" => persist::persist_set(
                    cc,
                    cm,
                    Variant::DisplayActive(DisplayActive::Horizon),
                    PersistenceId::Display,
                    Echo::None,
                ),
                _ => persist::persist_set(
                    cc,
                    cm,
                    Variant::DisplayActive(DisplayActive::Vario),
                    PersistenceId::Display,
                    Echo::None,
                ),
            },
            Editable::Theme => persist::persist_set(
                cc,
                cm,
                Variant::Str(val.as_str()),
                PersistenceId::DisplayTheme,
                Echo::None,
            ),
            Editable::VarioModeControl => {
                let mode = match val.as_str() {
                    "Vario" => VarioModeControl::Vario,
                    "SpeedToFly" => VarioModeControl::SpeedToFly,
                    _ => VarioModeControl::Auto,
                };
                persist::persist_set(
                    cc,
                    cm,
                    Variant::VarioModeControl(mode),
                    PersistenceId::VarioModeControl,
                    Echo::NmeaAndCan,
                );
            }
            Editable::Rotation => persist::persist_set(
                cc,
                cm,
                Variant::Rotation(Rotation::from(val.as_str())),
                PersistenceId::Rotation,
                Echo::None,
            ),
            Editable::ResetConfig => {
                cm.control.reset_config = match val.as_str() {
                    DEFAULT_CONFIG => 0,
                    FACTORY_RESET => 1,
                    _ => 2,
                };
                if cm.control.editor.enter_pushed {
                    match cm.control.reset_config {
                        0 => persist::delete_config(cc),
                        1 => persist::factory_reset(cc),
                        _ => (),
                    }
                }
            }
            Editable::UserProfile => {
                if cm.control.editor.enter_pushed && val.as_str() != DO_NOT_CHANGE {
                    cm.config.user_profile = match val.as_str() {
                        USER_1 => 0,
                        USER_2 => 1,
                        USER_3 => 2,
                        USER_4 => 3,
                        _ => 0,
                    };
                    persist::user_profile(cc, cm); // store value and reset device
                }
            }
            _ => (),
        }
    }

    pub fn set_f32_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: f32) {
        match self {
            Editable::Bugs => persist::persist_set(
                cc,
                cm,
                Variant::F32(1.0 + val / 100.0),
                PersistenceId::Bugs,
                Echo::NmeaAndCan,
            ),
            Editable::McCready => persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.m_s()),
                PersistenceId::McCready,
                Echo::NmeaAndCan,
            ),
            Editable::PilotWeight => persist::persist_set(
                cc,
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::PilotWeight,
                Echo::NmeaAndCan,
            ),
            Editable::TcClimbRate => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::TcClimbRate,
                Echo::Can,
            ),
            Editable::TcSpeedToFly => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::TcSpeedToFly,
                Echo::Can,
            ),
            Editable::Volume => persist::persist_set(
                cc,
                cm,
                Variant::I8(val as i8),
                PersistenceId::Volume,
                Echo::NmeaAndCan,
            ),
            Editable::WaterBallast => persist::persist_set(
                cc,
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::WaterBallast,
                Echo::NmeaAndCan,
            ),
            Editable::CenterFrequency => persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::CenterFrequency,
                Echo::Can,
            ),
            _ => (),
        }
    }

    #[allow(clippy::single_match)]
    pub fn set_list_content(&self, cm: &mut CoreModel, cc: &mut CoreController, val: i32) {
        match self {
            Editable::Glider => {
                let raw_idx = polar_store::to_raw_idx(val as usize);
                persist::persist_set(
                    cc,
                    cm,
                    Variant::Usize(raw_idx),
                    PersistenceId::Glider,
                    Echo::None,
                )
            }
            Editable::Info1 => {
                let variant = LineView::from_sorted(val as usize, Placement::Top) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::Info1,
                    Echo::None,
                )
            }
            Editable::Info2 => {
                let variant = LineView::from_sorted(val as usize, Placement::Bottom) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::Info2,
                    Echo::None,
                )
            }
            Editable::CenterViewCircling => {
                let variant = CenterView::from_sorted(val as usize, CenterType::Circling) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::CenterViewCircling,
                    Echo::None,
                )
            }
            Editable::CenterViewStraight => {
                let variant = CenterView::from_sorted(val as usize, CenterType::Straight) as i32;
                persist::persist_set(
                    cc,
                    cm,
                    Variant::I32(variant),
                    PersistenceId::CenterViewStraight,
                    Echo::None,
                )
            }
            _ => (),
        }
    }
}
