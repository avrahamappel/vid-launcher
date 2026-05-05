use std::fs;
use std::path::{Path, PathBuf};

use iced::{
    widget::{column, row, Button, Column, Scrollable, Text},
    Element, Task,
};
use rand::prelude::*;

fn get_subdirectories(path: &Path) -> Vec<PathBuf> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect(),
        Err(_) => vec![],
    }
}

fn get_video_files(directory: &Path) -> Vec<PathBuf> {
    let extensions = ["mp4", "mkv", "avi", "webm"];
    match fs::read_dir(directory) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .flat_map(|e| {
                if e.path().is_dir() {
                    get_video_files(&e.path())
                } else {
                    vec![e.path()]
                }
            })
            .filter(|p| {
                p.is_file()
                    && p.extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|ext| extensions.contains(&ext))
            })
            .collect(),
        Err(_) => vec![],
    }
}

fn get_open_command() -> &'static str {
    if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    }
}

#[derive(Debug, Clone)]
enum Message {
    PlayRandomVideo(PathBuf),
    OpenFolder(PathBuf),
}

struct VidLauncher {
    subdirs: Vec<PathBuf>,
}

impl VidLauncher {
    fn new() -> Self {
        let home_dir = std::env::var("HOME").expect("Failed to get HOME env");
        let videos_dir = PathBuf::from(home_dir).join("Videos");
        let subdirs = get_subdirectories(&videos_dir);

        VidLauncher { subdirs }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlayRandomVideo(dir) => {
                let files = get_video_files(&dir);
                if let Some(random_video) = files.choose(&mut rand::rng()) {
                    let cmd = get_open_command();
                    let _ = std::process::Command::new(cmd).arg(random_video).spawn();
                }
            },
            Message::OpenFolder(dir) => {
                let cmd = get_open_command();
                let _ = std::process::Command::new(cmd).arg(dir).spawn();
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let help_text = Text::new("Select a subfolder to launch a random video:");

        let folder_buttons = self
            .subdirs
            .iter()
            .map(|dir| {
                let folder_name = dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("<unknown>");

                let row_element: Element<_> = row![
                    Button::new(Text::new(folder_name))
                        .on_press(Message::PlayRandomVideo(dir.clone()))
                        .width(iced::Length::Fill),
                    Button::new(Text::new("📁"))
                        .on_press(Message::OpenFolder(dir.clone()))
                        .width(30),
                ]
                .spacing(10)
                .into();

                row_element
            })
            .collect::<Column<_>>();

        let scroll = Scrollable::new(folder_buttons);

        column![help_text, scroll].into()
    }
}

fn main() -> iced::Result {
    iced::application(VidLauncher::new, VidLauncher::update, VidLauncher::view)
        .window_size((320, 500))
        .title("Vid Launcher (iced.rs rewrite)")
        .run()
}
