use std::{collections::HashMap, ffi::OsStr, path::Path, path::PathBuf, sync::Arc};

use iced::{
    widget::{button, column, container, image, mouse_area, row, scrollable, space, text, Row},
    Alignment, Element, Length, Task,
};
use rand::prelude::*;

/// A show is a collection of channels; each show may have its own icon.
#[derive(Debug, Clone)]
struct Show {
    name: String,
    icon_path: Option<PathBuf>,
    channels: Vec<Channel>,
}

/// A channel is a single YouTube (or other) URL.
#[derive(Debug, Clone)]
struct Channel {
    name: String,
    url: String,
}

/// Represents a tile that appears on the main screen.
#[derive(Debug, Clone)]
struct Tile {
    /// The show that owns this tile.
    show: Arc<Show>,
    /// If a folder already exists on disk for this show, we keep its path.
    folder: Option<PathBuf>,
    /// Cached thumbnail (path on disk). `None` means we must fetch/generate it.
    thumb: Option<PathBuf>,
    /// UI state – are we currently hovered?
    hovered: bool,
    /// UI state – is a spinner active (download / play in progress)?
    loading: bool,
}

#[derive(Debug, Default)]
struct AppState {
    /// All known shows, keyed by a stable ID (e.g., name).
    shows: HashMap<String, Arc<Show>>,
    /// List of tiles that the UI renders (two‑column infinite list).
    tiles: Vec<Tile>,
    /// When a folder tile is opened we spawn a second window.
    /// The map holds the window id → Tiles for that folder.
    folder_windows: HashMap<u32, Vec<Tile>>,
}

#[derive(Debug, Clone)]
enum Message {
    /// Mouse entered/exited a tile.
    HoverTile { index: usize, hovered: bool },

    /// User pressed the Shuffle button on a tile.
    ClickShuffle { index: usize },

    /// User pressed the Folder button on a tile.
    ClickFolder { index: usize },

    /// A download finished (success or failure).
    DownloadFinished {
        index: usize,
        result: Result<PathBuf, String>, // path to video file or error string
    },

    /// Request to open a new window for a folder.
    OpenFolderWindow { index: usize },

    /// Message forwarded from a child window (e.g., play video inside that window).
    //ChildWindow(Message),

    /// Internal – spinner finished, re‑enable UI.
    LoadingFinished { index: usize },
}

