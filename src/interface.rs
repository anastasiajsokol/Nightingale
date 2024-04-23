use iced::{Alignment, Element, Sandbox, Settings};
use iced::widget::{button, column, text};

pub fn run() -> iced::Result {
    Interface::run(Settings::default())
}

struct Interface {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Interface {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Nightingale")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}