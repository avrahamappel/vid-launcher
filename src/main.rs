use gtk::prelude::*;
use gtk::{Box, Button, Window, WindowType};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title(include_str!("./title").trim());
    window.set_default_size(300, 400);

    // Create a vertical box to hold buttons
    let vbox = Box::new(gtk::Orientation::Vertical, 5);
    window.add(&vbox);

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
            button.connect_clicked(move |_| {
                play_random_video(&dir_clone);
            });
            vbox.pack_start(&button, true, true, 0);
        }
    }

    // Show all widgets
    window.show_all();

    // Connect the window's close event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        // This indicates that the event should not be propagated further
        false.into()
    });

    // Start the GTK main loop
    gtk::main();
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
