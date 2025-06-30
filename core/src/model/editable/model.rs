use super::{Content, EditableFuncs, EnumParams, F32Params, ListParams, Params};
use crate::{
    model::{
        control::{DATA_SOURCE_FRONTEND, DATA_SOURCE_SENSORBOX},
        DataSource, DisplayActive, DisplayTheme,
        config::{VARIO, HORIZON}},
    persist, polar_store,
    utils::{TString, Variant},
    view::viewable::{
        centerview::{CenterType, CenterView},
        lineview::{LineView, Placement},
    },
    CoreController, CoreModel, Echo, FloatToSpeed, PersistenceId,
};
use tfmt::Convert;

pub struct AlarmVolume;
const VOLUME_PARAMS: Params = Params::F32(F32Params {
    min: 0.0,
    max: 50.0,
    small_inc: 1.0,
    big_inc: 3.0,
    dec_places: 0,
    unit: "",
});

impl EditableFuncs for AlarmVolume {
    fn name() -> &'static str {
        "Alarm Volume"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.control.alarm_volume as f32))
    }

    fn params() -> Params {
        VOLUME_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::I8(val as i8),
                PersistenceId::AlarmVolume,
                Echo::None,
            );
        }
    }
}

pub struct AvgClimbRateSrc;
impl EditableFuncs for AvgClimbRateSrc {
    fn name() -> &'static str {
        "Avg Climb Source"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            cm.control.avg_climb_rate_src.as_str(),
        ))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [DATA_SOURCE_FRONTEND, DATA_SOURCE_SENSORBOX, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            let source = DataSource::from(val.as_str());
            persist::persist_set(
                cc,
                cm,
                Variant::U32(source as u32),
                PersistenceId::AvgClimbeRateSrc,
                Echo::None,
            );
        }
    }
}

pub struct BatteryGood;
const BAT_PARAMS: Params = Params::F32(F32Params {
    min: 7.0,
    max: 15.0,
    small_inc: 0.1,
    big_inc: 1.0,
    dec_places: 1,
    unit: "V",
});

impl EditableFuncs for BatteryGood {
    fn name() -> &'static str {
        "Battery Good"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.battery_good))
    }

    fn params() -> Params {
        BAT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::BatteryGood,
                Echo::None,
            );
        }
    }
}

pub struct BatteryLow;
impl EditableFuncs for BatteryLow {
    fn name() -> &'static str {
        "Battery Low"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.battery_low))
    }

    fn params() -> Params {
        BAT_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::BatteryLow,
                Echo::None,
            );
        }
    }
}

pub struct CenterFrequency;
impl EditableFuncs for CenterFrequency {
    fn name() -> &'static str {
        "Center Frequency"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.snd_center_freq))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 500.0,
            max: 1000.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "Hz",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::CenterFrequency,
                Echo::Can,
            );
        }
    }
}

