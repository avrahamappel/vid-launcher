use iced::{
    executor, 
    widget::{Button, Column, Row, Scrollable, Text}, 
    Application, Command, Element, Settings,
};
use std::fs;
use std::path::{Path, PathBuf};
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
                        .map(|ext| extensions.contains(&ext))
                        .unwrap_or(false)
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
    videos_dir: PathBuf,
    subdirs: Vec<PathBuf>,
}

impl Application for VidLauncher {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let home_dir = std::env::var("HOME").expect("Failed to get HOME env");
        let videos_dir = PathBuf::from(home_dir).join("Videos");
        let subdirs = get_subdirectories(&videos_dir);
        (
            VidLauncher { videos_dir, subdirs },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Vid Launcher (iced.rs rewrite)".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PlayRandomVideo(dir) => {
                let files = get_video_files(&dir);
                if let Some(random_video) = files.choose(&mut rand::thread_rng()) {
                    let cmd = get_open_command();
                    let _ = std::process::Command::new(cmd)
                        .arg(random_video)
                        .spawn();
                }
            }
            Message::OpenFolder(dir) => {
                let cmd = get_open_command();
                let _ = std::process::Command::new(cmd).arg(dir).spawn();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut col = Column::new()
            .push(Text::new("Select a subfolder to launch a random video:"));

        let mut scroll = Scrollable::new(&self.subdirs);

        for dir in &self.subdirs {
            let folder_name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<unknown>");
            let play = Button::new(Text::new(folder_name))
                .on_press(Message::PlayRandomVideo(dir.clone()))
                .width(iced::Length::Fill);
            let open = Button::new(Text::new("📁"))
                .on_press(Message::OpenFolder(dir.clone()))
                .width(30);
            scroll = scroll.push(
                Row::new().push(play).push(open).spacing(10)
            );
        }

        col.push(scroll).into()
    }
}

fn main() -> iced::Result {
    VidLauncher::run(Settings {
        window: iced::window::Settings {
            size: (320, 500),
            ..Default::default()
        },
        ..Settings::default()
    })
}
