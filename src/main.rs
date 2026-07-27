mod components;
mod file_operations;
mod weights;

use std::path::PathBuf;
use std::time::Duration;

use async_io::Timer;
use async_process::Command;
use iced::widget::button::secondary;
use iced::widget::container::{danger, rounded_box};
use iced::widget::image::Handle;
use iced::widget::{
    button, center, column, container, float, hover, image, mouse_area, row, Column, Row,
};
use iced::{Element, Task, Vector};
use itertools::Itertools;
use rand::prelude::*;

use crate::{
    components::{centered, loading},
    file_operations::{get_open_command, get_subdirectories, get_video_files},
};

#[derive(Clone)]
struct Show {
    // TODO: some kind of id for shows
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
    /// Which title to display in the bottom of the view
    display_title: Option<String>,
}

impl App {
    fn new() -> Self {
        Self {
            shows: vec![],
            loading: true,
            error: None,
            display_title: None,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
enum Event {
    ShowsLoaded(Vec<Show>),
    Entered(usize),
    Exited,
    PlayRandomVideo(usize),
    BrowseShow(usize),
    Complete(Result<(), String>),
    ClearError,
}

/// Load all shows
#[expect(clippy::unused_async)]
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
        Entered(idx) => {
            app.display_title = app.shows.get(idx).map(|s| s.name.clone());
            Task::none()
        },
        Exited => {
            app.display_title = None;
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

const TILES_PER_ROW: usize = 2;
const TILE_WIDTH: u32 = 150;
const TILE_HEIGHT: u32 = 100;
#[expect(clippy::cast_possible_truncation)]
const WINDOW_WIDTH: u32 = TILE_WIDTH * (TILES_PER_ROW as u32);

const IMG_BYTES: &[u8] = include_bytes!("../ferris.png").as_slice();

fn view(app: &App) -> Column<'_, Event> {
    let handle = Handle::from_bytes(IMG_BYTES);

    let list = app
        .shows
        .iter()
        .enumerate()
        .chunks(TILES_PER_ROW)
        .into_iter()
        .map(|chunk| {
            Element::from(
                chunk
                    .map(|(idx, show)| {
                        let tile = mouse_area(
                            button(image(handle.clone()))
                                .style(secondary)
                                .width(TILE_WIDTH)
                                .height(TILE_HEIGHT),
                        )
                        .on_enter(Event::Entered(idx))
                        .on_exit(Event::Exited);

                        let hover_view = center(row![
                            button(" ▶️").on_press_maybe(
                                (!app.loading).then_some(Event::PlayRandomVideo(idx))
                            ),
                            button("📁")
                                .on_press_maybe((!app.loading).then_some(Event::BrowseShow(idx))),
                        ]);

                        hover(tile, hover_view)
                    })
                    .collect::<Row<_>>(),
            )
        })
        .collect::<Column<_>>();

    let mut root = column![list];

    if let Some(title) = &app.display_title {
        root = root.push(
            float(container(title.as_str()).style(rounded_box)).translate(|container, viewport| {
                Vector::new(
                    (viewport.width - container.width) / 2.0 - container.x,
                    viewport.height - container.height - 10.0 - container.y,
                )
            }),
        );
    }

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
        .window_size((WINDOW_WIDTH, 400))
        .run()
}
