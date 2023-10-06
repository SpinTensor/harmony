use iced::{Element, Sandbox, Settings};

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
}
