use crate::{CoreModel, IdleEvent, VarioMode};
use num::clamp;
use num_enum::FromPrimitive;

#[allow(unused_imports)]
use micromath::F32Ext;

#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Waveform {
    #[default]
    Triangular,
    Sawtooth,
    Rectangular,
    SineWave,
}

#[allow(unused)]
pub const WAVEFORM_TRIANGULAR: &str = "Triangular";
pub const WAVEFORM_SAWTOOTH: &str = "Sawtooth";
pub const WAVEFORM_RECTANGULAR: &str = "Rectangular";
pub const WAVEFORM_SINE_WAVE: &str = "Sine wave";

impl From<&str> for Waveform {
    fn from(value: &str) -> Self {
        match value {
            WAVEFORM_SAWTOOTH => Waveform::Sawtooth,
            WAVEFORM_RECTANGULAR => Waveform::Rectangular,
            WAVEFORM_SINE_WAVE => Waveform::SineWave,
            _ => Waveform::Triangular,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SoundParams {
    pub frequency: u16,
    pub continuous: bool,
    pub gain: i8,
    pub waveform: Waveform,
}

impl Default for SoundParams {
    fn default() -> Self {
        SoundParams {
            frequency: 500,
            continuous: false,
            gain: 2,
            waveform: Waveform::Triangular,
        }
    }
}

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
        SoundControl {
            scenario: SoundScenario::Standard as u8,
            tick: 0,
        }
    }
}

impl SoundControl {
    pub fn activate_scenariio(&mut self, scenariio: SoundScenario) {
        self.scenario |= scenariio as u8;
    }

    pub fn clear_scenariio(&mut self, scenariio: SoundScenario) {
        self.scenario &= !(scenariio as u8);
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

        cm.calculated.sound_params.frequency = clamp(
            frequency,
            cm.config.snd_min_freq as u16,
            cm.config.snd_max_freq as u16,
        );
        cm.calculated.sound_params.continuous = continuous;

        if gain != cm.calculated.sound_params.gain {
            cm.calculated.sound_params.gain = gain;
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
            VarioMode::Vario => {
                let climb_rate = cms.climb_rate.to_m_s();
                if climb_rate < cmc.vario_upper_limit.to_m_s()
                    && climb_rate > cmc.vario_lower_limit.to_m_s()
                {
                    (500, true, 0) // be quiet then
                } else {
                    (
                        (cmc.snd_center_freq * (cmc.snd_exp_mul * climb_rate).exp()) as u16,
                        cms.climb_rate.to_m_s() < 0.0,
                        cmc.volume,
                    )
                }
            }
            VarioMode::SpeedToFly => {
                let stf_dif = -cm.calculated.speed_to_fly_dif.to_km_h();
                let stf_val_ms = stf_dif / 10.0;
                if stf_dif < cm.config.stf_upper_limit.to_km_h()
                    && stf_dif > cm.config.stf_lower_limit.to_km_h()
                {
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
            0..=4 => (START_FREQ, false, 0), // silence
            5 => (START_FREQ, true, cm.control.alarm_volume),
            6..=10 => (
                cm.calculated.sound_params.frequency + INC_FREQ,
                true,
                cm.control.alarm_volume,
            ),
            11 => (START_FREQ, false, 0), // silence
            12 => (START_FREQ, true, cm.control.alarm_volume),
            13..=17 => (
                cm.calculated.sound_params.frequency + INC_FREQ,
                true,
                cm.control.alarm_volume,
            ),
            18 => (START_FREQ, false, 0), // silence
            19 => (START_FREQ, true, cm.control.alarm_volume),
            20..=24 => (
                cm.calculated.sound_params.frequency + INC_FREQ,
                true,
                cm.control.alarm_volume,
            ),
            25..=40 => (START_FREQ, false, 0), // silence
            _ => {
                self.tick = 0;
                (START_FREQ, false, 0)
            }
        }
    }
}
