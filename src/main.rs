mod weights;

use std::fs;
use std::path::{Path, PathBuf};

use async_process::Command;
use iced::widget::{row, Button, Column};
use iced::{Element, Length, Task};
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
}

impl App {
    fn new() -> Self {
        let home_dir = std::env::var("HOME").expect("Failed to get HOME environment variable");
        let videos_dir = PathBuf::from(home_dir).join("Videos");

        let directories = get_subdirectories(&videos_dir);
        let shows = directories.into_iter().filter_map(Show::new).collect();

        Self { shows }
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
            // TODO turn on loading indicator
            let show = &app.shows[idx];
            play_random_video(&show.path)
        },
        BrowseShow(idx) => {
            // TODO turn on loading indicator
            let show = &app.shows[idx];
            open_folder(&show.path)
        },
        Complete => {
            // TODO turn off loading indicator
            Task::none()
        },
        Error(e) => {
            todo!("display errors")
        },
    }
}

fn view(app: &App) -> Column<'_, Event> {
    app.shows
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
        .collect::<Column<_>>()
}

fn main() -> iced::Result {
    iced::application(App::new, update, view)
        // TODO title (via cargo metadata?)
        .window_size((300, 400))
        .run()
}

#[cfg(test)]
mod tests {
    use std::fs::{DirBuilder, File};

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_get_subdirectories_empty() {
        let dir = tempdir().unwrap();
        let subs = get_subdirectories(dir.path());
        assert_eq!(subs.len(), 0);
    }

    #[test]
    fn test_get_subdirectories_nonempty() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("subdir");
        fs::create_dir(&subdir_path).unwrap();
        let subs = get_subdirectories(dir.path());
        assert!(subs.contains(&subdir_path));
    }

    #[test]
    fn test_get_video_files_filtering() {
        let dir = tempdir().unwrap();
        let f1 = dir.path().join("a.mp4");
        let f2 = dir.path().join("b.mkv");
        let f3 = dir.path().join("c.txt");
        DirBuilder::new()
            .recursive(true)
            .create(dir.path().join("subdir"))
            .unwrap();
        let f4 = dir.path().join("subdir/d.webm");
        File::create(&f1).unwrap();
        File::create(&f2).unwrap();
        File::create(&f3).unwrap();
        File::create(&f4).unwrap();

        let videos = get_video_files(dir.path());
        assert!(videos.contains(&f1));
        assert!(videos.contains(&f2));
        assert!(!videos.contains(&f3));
        assert!(videos.contains(&f4));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_get_open_command() {
        let cmd = get_open_command();
        assert!(cmd == "open");
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_get_open_command() {
        let cmd = get_open_command();
        assert!(cmd == "xdg-open");
    }
}
