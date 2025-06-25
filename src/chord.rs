use nom::IResult;

use crate::{
    DEFAULT_BASE,
    note::{Harmonym, parse_harmonym},
};

pub struct Chord {
    pub tones: Vec<Harmonym>,
}

impl From<Vec<Harmonym>> for Chord {
    fn from(value: Vec<Harmonym>) -> Self {
        Self { tones: value }
    }
}

impl Chord {
    pub fn parse_chord(s: &str) -> IResult<&str, Chord> {
        let parse_result = split_pascal(s).into_iter().map(|el| parse_harmonym(el));
        let mut ret = Vec::new();
        for i in parse_result {
            ret.push(i?.1);
        }
        Ok(("", Self { tones: ret }))
    }
    pub fn sort(&mut self) {
        self.tones.sort();
    }
}

fn split_pascal(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let chars = s.char_indices().peekable();
    for (i, c) in chars {
        if c.is_uppercase() && i != 0 {
            result.push(&s[start..i]);
            start = i;
        }
    }
    if start < s.len() {
        result.push(&s[start..]);
    }
    result
}

impl From<Chord> for FullChord {
    fn from(val: Chord) -> Self {
        FullChord {
            tones: val.tones,
            base: DEFAULT_BASE,
        }
    }
}

pub struct FullChord {
    pub tones: Vec<Harmonym>,
    pub base: f32,
}
