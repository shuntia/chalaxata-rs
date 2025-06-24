use std::time::Duration;

use rodio::Source;

use crate::{
    DEFAULT_BASE,
    chord::{Chord, FullChord},
    note::Harmonym,
};

pub const SAMPLE_RATE: u32 = 48000;

pub const DEFAULT_WAVE: Waveform = Waveform::Triangle;

pub struct PlayableChord {
    harmonyms: Vec<Harmonym>,
    base: f32,
    current_sample: u32,
    wavetype: Waveform,
    cache: Vec<Vec<f32>>,
}

impl PlayableChord {
    pub fn prerender(&mut self) {}
}

impl From<FullChord> for PlayableChord {
    fn from(value: FullChord) -> Self {
        Self {
            harmonyms: value.tones,
            base: value.base,
            current_sample: 0,
            wavetype: DEFAULT_WAVE,
            cache: Vec::new(),
        }
    }
}

impl From<Chord> for PlayableChord {
    fn from(value: Chord) -> Self {
        Self {
            harmonyms: value.tones,
            base: DEFAULT_BASE,
            current_sample: 0,
            wavetype: DEFAULT_WAVE,
            cache: Vec::new(),
        }
    }
}

impl Iterator for PlayableChord {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.current_sample as f32 / SAMPLE_RATE as f32;
        let mut total = 0.;

        for i in &self.harmonyms {
            let phase = 2.0 * std::f32::consts::PI * (*i * self.base) * t;
            let sample = match self.wavetype {
                Waveform::Sine => phase.sin(),
                Waveform::Square => {
                    if phase.sin() >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                Waveform::Saw => {
                    2.0 * (phase / (2.0 * std::f32::consts::PI)
                        - ((phase / (2.0 * std::f32::consts::PI)) + 0.5).floor())
                }
                Waveform::Triangle => (2.0 / std::f32::consts::PI) * (phase * 0.5).sin().asin(),
            };
            total += sample;
        }

        self.current_sample += 1;
        Some(total * self.harmonyms.len() as f32)
    }
}
impl Source for PlayableChord {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}
