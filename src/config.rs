use std::path::{Path, PathBuf};

struct Url(String);

type PlaylistIndex = usize;

struct PlaylistObject {
    url: Url,
    name: Option<String>,
    skip: Option<Vec<PlaylistIndex>>,
}

enum Playlist {
    Url(Url),
    Object(PlaylistObject),
}

struct Show {
    name: String,
    playlists: Vec<Playlist>,
}

struct Config {
    shows: Vec<Show>,
}

impl Config {
    fn new() -> Result<Self, std::io::Error> {
        const CONFIG_FILE_PATH: &Path = ".config/vid-launcher/config.yml".into();

        let mut config_path = PathBuf::from(std::env::var("HOME")?);
        config_path.push(CONFIG_FILE_PATH);

        todo!()
    }
}
