use std::{
    io::{BufRead, stdin},
    time::Duration,
};

use rodio::{OutputStream, Sink, Source};

use crate::playable::PlayableChord;

mod chord;
mod note;
mod playable;
mod score;

pub const DEFAULT_BASE: f32 = 523.26;

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
                sinks.clear()
            }
            "gather" => {
                let mut chordname: String = String::new();
                for i in &harmonyms {
                    chordname.push_str(i.to_string().as_str());
                }
                println!("Currently playing: {}", chordname);
            }
            "exit" => return,
            chord => {
                let harmonym = match note::parse_harmonym(chord) {
                    Ok(o) => o.1,
                    Err(e) => {
                        println!("Invalid Harmonym.:{:#?}", e);
                        continue;
                    }
                };
                println!("reverse translated:\n{}\n{:#?}", harmonym, harmonym);
                println!("ratio: {}", harmonym.evaluate());
                if stack {
                    harmonyms.push(harmonym);
                    sink.append(Into::<PlayableChord>::into(harmonym).amplify(0.2));
                } else {
                    sink.append(
                        Into::<PlayableChord>::into(harmonym)
                            .take_duration(Duration::from_secs(3))
                            .amplify(0.2),
                    );
                }
                sinks.push(sink);
            }
        }
    }
}
