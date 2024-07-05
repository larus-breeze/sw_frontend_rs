use heapless::Vec;

use super::{VarioModeControl, MAX_PERS_IDS};
use crate::{
    basic_config::PERSISTENCE_TIMEOUT,
    controller::helpers::IntToDuration,
    system_of_units::Speed,
    themes::{BRIGHT_MODE, DARK_MODE},
    CoreController, CoreModel, Mass, PersistenceId, PersistenceItem, Pressure,
};

#[derive(PartialEq)]
pub enum Echo {
    None,
    Nmea,
    Can,
    NmeaAndCan,
}

impl CoreController {
    pub fn persist_restore_item(&mut self, cm: &mut CoreModel, item: PersistenceItem) {
        match item.id {
            PersistenceId::Volume => cm.config.volume = item.to_i8(),
            PersistenceId::McCready => cm.config.mc_cready = Speed::from_m_s(item.to_f32()),
            PersistenceId::WaterBallast => {
                cm.glider_data.water_ballast = Mass::from_kg(item.to_f32())
            }
            PersistenceId::PilotWeight => {
                cm.glider_data.pilot_weight = Mass::from_kg(item.to_f32())
            }
            PersistenceId::Glider => cm.config.glider_idx = item.to_i32(),
            PersistenceId::VarioModeControl => {
                cm.control.vario_mode_control = VarioModeControl::from(item.to_u8())
            }
            PersistenceId::DisplayMode => {
                cm.config.theme = if item.to_i32() == 0 {
                    &DARK_MODE
                } else {
                    &BRIGHT_MODE
                }
            }
            PersistenceId::Bugs => cm.glider_data.bugs = item.to_f32(),
            PersistenceId::Qnh => {
                let qnh = Pressure::from_hpa(item.to_f32());
                cm.sensor.pressure_altitude.set_qnh(qnh)
            }
            _ => (),
        }
    }

    pub fn persist_push_id(&mut self, id: PersistenceId) {
        self.scheduler
            .after(crate::Timer::PersistSetting, PERSISTENCE_TIMEOUT.millis());
        let _ = self.pers_vals.insert(id);
    }

    pub fn persist_store_id(&mut self, cm: &mut CoreModel, id: PersistenceId) {
        let p_item = match id {
            PersistenceId::Volume => PersistenceItem::from_i8(id, cm.config.volume),
            PersistenceId::McCready => PersistenceItem::from_f32(id, cm.config.mc_cready.to_m_s()),
            PersistenceId::WaterBallast => {
                PersistenceItem::from_f32(id, cm.glider_data.water_ballast.to_kg())
            }
            PersistenceId::PilotWeight => {
                PersistenceItem::from_f32(id, cm.glider_data.pilot_weight.to_kg())
            }
            PersistenceId::Glider => PersistenceItem::from_i32(id, cm.config.glider_idx),
            PersistenceId::VarioModeControl => {
                PersistenceItem::from_u8(id, cm.control.vario_mode_control as u8)
            }
            PersistenceId::DisplayMode => {
                let mode = if cm.config.theme == &DARK_MODE { 0 } else { 1 };
                PersistenceItem::from_i32(id, mode)
            }
            PersistenceId::Bugs => PersistenceItem::from_f32(id, cm.glider_data.bugs),
            PersistenceId::Qnh => {
                PersistenceItem::from_f32(id, cm.sensor.pressure_altitude.qnh().to_hpa())
            }
            _ => PersistenceItem::do_not_store(),
        };
        self.send_idle_event(crate::IdleEvent::EepromItem(p_item));
    }

    pub fn persist_set_bugs(&mut self, cm: &mut CoreModel, val: f32, echo: Echo) {
        cm.glider_data.bugs = val;
        self.persist_finish_push(cm, PersistenceId::Bugs, echo);
    }

    pub fn persist_set_glider_idx(&mut self, cm: &mut CoreModel, val: i32, echo: Echo) {
        cm.config.glider_idx = val;
        self.persist_finish_push(cm, PersistenceId::Glider, echo);
    }

    pub fn persist_set_maccready(&mut self, cm: &mut CoreModel, val: Speed, echo: Echo) {
        cm.config.mc_cready = val;
        self.persist_finish_push(cm, PersistenceId::McCready, echo);
    }

    pub fn persist_set_pilot_weight(&mut self, cm: &mut CoreModel, val: Mass, echo: Echo) {
        cm.glider_data.pilot_weight = val;
        self.persist_finish_push(cm, PersistenceId::PilotWeight, echo);
    }

    pub fn persist_set_pilot_qnh(&mut self, cm: &mut CoreModel, val: Pressure, echo: Echo) {
        cm.sensor.pressure_altitude.set_qnh(val);
        self.persist_finish_push(cm, PersistenceId::Qnh, echo);
    }

    pub fn persist_set_vario_mode_control(
        &mut self,
        cm: &mut CoreModel,
        val: VarioModeControl,
        echo: Echo,
    ) {
        cm.control.vario_mode_control = val;
        self.persist_finish_push(cm, PersistenceId::VarioModeControl, echo);
    }

    pub fn persist_set_volume(&mut self, cm: &mut CoreModel, val: i8, echo: Echo) {
        cm.config.volume = val;
        self.persist_finish_push(cm, PersistenceId::Volume, echo);
    }

    pub fn persist_set_water_ballast(&mut self, cm: &mut CoreModel, val: Mass, echo: Echo) {
        cm.glider_data.water_ballast = val;
        self.persist_finish_push(cm, PersistenceId::WaterBallast, echo);
    }

    fn persist_finish_push(&mut self, cm: &mut CoreModel, id: PersistenceId, echo: Echo) {
        if echo == Echo::Nmea || echo == Echo::NmeaAndCan {
            let _ = self.nmea_vals.insert(id);
        }
        if echo == Echo::Can || echo == Echo::NmeaAndCan {
            if let Some(frame) = cm.can_frame_sys_config(id) {
                let _ = self.p_tx_frames.enqueue(frame);
            }
        }
        self.persist_push_id(id);
    }
}

pub fn store_persistence_ids(cm: &mut CoreModel, cc: &mut CoreController) {
    let mut ids = Vec::<PersistenceId, MAX_PERS_IDS>::new();

    // We must first copy the ids into a Vec, because we can't borrow cc twice
    for id in cc.pers_vals.iter() {
        let _ = ids.push(*id);
    }
    cc.pers_vals.clear();
    while let Some(id) = ids.pop() {
        cc.persist_store_id(cm, id);
    }

    ids.clear();
    // We must first copy the ids into a Vec, because we can't borrow cc twice
    for id in cc.nmea_vals.iter() {
        let _ = ids.push(*id);
    }
    cc.nmea_vals.clear();
    while let Some(id) = ids.pop() {
        cc.nmea_config(id);
    }
}
