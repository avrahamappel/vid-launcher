use std::{path::PathBuf, time::SystemTime};

type Error = String;

pub trait LastAccessible {
    fn last_accessed(&self) -> Result<SystemTime, Error>;
}

impl LastAccessible for PathBuf {
    fn last_accessed(&self) -> Result<SystemTime, Error> {
        match self.metadata() {
            Err(_) => Err("Failed to retrieve metadata for file".into()),
            Ok(metadata) => match metadata.accessed() {
                Err(_) => Err("This system does not support file access times".into()),
                Ok(accessed) => Ok(accessed),
            },
        }
    }
}

fn seconds_since_system_time(time: SystemTime) -> Result<u64, String> {
    SystemTime::now()
        .duration_since(time)
        .map(|d| d.as_secs())
        .map_err(|e| {
            format!(
                "File was accessed {} seconds in the future",
                e.duration().as_secs_f32()
            )
        })
}

/// Returns the weight of a file as measured by its last accessed time
/// If last accessed time is unavailable, it returns 0
pub fn weight_by_last_accessed(file: &impl LastAccessible) -> u64 {
    match file.last_accessed().and_then(seconds_since_system_time) {
        Err(e) => {
            eprintln!("Error: {e}. Unable to add weight for file choosing");
            0
        },
        Ok(seconds) => seconds,
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    struct MockFile {
        last_accessed: Option<SystemTime>,
    }

    impl LastAccessible for MockFile {
        fn last_accessed(&self) -> Result<SystemTime, Error> {
            self.last_accessed.ok_or("No last accessed time".into())
        }
    }

    mod weight_by_last_accessed {
        use super::*;

        #[test]
        fn returns_higher_for_older_time() {
            let file1 = MockFile {
                last_accessed: Some(SystemTime::now() - Duration::from_secs(5)),
            };
            let file2 = MockFile {
                last_accessed: Some(SystemTime::now() - Duration::from_mins(10)),
            };

            assert_eq!(5, weight_by_last_accessed(&file1));
            assert_eq!(600, weight_by_last_accessed(&file2));
        }

        #[test]
        fn returns_zero_if_time_is_not_available() {
            let file = MockFile {
                last_accessed: None,
            };

            assert_eq!(0, weight_by_last_accessed(&file));
        }
    }
}