pub struct CenterViewCircling;
impl EditableFuncs for CenterViewCircling {
    fn name() -> &'static str {
        "Center Circling"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::List(
            cm.config
                .center_circling
                .sorted_as_i32(CenterType::Circling),
        )
    }

    fn content_as_str(convert: &mut Convert<20>, idx: i32) {
        convert
            .write_str(CenterView::from_sorted(idx as usize, CenterType::Circling).name())
            .unwrap()
    }

    fn params() -> Params {
        Params::List(ListParams {
            max: CenterView::max(CenterType::Circling) as i32,
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::List(value) = content {
            let variant = CenterView::from_sorted(value as usize, CenterType::Circling) as i32;
            persist::persist_set(
                cc,
                cm,
                Variant::I32(variant),
                PersistenceId::CenterViewCircling,
                Echo::None,
            )
        }
    }
}

pub struct CenterViewStraight;
impl EditableFuncs for CenterViewStraight {
    fn name() -> &'static str {
        "Center Straight"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::List(
            cm.config
                .center_straight
                .sorted_as_i32(CenterType::Straight),
        )
    }

    fn content_as_str(convert: &mut Convert<20>, idx: i32) {
        convert
            .write_str(CenterView::from_sorted(idx as usize, CenterType::Straight).name())
            .unwrap()
    }

    fn params() -> Params {
        Params::List(ListParams {
            max: CenterView::max(CenterType::Straight) as i32,
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::List(value) = content {
            let variant = CenterView::from_sorted(value as usize, CenterType::Straight) as i32;
            persist::persist_set(
                cc,
                cm,
                Variant::I32(variant),
                PersistenceId::CenterViewStraight,
                Echo::None,
            )
        }
    }
}

pub struct Display;

impl EditableFuncs for Display {
    fn name() -> &'static str {
        "Display"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        match cm.config.last_display_active {
            DisplayActive::Horizon => Content::Enum(TString::<16>::from_str(HORIZON)),
            _ => Content::Enum(TString::<16>::from_str(VARIO)),
        }
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [VARIO, HORIZON, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            let display_active = DisplayActive::from(val.as_str());
            persist::persist_set(
                cc,
                cm,
                Variant::U32(display_active as u32),
                PersistenceId::Display,
                Echo::None,
            );
        }
    }
}

pub struct Glider;
impl EditableFuncs for Glider {
    fn name() -> &'static str {
        "Glider"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        let sorted_idx = polar_store::to_sorted_idx(cm.config.glider_idx as usize);
        Content::List(sorted_idx as i32)
    }

    fn content_as_str(convert: &mut Convert<20>, idx: i32) {
        let raw_idx = polar_store::to_raw_idx(idx as usize);
        let name = polar_store::from_raw_idx(raw_idx).name;
        convert.write_str(name).unwrap()
    }

    fn params() -> Params {
        Params::List(ListParams {
            max: polar_store::size() as i32 - 1,
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::List(sorted_idx) = content {
            let raw_idx = polar_store::to_raw_idx(sorted_idx as usize) as u32;
            persist::persist_set(
                cc,
                cm,
                Variant::U32(raw_idx),
                PersistenceId::Glider,
                Echo::None,
            )
        }
    }
}

pub struct GliderSymbol;
const ON: &str = "On";
const OFF: &str = "Off";

impl EditableFuncs for GliderSymbol {
    fn name() -> &'static str {
        "Glider Symbol"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        if cm.config.glider_symbol {
            Content::Enum(TString::<16>::from_str(ON))
        } else {
            Content::Enum(TString::<16>::from_str(OFF))
        }
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [ON, OFF, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::Bool(val.as_str() == ON),
                PersistenceId::GliderSymbol,
                Echo::None,
            );
        }
    }
}

