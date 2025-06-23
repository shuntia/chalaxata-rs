use crate::note::Harmonym;

pub struct Chord {
    pub tones: Vec<Harmonym>,
}

pub struct FullChord {
    pub tones: Vec<Harmonym>,
    pub base: f32,
}
