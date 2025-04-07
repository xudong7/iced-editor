use iced::widget::text;
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            // Handle messages here
        }
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, iced!").into()
    }
}
