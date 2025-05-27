use super::{Content, EditableFuncs, F32Params, Params};
use crate::{persist, utils::Variant, CoreController, CoreModel, Echo, FloatToMass, PersistenceId};

pub struct Bugs;
impl EditableFuncs for Bugs {
    fn name() -> &'static str {
        "Bugs"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some((cm.glider_data.bugs - 1.0) * 100.0))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 100.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "%",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(1.0 + val / 100.0),
                PersistenceId::Bugs,
                Echo::NmeaAndCan,
            )
        }
    }
}

pub struct PilotWeight;
impl EditableFuncs for PilotWeight {
    fn name() -> &'static str {
        "Pilot Weight"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.pilot_weight.to_kg()))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 250.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "kg",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::PilotWeight,
                Echo::NmeaAndCan,
            )
        }
    }
}

pub struct WaterBallast;
impl EditableFuncs for WaterBallast {
    fn name() -> &'static str {
        "Water Ballast"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.water_ballast.to_kg()))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 250.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "kg",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::Mass(val.kg()),
                PersistenceId::WaterBallast,
                Echo::NmeaAndCan,
            )
        }
    }
}

pub struct EmptyMass;
impl EditableFuncs for EmptyMass {
    fn name() -> &'static str {
        "Empty Mass"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.empty_mass))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 100.0,
            max: 850.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "kg",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::EmptyMass,
                Echo::Can,
            )
        }
    }
}

pub struct MaxBallast;
impl EditableFuncs for MaxBallast {
    fn name() -> &'static str {
        "Max Ballast"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.max_ballast))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 0.0,
            max: 300.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "kg",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::MaxBallast,
                Echo::Can,
            )
        }
    }
}

pub struct ReferenceWeight;
impl EditableFuncs for ReferenceWeight {
    fn name() -> &'static str {
        "Reference Weight"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.reference_weight))
    }

    fn params() -> Params {
        Params::F32(F32Params {
            min: 100.0,
            max: 1000.0,
            small_inc: 1.0,
            big_inc: 10.0,
            dec_places: 0,
            unit: "kg",
        })
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::ReferenceWeight,
                Echo::Can,
            )
        }
    }
}

pub struct PolarValueV1;
const V_PARAMS: Params = Params::F32(F32Params {
    min: 50.0,
    max: 250.0,
    small_inc: 1.0,
    big_inc: 10.0,
    dec_places: 0,
    unit: "km/h",
});

impl EditableFuncs for PolarValueV1 {
    fn name() -> &'static str {
        "Polar V 1"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[0][0]))
    }

    fn params() -> Params {
        V_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueV1,
                Echo::Can,
            )
        }
    }
}

pub struct PolarValueV2;
impl EditableFuncs for PolarValueV2 {
    fn name() -> &'static str {
        "Polar V 2"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[1][0]))
    }

    fn params() -> Params {
        V_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueV2,
                Echo::Can,
            )
        }
    }
}

pub struct PolarValueV3;
impl EditableFuncs for PolarValueV3 {
    fn name() -> &'static str {
        "Polar V 3"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[2][0]))
    }

    fn params() -> Params {
        V_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueV3,
                Echo::Can,
            )
        }
    }
}

pub struct PolarValueSi1;
const SI_PARAMS: Params = Params::F32(F32Params {
    min: -5.0,
    max: 0.0,
    small_inc: 0.01,
    big_inc: 0.1,
    dec_places: 2,
    unit: "m/s",
});

impl EditableFuncs for PolarValueSi1 {
    fn name() -> &'static str {
        "Polar Si 1"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[0][1]))
    }

    fn params() -> Params {
        SI_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueSi1,
                Echo::Can,
            )
        }
    }
}

pub struct PolarValueSi2;
impl EditableFuncs for PolarValueSi2 {
    fn name() -> &'static str {
        "Polar Si 2"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[1][1]))
    }

    fn params() -> Params {
        SI_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueSi2,
                Echo::Can,
            )
        }
    }
}

pub struct PolarValueSi3;
impl EditableFuncs for PolarValueSi3 {
    fn name() -> &'static str {
        "Polar Si 3"
    }

    fn content(cm: &mut CoreModel, _cc: &mut CoreController) -> Content {
        Content::F32(Some(cm.glider_data.basic_glider_data.polar_values[2][1]))
    }

    fn params() -> Params {
        SI_PARAMS
    }

    fn set_content(cm: &mut CoreModel, cc: &mut CoreController, content: Content) {
        if let Content::F32(Some(val)) = content {
            persist::persist_set(
                cc,
                cm,
                Variant::F32(val),
                PersistenceId::PolarValueSi3,
                Echo::Can,
            )
        }
    }
}
