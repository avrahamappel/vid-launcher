use std::path::PathBuf;

use iced::widget::image::Handle;

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