/// -------------------------------------------------------------
/// Helper to load a thumbnail (placeholder for actual logic).
async fn load_thumbnail(_show: &Show) -> Option<PathBuf> {
    // In a real program you would either:
    //   • read the show.icon_path,
    //   • or run yt-dlp `--skip-download --write-thumbnail` on one video.
    None
}
/// Helper to pick a random video file in a folder.
fn random_video_in_folder(folder: &Path) -> Option<PathBuf> {
    std::fs::read_dir(folder)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| matches!(ext, "mp4" | "webm" | "mkv"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>()
        .choose(&mut rand::rng())
        .map(|e| e.path())
}

/// Helper to invoke yt‑dlp for a single video.
async fn yt_dlp_download_one(url: &str, dest_dir: &Path) -> Result<PathBuf, String> {
    todo!()
    //let mut cmd = TokioCommand::new("yt-dlp");
    //cmd.arg("-o")
    //    .arg(format!("{}/%(title)s.%(ext)s", dest_dir.display()))
    //    .arg("-f")
    //    .arg("best[ext=mp4]/best")
    //    .arg(url);
    //// strip channel/show names later (see post‑process step)
    //let out = cmd.output().await.map_err(|e| e.to_string())?;
    //if !out.status.success() {
    //    return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    //}
    //// yt‑dlp writes the file; we locate it by scanning the folder.
    //let file = std::fs::read_dir(dest_dir)
    //    .ok()
    //    .and_then(|mut it| it.find_map(|e| e.ok()))
    //    .ok_or_else(|| "download succeeded but file not found".to_string())?;
    //Ok(file.path())
}

/// Helper to launch mpv (or any player) with a given file.
fn launch_player(path: &Path) {
    //let _ = std::process::Command::new("mpv").arg(path).spawn(); // fire‑and‑forget
}

/// -------------------------------------------------------------
/// Application implementation
struct VidLauncher {
    state: AppState,
}
impl VidLauncher {
    fn new() -> (Self, Task<Message>) {
        // ----- Stub data -------------------------------------------------
        let demo_show = Arc::new(Show {
            name: "Demo Show".into(),
            icon_path: None,
            channels: vec![Channel {
                name: "Demo Channel".into(),
                url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".into(),
            }],
        });
        let tiles = vec![
            Tile {
                show: demo_show.clone(),
                folder: None, // no folder yet
                thumb: None,
                hovered: false,
                loading: false,
            },
            Tile {
                show: demo_show,
                folder: None,
                thumb: None,
                hovered: false,
                loading: false,
            },
        ];
        // -----------------------------------------------------------------
        (
            Self {
                state: AppState {
                    shows: HashMap::new(),
                    tiles,
                    folder_windows: HashMap::new(),
                },
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HoverTile { index, hovered } => {
                //if let Some(t) = self.state.tiles.get_mut(index) {
                //    t.hovered = hovered;
                //}
                Task::none()
            },

            Message::ClickShuffle { index } => {
                //if let Some(tile) = self.state.tiles.get_mut(index) {
                //    tile.loading = true;
                //}
                //// Decide which async action to run
                //let tile = self.state.tiles[index].clone();
                //Task::perform(
                //    async move {
                //        if let Some(folder) = &tile.folder {
                //            // Folder exists → pick random video
                //            if let Some(v) = random_video_in_folder(folder) {
                //                Ok(v)
                //            } else {
                //                Err("no video files in folder".into())
                //            }
                //        } else {
                //            // No folder → download first video with yt‑dlp
                //            // Use a deterministic sub‑dir under ./downloads/<show_name>
                //            let dest = PathBuf::from("./downloads").join(&tile.show.name);
                //            std::fs::create_dir_all(&dest).ok();
                //            // Grab the first channel URL
                //            let url = tile
                //                .show
                //                .channels
                //                .first()
                //                .map(|c| c.url.clone())
                //                .ok_or_else(|| "no channel URL".to_string())?;
                //            yt_dlp_download_one(&url, &dest).await
                //        }
                //    },
                //    move |res| Message::DownloadFinished { index, result: res },
                //)
                Task::none()
            },

            Message::DownloadFinished { index, result } => {
                //// Turn spinner off regardless of success/failure
                //if let Some(t) = self.state.tiles.get_mut(index) {
                //    t.loading = false;
                //}
                //match result {
                //    Ok(path) => {
                //        // Successful → play video
                //        launch_player(&path);
                //    },
                //    Err(err) => {
                //        eprintln!("shuffle error: {}", err);
                //        // In a full UI you would surface the error to the user.
                //    },
                //}
                Task::none()
            },

            Message::ClickFolder { index } => {
                Task::none()
                //// Turn spinner on while we inspect/create the folder
                //if let Some(t) = self.state.tiles.get_mut(index) {
                //    t.loading = true;
                //}
                //let tile = self.state.tiles[index].clone();
                //Task::perform(
                //    async move {
                //        // Resolve folder existence (or create one)
                //        let folder = match &tile.folder {
                //            Some(p) => p.clone(),
                //            None => {
                //                // First time → create a folder under ./library/<show_name>
                //                let p = PathBuf::from("./library").join(&tile.show.name);
                //                std::fs::create_dir_all(&p).ok();
                //                p
                //            },
                //        };
                //        // Populate folder with thumbnails / video list (simplified)
                //        // Real implementation would run `yt-dlp --skip-download --write-thumbnail`
                //        // for each channel and store the images.
                //        Ok(folder)
                //    },
                //    move |folder_res| match folder_res {
                //        Ok(path) => Message::OpenFolderWindow { index },
                //        Err(e) => {
                //            eprintln!("folder error: {}", e);
                //            Message::LoadingFinished { index }
                //        },
                //    },
                //)
            },

            Message::OpenFolderWindow { index } => {
                //    if let Some(tile) = self.state.tiles.get_mut(index) {
                //        tile.loading = false;
                //    }
                //    // Build a new window that shows the folder's contents.
                //    // Iced 0.12 supports multiple windows via `Program::new_window`.
                //    let new_tiles = build_folder_tiles(&self.state.tiles[index]);
                //    let window_id = iced::window::new(
                //        format!("{} – {}", self.state.tiles[index].show.name, "Channel view"),
                //        folder_view(new_tiles.clone()),
                //    );
                //    // Store tiles for later (optional, if you want to route messages back)
                //    self.state.folder_windows.insert(window_id, new_tiles);
                Task::none()
            },

            //Message::ChildWindow(msg) => {
            //    // Forward messages that originated from a child window.
            //    self.update(*msg)
            //},
            Message::LoadingFinished { index } => {
                //if let Some(t) = self.state.tiles.get_mut(index) {
                //    t.loading = false;
                //}
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        // ----- Build two‑column infinite scroll -------------------------
        let mut col_left = column![];
        let mut col_right = column![];

        for (i, tile) in self.state.tiles.iter().enumerate() {
            let tile_elem = tile_view(i, tile);
            if i % 2 == 0 {
                col_left = col_left.push(tile_elem);
            } else {
                col_right = col_right.push(tile_elem);
            }
        }

        let content = row![
            scrollable(col_left).height(Length::Fill),
            scrollable(col_right).height(Length::Fill),
        ]
        .spacing(20)
        .padding(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            //.center_x()
            //.center_y()
            .into()
    }
}

/// ---------------------------------------------------------------------
/// Build a single tile UI element
fn tile_view<'t, 'e>(index: usize, tile: &'t Tile) -> Element<'e, Message> {
    // Background container that reports hover events
    mouse_area(
        container(
            column![
                // Thumbnail / icon
                if let Some(ref thumb) = tile.thumb {
                    Element::from(
                        image(thumb.clone())
                            .width(Length::Fill)
                            .height(Length::FillPortion(8)),
                    )
                } else {
                    // placeholder box
                    Element::from(
                        container(text("No thumb"))
                            .width(Length::Fill)
                            .height(Length::FillPortion(8)),
                    )
                },
                // Hover overlay (buttons + spinner)
                if tile.hovered {
                    let mut btns: Row<_> = row![
                        button("Shuffle")
                            .on_press(Message::ClickShuffle { index })
                            .padding(5),
                        button("Folder")
                            .on_press(Message::ClickFolder { index })
                            .padding(5),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center);

                    if tile.loading {
                        //btns = btns.push(spinner::default().size(20));
                    }

                    btns.into()
                } else {
                    // empty space when not hovered
                    Element::from(
                        container(space().width(Length::Fill)).height(Length::FillPortion(2)),
                    )
                }
            ]
            .spacing(5),
        )
        .width(Length::FillPortion(1))
        .height(Length::FillPortion(1))
        .padding(5),
    )
    // Capture hover events
    .on_enter(Message::HoverTile {
        index,
        hovered: true,
    })
    .on_exit(Message::HoverTile {
        index,
        hovered: false,
    })
    .into()
}

/// ---------------------------------------------------------------------
/// Build tiles for a folder view (4‑column grid)
fn build_folder_tiles(parent_tile: &Tile) -> Vec<Tile> {
    let folder = parent_tile
        .folder
        .clone()
        .unwrap_or_else(|| PathBuf::from("./library").join(&parent_tile.show.name));
    // Scan folder for video files → each becomes a tile.
    let mut tiles = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|e| matches!(e, "mp4" | "webm" | "mkv"))
                    .unwrap_or(false)
            {
                tiles.push(Tile {
                    show: parent_tile.show.clone(),
                    folder: Some(folder.clone()),
                    thumb: Some(path.clone()), // In reality you would have a separate thumbnail file.
                    hovered: false,
                    loading: false,
                });
            }
        }
    }
    tiles
}

/// ---------------------------------------------------------------------
/// Child‑window view (4‑column infinite scroll)
fn folder_view(tiles: Vec<Tile>) -> iced::Element<'static, Message> {
    let mut cols = [vec![], vec![], vec![], vec![]];
    for (i, tile) in tiles.iter().enumerate() {
        let view = tile_view(i, tile);
        cols[i % 4].push(view);
    }
    let row_content = row(cols.into_iter().map(|tiles| Element::from(column(tiles))))
        .spacing(10)
        .padding(10);
    container(scrollable(row_content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application(VidLauncher::new, VidLauncher::update, VidLauncher::view)
        .title("Random Vid Launcher")
        .run()
}
