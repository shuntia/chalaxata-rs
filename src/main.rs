use std::io::{BufRead, stdin};

mod note;
mod score;

fn main() {
    for i in stdin().lock().lines() {
        let harmonym = match note::parse_harmonym(i.unwrap().as_str()) {
            Ok(o) => o.1,
            Err(_) => return,
        };
        println!("{}\n{:#?}", harmonym, harmonym);
    }
}
