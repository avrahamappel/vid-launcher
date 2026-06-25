mod components;
mod file_operations;
mod weights;

use std::path::{Path, PathBuf};

use async_process::Command;
use iced::widget::{column, row, Button, Column};
use iced::{Element, Length, Task};
use rand::prelude::*;

use crate::components::loading;
use crate::file_operations::*;

fn play_random_video(directory: &Path) -> Task<Event> {
    let video_files = get_video_files(directory);
    if let Ok(random_video) =
        video_files.choose_weighted(&mut rand::rng(), crate::weights::weight_by_last_accessed)
    {
        let cmd = get_open_command();

        Task::perform(
            Command::new(cmd).arg(random_video).status(),
            |res| match res {
                Ok(_) => Event::Complete,
                Err(e) => Event::Error(e.kind().to_string()),
            },
        )
    } else {
        Task::done(Event::Error("No video files found in folder".into()))
    }
}

fn open_folder(directory: &PathBuf) -> Task<Event> {
    let cmd = get_open_command();

    Task::perform(Command::new(cmd).arg(directory).status(), |res| match res {
        Ok(_) => Event::Complete,
        Err(e) => Event::Error(e.kind().to_string()),
    })
}

struct Show {
    name: String,
    path: PathBuf,
}

impl Show {
    fn new(path: PathBuf) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();

        Some(Self { name, path })
    }
}

struct App {
    shows: Vec<Show>,
    loading: bool,
}

impl App {
    fn new() -> Self {
        // TODO do this work on load so we show window right away
        let home_dir = std::env::var("HOME").expect("Failed to get HOME environment variable");
        let videos_dir = PathBuf::from(home_dir).join("Videos");

        let directories = get_subdirectories(&videos_dir);
        let shows = directories.into_iter().filter_map(Show::new).collect();

        Self {
            shows,
            loading: false,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
enum Event {
    PlayRandomVideo(usize),
    BrowseShow(usize),
    Complete,
    Error(String),
}

fn update(app: &mut App, event: Event) -> Task<Event> {
    use Event::*;
    match event {
        PlayRandomVideo(idx) => {
            app.loading = true;
            let show = &app.shows[idx];
            play_random_video(&show.path)
        },
        BrowseShow(idx) => {
            app.loading = true;
            let show = &app.shows[idx];
            open_folder(&show.path)
        },
        Complete => {
            app.loading = false;
            Task::none()
        },
        Error(_) => {
            app.loading = false;
            // TODO display errors
            Task::none()
        },
    }
}

fn view(app: &App) -> Column<'_, Event> {
    let list = app
        .shows
        .iter()
        .enumerate()
        .map(|(idx, show)| {
            // TODO make tiles (after thumbnails)
            Element::from(row![
                Button::new(show.name.as_str())
                    .width(Length::Fill)
                    .on_press(Event::PlayRandomVideo(idx)),
                Button::new("📁").on_press(Event::BrowseShow(idx))
            ])
        })
        .collect::<Column<_>>();

    let root = column![list];

    if app.loading {
        root.push(loading())
    } else {
        root
    }
}

fn main() -> iced::Result {
    iced::application(App::new, update, view)
        .title(option_env!("VID_LAUNCHER_TITLE").unwrap_or("vid-launcher-debug"))
        .window_size((300, 400))
        .run()
}
