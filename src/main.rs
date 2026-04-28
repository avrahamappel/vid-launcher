use iced::{Element, Sandbox, Settings, widget::{Column, Text}};

fn main() -> iced::Result {
    VidLauncher::run(Settings::default())
}

struct VidLauncher;

impl Sandbox for VidLauncher {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Vid Launcher (iced.rs rewrite)")
    }

    fn update(&mut self, _message: Self::Message) {
        // handle messages (none yet)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Column::new()
            .push(Text::new("Welcome to the iced.rs rewrite!"))
            .into()
    }
}
