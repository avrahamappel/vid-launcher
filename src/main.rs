#![allow(clippy::enum_glob_use)]

mod components;
mod file_operations;
mod shows;
mod utils;
mod weights;

use std::path::PathBuf;
use std::time::Duration;

use async_io::Timer;
use async_process::Command;
use iced::widget::button::secondary;
use iced::widget::container::{danger, rounded_box};
use iced::widget::image::Handle;
use iced::widget::{
    Column, Row, button, center, column, container, float, hover, image, mouse_area, row,
};
use iced::{Task, Vector};
use rand::prelude::*;

use crate::shows::Show;
use crate::{
    components::{centered, loading},
    file_operations::{get_open_command, get_subdirectories, get_video_files},
};

const IMG_BYTES: &[u8] = include_bytes!("../assets/video.png").as_slice();

struct App {
    shows: Vec<Show>,
    loading: bool,
    error: Option<String>,
    /// Which title to display in the bottom of the view
    display_title: Option<String>,
    /// The default image to display for a show with no thumbnail
    default_image: Handle,
}

impl App {
    fn new() -> Self {
        let default_image = Handle::from_bytes(IMG_BYTES);

        Self {
            shows: vec![],
            loading: true,
            error: None,
            display_title: None,
            default_image,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
enum Event {
    ShowsLoaded(Vec<Show>),
    ThumbnailLoaded(usize, Option<Handle>),
    EnteredTile(usize),
    ExitedTile(usize),
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
    let videos_dir = utils::home_dir().join("Videos");

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
            Task::batch(app.shows.iter().enumerate().map(|(idx, show)| {
                Task::perform(shows::load_thumbnail(show.path.clone()), move |thumbnail| {
                    ThumbnailLoaded(idx, thumbnail)
                })
            }))
        },
        ThumbnailLoaded(idx, thumbnail) => {
            if let Some(show) = app.shows.get_mut(idx) {
                show.thumbnail = thumbnail;
            }
            Task::none()
        },
        EnteredTile(idx) => {
            app.display_title = app.shows.get(idx).map(|s| s.name.clone());
            Task::none()
        },
        ExitedTile(idx) => {
            // Clear the displayed title, if it's the one we're exiting
            // (i.e., if the enter event has already fired
            // on another tile, do nothing)
            if let Some(show) = app.shows.get(idx)
                && let Some(ref title) = app.display_title
                && &show.name == title
            {
                app.display_title = None;
            }
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

fn view(app: &App) -> Column<'_, Event> {
    let list = app
        .shows
        .iter()
        .enumerate()
        .map(|(idx, show)| {
            let handle = show
                .thumbnail
                .clone()
                .unwrap_or_else(|| app.default_image.clone());
            let tile = mouse_area(image(handle).width(TILE_WIDTH).height(TILE_HEIGHT))
                .on_enter(Event::EnteredTile(idx))
                .on_exit(Event::ExitedTile(idx));

            let hover_view = center(row![
                button(" ▶️")
                    .style(secondary)
                    .on_press_maybe((!app.loading).then_some(Event::PlayRandomVideo(idx))),
                button("📁")
                    .style(secondary)
                    .on_press_maybe((!app.loading).then_some(Event::BrowseShow(idx))),
            ]);

            hover(tile, hover_view)
        })
        .collect::<Row<_>>()
        .wrap();

    let mut root = column![list];

    if let Some(title) = &app.display_title {
        root = root.push(
            float(container(title.as_str()).style(rounded_box).padding(10)).translate(
                |container, viewport| {
                    Vector::new(
                        (viewport.width - container.width) / 2.0 - container.x,
                        viewport.height - container.height - 10.0 - container.y,
                    )
                },
            ),
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
