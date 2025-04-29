use crate::{CoreController, CoreModel, IdleEvent, VarioMode};
use num::clamp;

pub enum AlarmSoundState {
    None = 0b0000_0000,
    Gear = 0b0000_1000,
}

impl core::ops::BitAnd<u8> for AlarmSoundState {
    type Output = bool;

    fn bitand(self, rhs: u8) -> Self::Output {
        self as u8 & rhs != 0
    }
}

#[allow(unused_imports)]
use micromath::F32Ext;

impl CoreController {
    // is called every 100ms
    pub fn sound(&mut self, cm: &mut CoreModel) {
        let (frequency, continuous, gain) = if AlarmSoundState::Gear & cm.control.alarm_sound_state {
            self.gear_alarm_sound(cm)
        } else {
            self.vario_sound(cm)
        };

        cm.calculated.frequency = clamp(
            frequency,
            cm.config.snd_min_freq as u16,
            cm.config.snd_max_freq as u16,
        );
        cm.calculated.continuous = continuous;
        if gain != cm.calculated.gain {
            cm.calculated.gain = gain;
            let event = IdleEvent::SetGain(gain as u8);

            // send event to the idle loop, which handles the amplifier via i2c
            self.send_idle_event(event);
        }

        // create CAN frames
        let can_frame = cm.can_frame_sound();
        let _ = self.p_tx_frames.enqueue(can_frame); // ignore when queue is full
    }

    fn vario_sound(&mut self, cm: &mut CoreModel) -> (u16, bool, i8) {
        // calculate sound parameters and push can frame to queue
        let cms = &cm.sensor;
        let cmc = &cm.config;
        match cm.control.vario_mode {
            VarioMode::Vario => (
                (cmc.snd_center_freq * (cmc.snd_exp_mul * cms.climb_rate.to_m_s()).exp()) as u16,
                cms.climb_rate.to_m_s() < 0.0,
                cmc.volume,
            ),
            VarioMode::SpeedToFly => {
                let sped_to_fly_val = cm.calculated.speed_to_fly_dif.to_km_h() / -10.0;
                if sped_to_fly_val < 1.0 && sped_to_fly_val > -1.0 {
                    (500, true, 0) // speed to fly is ok, so be quiet
                } else {
                    (
                        (cmc.snd_center_freq * (cmc.snd_exp_mul * sped_to_fly_val).exp()) as u16,
                        sped_to_fly_val < 0.0,
                        cmc.volume,
                    )
                }
            }
        }
    }

    fn gear_alarm_sound(&mut self, _cm: &mut CoreModel) -> (u16, bool, i8) {
        (0, false, 0)
    }
}
