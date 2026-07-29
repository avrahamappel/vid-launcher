use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .expect("Failed to get HOME environment variable")
        .into()
}
