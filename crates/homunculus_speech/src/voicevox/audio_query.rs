use crate::{Mora, Moras, VowelName};
use bevy::time::{Timer, TimerMode};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioQuery {
    pub accent_phrases: Vec<AccentPhrase>,
    #[serde(rename = "speedScale")]
    pub speed_scale: f32,
    #[serde(rename = "pitchScale")]
    pub pitch_scale: f32,
    #[serde(rename = "intonationScale")]
    pub intonation_scale: f32,
    #[serde(rename = "volumeScale")]
    pub volume_scale: f32,
    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f32,
    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f32,
    #[serde(rename = "pauseLength")]
    pub pause_length: Option<f32>,
    #[serde(rename = "pauseLengthScale")]
    pub pause_length_scale: Option<f32>,
    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: f32,
    #[serde(rename = "outputStereo")]
    pub output_stereo: bool,
    pub kana: Option<String>,
}

impl AudioQuery {
    pub fn to_moras(&self) -> Moras {
        let mut queue = VecDeque::new();
        queue.push_back(Mora {
            timer: self.create_timer(self.pre_phoneme_length),
            vowel: None,
        });
        for phrase in &self.accent_phrases {
            for mora in &phrase.moras {
                let vowel = self.create_vowel(&mora.vowel);
                queue.push_back(Mora {
                    timer: self
                        .create_timer(mora.vowel_length + mora.consonant_length.unwrap_or(0.0)),
                    vowel,
                });
            }
            if let Some(pause_mora) = &phrase.pause_mora {
                queue.push_back(Mora {
                    timer: self.create_timer(pause_mora.vowel_length),
                    vowel: self.create_vowel(&pause_mora.vowel),
                });
            }
        }
        queue.push_back(Mora {
            timer: self.create_timer(self.post_phoneme_length),
            vowel: None,
        });
        Moras(queue)
    }

    #[inline]
    fn create_timer(&self, secs: f32) -> Timer {
        Timer::from_seconds(secs * self.speed_scale, TimerMode::Once)
    }

    #[inline]
    fn create_vowel(&self, name: &str) -> Option<VowelName> {
        match name {
            "a" | "b" | "h" | "l" | "m" | "p" => Some(VowelName::Aa),
            "i" | "d" | "f" | "n" | "r" | "t" | "v" => Some(VowelName::Ih),
            "u" => Some(VowelName::Ou),
            "e" | "j" | "s" | "x" | "y" | "z" => Some(VowelName::Ee),
            "o" | "g" | "q" | "w" => Some(VowelName::Oh),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccentPhrase {
    pub moras: Vec<PauseMora>,
    pub accent: f32,
    pub pause_mora: Option<PauseMora>,
    pub is_interrogative: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PauseMora {
    pub text: String,
    pub consonant: Option<String>,
    pub consonant_length: Option<f32>,
    pub vowel: String,
    pub vowel_length: f32,
    pub pitch: f32,
}
