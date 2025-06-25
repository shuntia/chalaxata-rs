use std::{
    io::{BufRead, stdin},
    sync::atomic::AtomicBool,
    time::Duration,
};

use rodio::{OutputStream, Sink, Source};

use crate::{chord::Chord, playable::PlayableChord};

mod chord;
mod note;
mod playable;
mod score;

pub const DEFAULT_BASE: f32 = 523.26;
pub static STRUMMING: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut stack = false;
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let mut lines = stdin().lock().lines();
    let mut sinks = Vec::new();
    let mut harmonyms: Vec<note::Harmonym> = Vec::new();
    while let Some(Ok(i)) = lines.next() {
        let sink = Sink::try_new(&handle).unwrap();
        match i.as_str() {
            "stack" => {
                if stack {
                    println!("Stopped stacking");
                    stack = false;
                    sinks.clear();
                    harmonyms.clear();
                } else {
                    println!("Stacking");
                    stack = true;
                }
            }
            "stop" => {
                println!("All sounds stopped");
                sinks.clear();
                harmonyms.clear();
            }
            "gather" => {
                let mut chordname: String = String::new();
                for i in &harmonyms {
                    chordname.push_str(i.to_string().as_str());
                }
                println!("Currently playing: {}", chordname);
            }
            "strum" => {
                println!("strumming.");
                STRUMMING.fetch_not(std::sync::atomic::Ordering::Release);
            }
            "exit" => return,
            chord => {
                let mut chord: Chord = match Chord::parse_chord(chord) {
                    Ok(o) => o.1,
                    Err(e) => {
                        println!("Failed to parse chord: {:?}", e);
                        continue;
                    }
                };
                chord.sort();
                println!(
                    "ratio: {}",
                    chord
                        .tones
                        .iter()
                        .map(|el| format!("{}, ", el.eval().to_string()))
                        .collect::<String>()
                );
                if stack {
                    harmonyms.append(&mut chord.tones.clone());
                    let mut playable_chord: PlayableChord = chord.into();
                    playable_chord.prerender();
                    sink.append(playable_chord.amplify(0.2));
                } else {
                    let mut playable_chord: PlayableChord = chord.into();
                    playable_chord.prerender();
                    sink.append(
                        playable_chord
                            .take_duration(Duration::from_secs(3))
                            .amplify(0.2),
                    );
                }
                sinks.push(sink);
            }
        }
    }
}
