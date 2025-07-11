use crate::{
    controller::helpers::RemoteConfig, model::editable::Content, CanFrame, CoreModel, Frame, GenericId, SpecialId, RAD_PER_DEGREE
};
use byteorder::{ByteOrder, LittleEndian as LE};

use super::CanConfigId;

const OBJECT_ID: u16 = 4;
const OBJECT_ID_GEN: u16 = 0;

impl CoreModel {
    pub fn can_frame_heartbeat(&self) -> Frame {
        Frame::generic(
            CanFrame::empty_from_id(0x00)
                .push_u16(OBJECT_ID)
                .push_u16(OBJECT_ID_GEN)
                .push_u32(self.config.uuid),
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

    pub fn can_frame_volt_temp(&self) -> Frame {
        Frame::specific(
            CanFrame::empty_from_id(0x00)
                .push_f32(self.device.supply_voltage)
                .push_f32(self.device.temperature_pcb),
            SpecialId::VoltTemp as u16,
            OBJECT_ID,
        )
    }

    pub fn can_frame_avg_climb_rates(&self) -> Frame {
        Frame::specific(
            CanFrame::empty_from_id(0x00)
                .push_f32(self.calculated.av2_climb_rate.to_m_s())
                .push_f32(self.calculated.thermal_climb_rate.to_m_s()),
            SpecialId::AvgClimbRates as u16,
            OBJECT_ID,
        )
    }

    pub fn can_frame_sys_config(&self, config_id: CanConfigId) -> Option<Frame> {
        let mut data = [0u8; 6];
        match config_id {
            CanConfigId::Volume => {
                data[0] = self.config.volume as u8;
            }
            CanConfigId::MacCready => {
                LE::write_f32(&mut data[2..6], self.config.mc_cready.to_m_s());
            }
            CanConfigId::WaterBallast => {
                LE::write_f32(&mut data[2..6], self.glider_data.water_ballast.to_kg());
            }
            CanConfigId::Bugs => {
                LE::write_f32(&mut data[2..6], self.glider_data.bugs);
            }
            CanConfigId::Qnh => {
                LE::write_f32(
                    &mut data[2..6],
                    self.sensor.pressure_altitude.qnh().to_hpa(),
                );
            }
            CanConfigId::PilotWeight => {
                LE::write_f32(&mut data[2..6], self.glider_data.pilot_weight.to_kg());
            }
            CanConfigId::VarioModeControl => return None, // do nothing
            CanConfigId::TcClimbRate => {
                LE::write_f32(&mut data[2..6], self.config.av2_climb_rate_tc);
            }
            CanConfigId::TcSpeedToFly => {
                LE::write_f32(&mut data[2..6], self.config.av_speed_to_fly_tc);
            }
            CanConfigId::VarioMode => {
                data[0] = self.control.vario_mode as u8;
            }
            _ => return None,
        };
        Some(Frame::generic(
            CanFrame::empty_from_id(0)
                .push_u16(config_id as u16)
                .push_slice(&data),
            GenericId::SetSysSetting as u16,
        ))
    }

    pub fn can_frame_remote_config(
        &mut self,
        config_id: CanConfigId,
        get_set: RemoteConfig,
    ) -> Option<Frame> {
        fn set_f32(data: &mut [u8; 6], content: Content, config_id: CanConfigId) -> bool {
            let mut r = false;
            if let Content::F32(Some(val)) = content {
                use defmt::trace;
                trace!("set remote config {}", val);
                let val = match config_id {
                    CanConfigId::SensTiltRoll
                    | CanConfigId::SensTiltPitch
                    | CanConfigId::SensTiltYaw => val * RAD_PER_DEGREE,
                    _ => val,
                };
                LE::write_f32(&mut data[2..6], val);
                r = true;
            }
            r
        }

        let mut data = [0u8; 6];
        let available = match get_set {
            RemoteConfig::Set => {
                data[0] = 1;
                set_f32(&mut data, self.control.editor.content, config_id)
            }
            RemoteConfig::Get => true,
        };

        if available {
            Some(Frame::generic(
                CanFrame::empty_from_id(0)
                    .push_u16(config_id as u16)
                    .push_slice(&data),
                GenericId::SetSysSetting as u16,
            ))
        } else {
            None
        }
    }
}
