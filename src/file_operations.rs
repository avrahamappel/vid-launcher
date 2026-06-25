use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_subdirectories(path: &Path) -> Vec<PathBuf> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect(),
        Err(_) => vec![],
    }
}

pub fn get_video_files(directory: &Path) -> Vec<PathBuf> {
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

pub fn get_open_command() -> &'static str {
    if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    }
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
