use std::time::Duration;

use rodio::Source;

use crate::chord::{Chord, FullChord};

pub const SAMPLE_RATE: u32 = 48000;

pub const DEFAULT_WAVE: Waveform = Waveform::Triangle;

pub struct PlayableChord {
    chord: FullChord,
    current_sample: u32,
    wavetype: Waveform,
}

impl From<FullChord> for PlayableChord {
    fn from(value: FullChord) -> Self {
        Self {
            chord: value,
            current_sample: 0,
            wavetype: DEFAULT_WAVE,
        }
    }
}

impl From<Chord> for PlayableChord {
    fn from(value: Chord) -> Self {
        Self {
            chord: value.into(),
            current_sample: 0,
            wavetype: DEFAULT_WAVE,
        }
    }
}

impl Iterator for PlayableChord {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.current_sample as f32 / SAMPLE_RATE as f32;
        let mut total = 0.;

        for i in &self.chord.tones {
            let phase = 2.0 * std::f32::consts::PI * (*i * self.chord.base) * t;
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
        Some(total * self.chord.tones.len() as f32)
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
