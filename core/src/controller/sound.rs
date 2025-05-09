use crate::{CoreModel, IdleEvent, VarioMode};
use num::clamp;

#[allow(unused_imports)]
use micromath::F32Ext;

#[allow(unused)]
pub enum SoundScenario {
    Standard = 0b0000_0000,
    GearAlarm = 0b0000_1000,
}

impl core::ops::BitAnd<u8> for SoundScenario {
    type Output = bool;

    fn bitand(self, rhs: u8) -> Self::Output {
        self as u8 & rhs != 0
    }
}

pub struct SoundControl {
    scenario: u8,
    tick: u16,
}

impl Default for SoundControl {
    fn default() -> Self {
        SoundControl { scenario: SoundScenario::Standard as u8, tick: 0 }
    }
}

impl SoundControl {

    pub fn activate_scenariio(&mut self, scenariio: SoundScenario) {
        self.scenario = self.scenario | scenariio as u8;
    }

    pub fn clear_scenariio(&mut self, scenariio: SoundScenario) {
        self.scenario = self.scenario & !(scenariio as u8);
    }

    pub fn set_scenario(&mut self, scenariio: SoundScenario, active: bool) {
        if active {
            self.activate_scenariio(scenariio);
        } else {
            self.clear_scenariio(scenariio);
        }
    }

    // is called every 100ms
    pub fn sound(&mut self, cm: &mut CoreModel) -> Option<IdleEvent> {
        let (frequency, continuous, gain) = if SoundScenario::GearAlarm & self.scenario {
            self.gear_alarm_sound(cm)
        } else {
            self.tick = 0;
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
            Some(event)
        } else {
            None
        }
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
                let stf_dif = -cm.calculated.speed_to_fly_dif.to_km_h();
                let stf_val_ms = stf_dif / 10.0;
                if stf_dif < cm.config.stf_upper_limit.to_km_h() && stf_dif > cm.config.stf_lower_limit.to_km_h() {
                    (500, true, 0) // speed to fly is ok, so be quiet
                } else {
                    (
                        (cmc.snd_center_freq * (cmc.snd_exp_mul * stf_val_ms).exp()) as u16,
                        stf_val_ms < 0.0,
                        cmc.volume,
                    )
                }
            }
        }
    }

    fn gear_alarm_sound(&mut self, cm: &mut CoreModel) -> (u16, bool, i8) {
        const START_FREQ: u16 = 700;
        const INC_FREQ: u16 = 150;

        self.tick += 1;

        match self.tick {
            0..=4 | 11 | 18..=21 => (START_FREQ, false, 0), // silence
            5 | 12 => (START_FREQ, true, cm.control.alarm_volume),
            6..=10 | 13..=17 => (cm.calculated.frequency + INC_FREQ, true, cm.control.alarm_volume),
            22..=60 => self.vario_sound(cm),
            _ => {
                self.tick = 0;
                (START_FREQ, false, 0)
            }
        }
    }
}
