use heapless::Vec;

use crate::{
    IdleEvent, Mass, PersistenceItem, CoreModel, PersistenceId, 
    system_of_units::Speed, basic_config::{CONTROLLER_TICK_RATE, PERSISTENCE_TIMEOUT},
};
use super::MAX_PERS_IDS;

impl CoreModel {
    pub fn send_idle_event(&mut self, idle_event: IdleEvent) {
        let _ = self.p_idle_events.enqueue(idle_event);
    }

    pub fn restore_persistent_item(&mut self, item: PersistenceItem) {
        match item.id {
            PersistenceId::Volume => self.config.volume = item.to_i8(),
            PersistenceId::McCready => self.config.mc_cready = Speed::from_m_s(item.to_f32()),
            PersistenceId::WaterBallast => {
                self.glider_data.water_ballast = Mass::from_kg(item.to_f32())
            }
            PersistenceId::PilotWeight => {
                self.glider_data.pilot_weight = Mass::from_kg(item.to_f32())
            }
            PersistenceId::Glider => self.config.glider_idx = item.to_i32(),
            _ => (),
        }
    }

    pub fn pers_tick(&mut self) {
        if self.control.pers_ticks > 0 {
            self.control.pers_ticks -= 1;
            if self.control.pers_ticks == 0 {
        let mut pids = Vec::<PersistenceId, MAX_PERS_IDS>::new();
                for id in self.control.pers_vals.iter() {
                    let _ = pids.push(*id);
                }
                self.control.pers_vals.clear();
                while let Some(id) = pids.pop() {
                    self.store_persistence_id(id);
                }
                    }
        }
    }

    pub fn push_persistence_id(&mut self, id: PersistenceId) {
        self.control.pers_ticks = CONTROLLER_TICK_RATE * PERSISTENCE_TIMEOUT;
        let _ = self.control.pers_vals.insert(id);
    }

    pub fn store_persistence_id(&mut self, id: PersistenceId) {
        let p_item = match id {
            PersistenceId::Volume => PersistenceItem::from_i8(id, self.config.volume),
            PersistenceId::McCready => PersistenceItem::from_f32(id, self.config.mc_cready.to_m_s()),
            PersistenceId::WaterBallast => PersistenceItem::from_f32(id, self.glider_data.water_ballast.to_kg()),
            PersistenceId::PilotWeight => PersistenceItem::from_f32(id,self.glider_data.pilot_weight.to_kg()),
            PersistenceId::Glider => PersistenceItem::from_i32(id, self.config.glider_idx),
            _ => PersistenceItem::do_not_store(),
        };
        self.send_idle_event(crate::IdleEvent::EepromItem(p_item));
    }
    

}