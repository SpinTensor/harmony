mod notenames;
mod accidentals;
mod notes;
mod notesequences;
mod diatonic_scales;

use iced::{Element, Sandbox, Settings};
use crate::notes::Note;
use crate::diatonic_scales::{DiatonicScale, Mode};

struct Harmony {
}

#[derive(Debug)]
enum Message {
}

impl Sandbox for Harmony {
    type Message = ();

    fn new() -> Self {
        Self {
        }
    }

    fn title(&self) -> String {
        String::from("Harmony Trainer")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<Self::Message> {
        "Harmony".into()
    }
}

fn main() {
    let _ = Harmony::run(Settings::default());
    println!("{:?}", DiatonicScale::from_tonic(Note::from_str("Gb3").unwrap(), Mode::Ionian));
}
