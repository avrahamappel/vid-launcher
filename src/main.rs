use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};
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

fn play_random_video(directory: &Path) {
    let video_files = get_video_files(directory);
    if let Some(random_video) = video_files.choose(&mut rand::rng()) {
        let cmd = get_open_command();

        Command::new(cmd)
            .arg(random_video)
            .spawn()
            .expect("Failed to open video file.");
    }
}

fn open_folder(directory: &PathBuf) {
    let cmd = get_open_command();

    Command::new(cmd)
        .arg(directory)
        .spawn()
        .expect("Failed to open folder.");
}

fn main() -> glib::ExitCode {
    // Initialize GTK
    let application = Application::builder().build();

    application.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title(include_str!("./title.txt").trim())
            .default_width(300)
            .default_height(400)
            .build();

        // Create a vertical box to hold buttons
        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        window.set_child(Some(&vbox));

        // Get the user's Videos directory from the $HOME environment variable
        let home_dir = std::env::var("HOME").expect("Failed to get HOME environment variable");
        let videos_dir = PathBuf::from(home_dir).join("Videos");

        let directories = get_subdirectories(&videos_dir);

        for dir in directories {
            let dir_clone = dir.clone();
            let button = Button::with_label(dir.file_name().unwrap().to_str().unwrap());
            button.set_hexpand(true);
            button.connect_clicked(move |_| {
                play_random_video(&dir_clone);
            });

            // Create a smaller button for opening the folder
            let folder_button = Button::with_label("📁");
            folder_button.set_size_request(30, 30);
            let dir_clone = dir.clone();
            folder_button.connect_clicked(move |_| {
                open_folder(&dir_clone);
            });

            // Create a horizontal box to hold the main button and the folder button
            let hbox = Box::new(gtk::Orientation::Horizontal, 5);
            hbox.append(&button);
            hbox.append(&folder_button);

            vbox.append(&hbox);
        }

        // Show all widgets
        window.show();
    });

    // Start the GTK main loop
    application.run()
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
