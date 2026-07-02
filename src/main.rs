mod components;
mod file_operations;
mod weights;

use std::path::PathBuf;
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

#[derive(Clone)]
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
        Self {
            shows: vec![],
            loading: true,
            error: None,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
enum Event {
    ShowsLoaded(Vec<Show>),
    PlayRandomVideo(usize),
    BrowseShow(usize),
    Complete(Result<(), String>),
    ClearError,
}

/// Load all shows
async fn get_shows() -> Vec<Show> {
    // TODO load preconfigured shows

    // Load all directories in Videos
    let home_dir = std::env::var("HOME").expect("Failed to get HOME environment variable");
    let videos_dir = PathBuf::from(home_dir).join("Videos");

    let directories = get_subdirectories(&videos_dir);

    directories.into_iter().filter_map(Show::new).collect()
}

/// Play a randomly selected video in the given directory,
/// preferring less recently accessed files
async fn play_random_video(directory: PathBuf) -> Result<(), String> {
    let video_files = get_video_files(&directory);

    // Call the rng within a block so it's dropped before we need to switch threads
    let try_random_video =
        { video_files.choose_weighted(&mut rand::rng(), crate::weights::weight_by_last_accessed) };

    if let Ok(random_video) = try_random_video {
        let cmd = get_open_command();

        Command::new(cmd)
            .arg(random_video)
            .status()
            .await
            .map_err(|e| e.kind().to_string())?;
        Ok(())
    } else {
        Err("No video files found in folder".into())
    }
}

async fn open_folder(directory: PathBuf) -> Result<(), String> {
    let cmd = get_open_command();

    Command::new(cmd)
        .arg(directory)
        .status()
        .await
        .map_err(|e| e.kind().to_string())?;

    Ok(())
}

fn init() -> (App, Task<Event>) {
    (App::new(), Task::perform(get_shows(), Event::ShowsLoaded))
}

fn update(app: &mut App, event: Event) -> Task<Event> {
    use Event::*;
    match event {
        ShowsLoaded(shows) => {
            app.shows = shows;
            app.loading = false;
            Task::none()
        },
        PlayRandomVideo(idx) => {
            app.loading = true;
            let show_path = app.shows[idx].path.clone();
            Task::perform(play_random_video(show_path), Event::Complete)
        },
        BrowseShow(idx) => {
            app.loading = true;
            let show_path = app.shows[idx].path.clone();
            Task::perform(open_folder(show_path), Event::Complete)
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
    iced::application(init, update, view)
        .title(option_env!("VID_LAUNCHER_TITLE").unwrap_or("vid-launcher-debug"))
        .window_size((300, 400))
        .run()
}
