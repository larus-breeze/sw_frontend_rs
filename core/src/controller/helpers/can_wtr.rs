use crate::{CanFrame, CoreModel, Frame, GenericId, PersistenceId, SpecialId};
use byteorder::{ByteOrder, LittleEndian as LE};

const OBJECT_ID: u16 = 4;
const OBJECT_ID_GEN: u16 = 0;

impl CoreModel {
    pub fn can_frame_heartbeat(&self) -> Frame {
        Frame::generic(
            CanFrame::empty_from_id(0x00)
                .push_u16(OBJECT_ID)
                .push_u16(OBJECT_ID_GEN)
                .push_u32(self.device_const.misc.uuid),
            GenericId::Heartbeat as u16,
        )
    }

    pub fn can_frame_sound(&self) -> Frame {
        Frame::specific(
            CanFrame::empty_from_id(0x00)
                .push_u16(self.calculated.frequency)
                .push_u16(self.config.snd_duty_cycle)
                .push_u8(self.calculated.gain as u8)
                .push_u8(if self.calculated.continuous { 1 } else { 0 }),
            SpecialId::Sound as u16,
            OBJECT_ID,
        )
    }

    pub fn can_frame_sys_config(&mut self, config_id: PersistenceId) -> Option<Frame> {
        let mut data = [0u8; 6];
        match config_id {
            PersistenceId::McCready => {
                LE::write_f32(&mut data[2..6], self.config.mc_cready.to_m_s());
            }
            PersistenceId::PilotWeight => {
                LE::write_f32(&mut data[2..6], self.glider_data.pilot_weight.to_kg());
            }
            PersistenceId::VarioModeControl => data[0] = self.control.vario_mode_control as u8,
            PersistenceId::Volume => data[0] = self.config.volume as u8,
            PersistenceId::WaterBallast => {
                LE::write_f32(&mut data[2..6], self.glider_data.water_ballast.to_kg());
            }
            PersistenceId::Qnh => {
                return None;
            }
            _ => return None,
        }

        Some(Frame::generic(
            CanFrame::empty_from_id(0)
                .push_u16(config_id as u16)
                .push_slice(&data),
            GenericId::SetSysSetting as u16,
        ))
    }

    pub fn can_frame_volt_temp(&self) -> Frame {
        Frame::specific(
            CanFrame::empty_from_id(0x00)
                .push_f32(self.device.supply_voltage)
                .push_f32(self.device.temperature_pcb),
            SpecialId::VoltTemp as u16,
            OBJECT_ID,
        )
    }
}
