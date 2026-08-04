use std::path::{Path, PathBuf};
use std::process::Stdio;

use async_fs as fs;
use async_process::Command;
use iced::widget::image::Handle;

use crate::app::{TILE_HEIGHT, TILE_WIDTH};
use crate::file_operations::get_video_files;
use crate::utils;
use crate::weights::LastAccessible;

/// Compute an md5 hash of the path, plus the ".png" extension
fn compute_thumbnail_basename(path: &Path) -> Option<String> {
    let hash = md5::compute(path.to_str()?);
    Some(format!("{hash:x}.png"))
}

pub async fn load_thumbnail(mut path: PathBuf) -> Option<Handle> {
    let cache_dir = utils::home_dir().join(".cache");

    if path.is_dir() {
        // Use the most recently added video file within the directory

        let mut files_and_last_accessed = get_video_files(&path)
            .into_iter()
            .filter_map(|file| match file.last_accessed() {
                Ok(last_accessed) => Some((file, last_accessed)),
                Err(e) => {
                    eprintln!(
                        "Path [{}]: getting last accessed time failed with: {e:?}",
                        file.display()
                    );
                    None
                },
            })
            .collect::<Vec<_>>();

        files_and_last_accessed.sort_by_key(|(_, last_accessed)| *last_accessed);

        path = files_and_last_accessed.pop()?.0;
    }

    let thumbnail_basename = compute_thumbnail_basename(&path)?;

    let app_thumbnail_path = {
        let app_thumbnail_dir_path = cache_dir.join("vid-launcher/thumbnails");
        if !app_thumbnail_dir_path.exists() {
            fs::create_dir_all(&app_thumbnail_dir_path)
                .await
                .inspect_err(|e| {
                    eprintln!(
                        "mkdir -p '{}' failed with: {e:?}",
                        app_thumbnail_dir_path.display()
                    );
                })
                .ok()?;
        }
        app_thumbnail_dir_path.join(&thumbnail_basename)
    };

    if app_thumbnail_path.exists() {
        eprintln!(
            "Path [{}]: Found thumbnail in [{}]",
            path.display(),
            app_thumbnail_path.display()
        );
        return Some(Handle::from_path(app_thumbnail_path));
    }

    eprintln!(
        "Path [{}]: No thumbnail found, generating a new one",
        path.display()
    );

    generate_thumbnail(&path, &app_thumbnail_path)
        .await
        .inspect_err(|e| {
            eprintln!(
                "Thumbnail generation for [{}] failed with: {e:?}",
                path.display()
            );
        })
        .ok()?;

    eprintln!(
        "Path [{}]: Thumbnail stored at [{}]",
        path.display(),
        app_thumbnail_path.display()
    );

    Some(app_thumbnail_path.into())
}

async fn generate_thumbnail(input: &Path, output: &Path) -> Result<(), std::io::Error> {
    let mut cmd = Command::new("ffmpeg");

    cmd.arg("-i")
        .arg(input)
        // Explanation of ffmpeg filters:
        // 1. 20 seconds into the video, grab a thumbnail image from the next 200 frames
        // 2. Scale it down to TILE_WIDTH x TILE_HEIGHT, increasing one of the dimensions if necessary
        // 3. Crop the final image to the exact tile proportions
        .arg("-ss")
        .arg("00:00:20")
        .arg("-vf")
        .arg(format!(
            "thumbnail=n=200,scale={TILE_WIDTH}:{TILE_HEIGHT}:force_original_aspect_ratio=increase,crop={TILE_WIDTH}:{TILE_HEIGHT}"
        ))
        .arg("-frames:v")
        .arg("1")
        .arg("-q:v")
        .arg("2")
        .arg(output)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::null());

    eprintln!("Path [{}]: running [{cmd:?}]", input.display());

    let status = cmd.status().await?;

    if !status.success() {
        eprintln!(
            "Path [{}]: cmd exited with status {status}",
            input.display()
        );
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_thumbnail_basename_correctly() {
        assert_eq!(
            "599f0812d17899dd390d8d159a8ab16a.png",
            &compute_thumbnail_basename(Path::new("/home/user/Videos/VideoDir1/Video1.mp4"))
                .unwrap()
        );
    }
}
