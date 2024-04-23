use iced::{Application, Command, Element, Settings, Theme};
use iced::executor;

pub fn run() -> iced::Result {
    Interface::run(Settings::default())
}

struct Interface;

impl Application for Interface {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Interface, Command<Self::Message>) {
        (Interface, Command::none())
    }

    fn title(&self) -> String {
        String::from("Nightingale")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }
}