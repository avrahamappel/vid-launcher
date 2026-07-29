use std::path::PathBuf;

use async_process::Command;
use iced::widget::image::Handle;

use crate::utils;

#[derive(Clone)]
pub struct Show {
    // TODO: some kind of id for shows
    pub name: String,
    pub path: PathBuf,
    pub thumbnail: Option<Handle>,
}

impl Show {
    pub fn new(path: PathBuf) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();

        Some(Self {
            name,
            path,
            thumbnail: None,
        })
    }
}

pub async fn load_thumbnail(path: PathBuf) -> Option<Handle> {
    let cache_dir = utils::home_dir().join(".cache");

    let thumbnail_basename = {
        let hash = md5::compute(path.to_str()?);
        format!("{hash:x}.png")
    };

    let global_thumbnail_path = cache_dir.join("thumbnails").join(&thumbnail_basename);

    if global_thumbnail_path.exists() {
        return Some(Handle::from_path(global_thumbnail_path));
    }

    let app_thumbnail_path = cache_dir
        .join("vid-launcher/thumbnails")
        .join(&thumbnail_basename);

    if app_thumbnail_path.exists() {
        return Some(Handle::from_path(app_thumbnail_path));
    }

    // FIXME: find newest file in folder

    generate_thumbnail(path, app_thumbnail_path);

    todo!()
}

async fn generate_thumbnail(input: &Path, output: &Path) -> () {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg("thumbnail")
        .arg("-frames:v")
        .arg("1")
        .arg(output)
        .spawn()?
        .wait()?;
}
