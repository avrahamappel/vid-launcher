use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() -> glib::ExitCode {
    // Initialize GTK
    let application = Application::builder().build();

    application.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title(include_str!("./title").trim())
            .default_width(300)
            .default_height(400)
            .build();

        // Create a vertical box to hold buttons
        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        window.set_child(Some(&vbox));

        // Get the user's Videos directory from the $HOME environment variable
        let home_dir = std::env::var("HOME").expect("Failed to get HOME environment variable");
        let videos_dir = PathBuf::from(home_dir).join("Videos");

        // Read subdirectories in the Videos directory
        if let Ok(entries) = fs::read_dir(videos_dir) {
            let mut directories: Vec<PathBuf> = Vec::new();

            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    directories.push(path);
                }
            }

            // Create buttons for each directory
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
        }

        // Show all widgets
        window.show();
    });

    // Start the GTK main loop
    application.run()
}

fn play_random_video(directory: &PathBuf) {
    // Read video files in the directory
    if let Ok(entries) = fs::read_dir(directory) {
        let video_files: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            // TODO filter non-video files
            //.filter(|entry| {
            //    let path = entry.path();
            //    path.is_file() && path.extension().map_or(false, |ext| {
            //        ext == "mp4" || ext == "mkv" || ext == "avi" // Add more extensions as needed
            //    })
            //})
            .map(|entry| entry.path())
            .collect();

        // Play a random video file if available
        if let Some(random_video) = video_files.choose(&mut rand::thread_rng()) {
            let cmd = if cfg!(target_os = "macos") {
                "open"
            } else {
                "xdg-open"
            };

            Command::new(cmd)
                .arg(random_video)
                .spawn()
                .expect("Failed to open video file.");
        }
    }
}

fn open_folder(directory: &PathBuf) {
    let cmd = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };

    Command::new(cmd)
        .arg(directory)
        .spawn()
        .expect("Failed to open folder.");
}
