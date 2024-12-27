use anyhow::{Context, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub file:         String,
    pub line_number:  u32,
    pub line_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub regex:   String,
    pub matches: Vec<(String, Vec<Match>)>, // (commit hash, matches)
}

const CACHE_FILE_NAME: &str = "gcs-cache.bin";

fn get_cache_file_path(repo_path: &Path) -> Result<PathBuf> {
    let repo = Repository::open(repo_path)?;
    let git_dir_path = repo.path();
    Ok(git_dir_path.join(CACHE_FILE_NAME))
}

pub fn initialize_cache(repo_path: &Path) -> Result<()> {
    let cache_path = get_cache_file_path(repo_path)?;

    if !cache_path.exists() {
        let empty_cache = Cache {
            regex:   String::new(),
            matches: Vec::new(),
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&cache_path)
            .with_context(|| {
                format!(
                    "Failed to create cache file at {}",
                    cache_path.display()
                )
            })?;

        bincode::serialize_into(&mut file, &empty_cache)
            .with_context(|| "Failed to serialize the empty cache")?;
    }

    Ok(())
}