pub struct Info1;
impl EditableFuncs for Info1 {
    fn name() -> &'static str {
        "Info 1 Content"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::List(cm.config.info1.sorted_as_i32(Placement::Top))
    }

    fn content_as_str(convert: &mut Convert<20>, idx: i32) {
        convert
            .write_str(LineView::from_sorted(idx as usize, Placement::Top).name())
            .unwrap()
    }

    fn params() -> Params {
        Params::List(ListParams {
            max: LineView::max(Placement::Top) as i32,
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::List(value) = content {
            let variant = LineView::from_sorted(value as usize, Placement::Top) as i32;
            persist::persist_set(
                cc,
                cm,
                Variant::I32(variant),
                PersistenceId::Info1,
                Echo::None,
            )
        }
    }
}

pub struct Info2;
impl EditableFuncs for Info2 {
    fn name() -> &'static str {
        "Info 2 Content"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::List(cm.config.info2.sorted_as_i32(Placement::Bottom))
    }

    fn content_as_str(convert: &mut Convert<20>, idx: i32) {
        convert
            .write_str(LineView::from_sorted(idx as usize, Placement::Bottom).name())
            .unwrap()
    }

    fn params() -> Params {
        Params::List(ListParams {
            max: LineView::max(Placement::Bottom) as i32,
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::List(value) = content {
            let variant = LineView::from_sorted(value as usize, Placement::Bottom) as i32;
            persist::persist_set(
                cc,
                cm,
                Variant::I32(variant),
                PersistenceId::Info2,
                Echo::None,
            )
        }
    }
}

const ALTERNATING: &str = "StF ALternating";
const CLIMB_RATE: &str = "Climb Rate";

pub struct Info3;
impl EditableFuncs for Info3 {
    fn name() -> &'static str {
        "Info 3 Content"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::Enum(TString::<16>::from_str(
            if cm.config.alt_stf_thermal_climb {
                ALTERNATING
            } else {
                CLIMB_RATE
            }    
        ))
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [ALTERNATING, CLIMB_RATE, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            let b = if val.as_str() == CLIMB_RATE {
                false
            } else {
                true
            };
            persist::persist_set(
                cc,
                cm,
                Variant::Bool(b),
                PersistenceId::StfClimbrateAlt,
                Echo::None,
            );
        }
    }
}

pub struct McCready;
impl EditableFuncs for McCready {
    fn name() -> &'static str {
        "Mac Cready"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.mc_cready.to_m_s()))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 5.0,
            small_inc: 0.1,
            big_inc: 0.1,
            dec_places: 1,
            unit: "m/s",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.m_s()),
                PersistenceId::McCready,
                Echo::NmeaAndCan,
            );
        }
    }
}

pub struct StfUpperLimit;
impl EditableFuncs for StfUpperLimit {
    fn name() -> &'static str {
        "StF Upper Limit"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.stf_upper_limit.to_km_h()))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 50.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "km/h",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.km_h()),
                PersistenceId::StfUpperLimit,
                Echo::None,
            )
        }
    }
}

pub struct StfLowerLimit;
impl EditableFuncs for StfLowerLimit {
    fn name() -> &'static str {
        "StF Lower Limit"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.stf_lower_limit.to_km_h()))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: -50.0,
            max: 0.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "km/h",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::Speed(val.km_h()),
                PersistenceId::StfLowerLimit,
                Echo::None,
            )
        }
    }
}

pub struct TcClimbRate;
impl EditableFuncs for TcClimbRate {
    fn name() -> &'static str {
        "TC Climb Rate"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.av2_climb_rate_tc))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 15.0,
            max: 120.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "s",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::TcClimbRate,
                Echo::Can,
            )
        }
    }
}

pub struct TcSpeedToFly;
impl EditableFuncs for TcSpeedToFly {
    fn name() -> &'static str {
        "TC Speed to Fly"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.av_speed_to_fly_tc))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 1.0,
            max: 60.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "s",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::TcSpeedToFly,
                Echo::Can,
            )
        }
    }
}

pub struct Theme;
const DARK: &str = "Dark";
const BRIGHT: &str = "Bright";

impl EditableFuncs for Theme {
    fn name() -> &'static str {
        "Theme"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        if cm.config.theme == &cm.device_const.dark_theme {
            Content::Enum(TString::<16>::from_str(DARK))
        } else {
            Content::Enum(TString::<16>::from_str(BRIGHT))
        }
    }

    fn params() -> Params {
        Params::Enum(EnumParams {
            variants: [DARK, BRIGHT, "", "", ""],
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::Enum(val) = content {
            let theme = match val.as_str() {
                BRIGHT => DisplayTheme::Bright,
                _ => DisplayTheme::Dark,
            };
            persist::persist_set(
                cc,
                cm,
                Variant::U32(theme as u32),
                PersistenceId::DisplayTheme,
                Echo::None,
            );
        }
    }
}

pub struct Volume;
impl EditableFuncs for Volume {
    fn name() -> &'static str {
        "Volume"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.config.volume as f32))
    }

    fn params() -> Params {
        VOLUME_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::I8(val as i8),
                PersistenceId::Volume,
                Echo::NmeaAndCan,
            );
        }
    }
}
