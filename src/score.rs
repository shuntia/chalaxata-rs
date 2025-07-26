use std::time::Duration;

use crate::{DEFAULT_BASE, note::Harmonym, playable::SAMPLE_RATE};

pub struct Score {
    sample_rate: u32,
    total_frames: u32,
    notes: Vec<(NoteDuration, Harmonym)>,
}

impl Into<PlayableScore> for Score {
    fn into(self) -> PlayableScore {
        PlayableScore {
            score: self,
            current_frame: 0,
            base: DEFAULT_BASE,
        }
    }
}

impl Score {
    pub fn push(&mut self, note: (NoteDuration, Harmonym)) {
        self.notes.push(note);
    }
    pub fn new() -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            total_frames: 0,
            notes: vec![],
        }
    }
}

pub struct NoteDuration {
    pub start: u32,
    pub dur: Duration,
}

pub struct PlayableScore {
    score: Score,
    current_frame: u32,
    base: f32,
}
