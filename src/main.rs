mod components;
mod file_operations;
mod weights;

use std::path::{Path, PathBuf};
use std::time::Duration;

use async_io::Timer;
use async_process::Command;
use iced::widget::button::secondary;
use iced::widget::container::danger;
use iced::widget::{column, container, row, Button, Column};
use iced::{Element, Length, Task};
use rand::prelude::*;

use crate::{
    components::{centered, loading},
    file_operations::{get_open_command, get_subdirectories, get_video_files},
};

fn play_random_video(directory: &Path) -> Task<Event> {
    let video_files = get_video_files(directory);
    if let Ok(random_video) =
        video_files.choose_weighted(&mut rand::rng(), crate::weights::weight_by_last_accessed)
    {
        let cmd = get_open_command();

        Task::perform(Command::new(cmd).arg(random_video).status(), |res| {
            Event::Complete(res.map(|_| ()).map_err(|e| e.kind().to_string()))
        })
    } else {
        Task::done(Event::Complete(
            Err("No video files found in folder".into()),
        ))
    }
}

fn open_folder(directory: &PathBuf) -> Task<Event> {
    let cmd = get_open_command();

    Task::perform(Command::new(cmd).arg(directory).status(), |res| {
        Event::Complete(res.map(|_| ()).map_err(|e| e.kind().to_string()))
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
    error: Option<String>,
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
            error: None,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
enum Event {
    PlayRandomVideo(usize),
    BrowseShow(usize),
    Complete(Result<(), String>),
    ClearError,
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
        Complete(res) => {
            app.loading = false;
            if let Err(error) = res {
                eprintln!("Error: {error}");
                app.error = Some(error);
                Task::perform(Timer::after(Duration::from_secs(1)), |_| ClearError)
            } else {
                Task::none()
            }
        },
        ClearError => {
            app.error = None;
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
                    .style(secondary)
                    .width(Length::Fill)
                    .on_press_maybe(if app.loading {
                        None
                    } else {
                        Some(Event::PlayRandomVideo(idx))
                    }),
                Button::new("📁")
                    .style(secondary)
                    .on_press_maybe(if app.loading {
                        None
                    } else {
                        Some(Event::BrowseShow(idx))
                    })
            ])
        })
        .collect::<Column<_>>();

    let mut root = column![list];

    if app.loading {
        root = root.push(loading());
    }
    if let Some(error) = &app.error {
        root = root.push(centered(
            container(error.as_str()).style(danger).padding(15),
        ));
    }
    root
}

fn main() -> iced::Result {
    iced::application(App::new, update, view)
        .title(option_env!("VID_LAUNCHER_TITLE").unwrap_or("vid-launcher-debug"))
        .window_size((300, 400))
        .run()
}
