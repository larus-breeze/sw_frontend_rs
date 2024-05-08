use heapless::Vec;

use super::{VarioModeControl, MAX_PERS_IDS};
use crate::{
    basic_config::{CONTROLLER_TICK_RATE, PERSISTENCE_TIMEOUT}, system_of_units::Speed, themes::{BRIGHT_MODE, DARK_MODE}, CoreController, CoreModel, IdleEvent, Mass, PersistenceId, PersistenceItem
};

impl CoreController {
    pub fn send_idle_event(&mut self, idle_event: IdleEvent) {
        let _ = self.p_idle_events.enqueue(idle_event);
    }

    pub fn restore_persistent_item(&mut self, cm: &mut CoreModel, item: PersistenceItem) {
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
            _ => (),
        }
    }

    pub fn pers_tick(&mut self, cm: &mut CoreModel) {
        if cm.control.pers_ticks > 0 {
            cm.control.pers_ticks -= 1;
            if cm.control.pers_ticks == 0 {
                let mut pids = Vec::<PersistenceId, MAX_PERS_IDS>::new();
                for id in self.pers_vals.iter() {
                    let _ = pids.push(*id);
                }
                self.pers_vals.clear();
                while let Some(id) = pids.pop() {
                    self.store_persistence_id(cm, id);
                }
            }
        }
    }

    pub fn push_persistence_id(&mut self, cm: &mut CoreModel, id: PersistenceId) {
        cm.control.pers_ticks = CONTROLLER_TICK_RATE * PERSISTENCE_TIMEOUT;
        let _ = self.pers_vals.insert(id);
    }

    pub fn store_persistence_id(&mut self, cm: &mut CoreModel, id: PersistenceId) {
        let p_item = match id {
            PersistenceId::Volume => PersistenceItem::from_i8(id, cm.config.volume),
            PersistenceId::McCready => {
                PersistenceItem::from_f32(id, cm.config.mc_cready.to_m_s())
            }
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
                let mode = if cm.config.theme == &DARK_MODE {
                    0
                } else {
                    1
                };
                PersistenceItem::from_i32(id, mode)
            }
            _ => PersistenceItem::do_not_store(),
        };
        self.send_idle_event(crate::IdleEvent::EepromItem(p_item));
    }
}
