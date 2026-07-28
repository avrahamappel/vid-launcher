#![allow(clippy::enum_glob_use)]

mod app;
mod components;
mod config;
mod file_operations;
mod shows;
mod thumbnails;
mod utils;
mod weights;

fn main() -> iced::Result {
    iced::application(app::init, app::update, app::view)
        .title(option_env!("VID_LAUNCHER_TITLE").unwrap_or("vid-launcher-debug"))
        .window_size((app::WINDOW_WIDTH, 400))
        .run()
}
