use anyhow::{Context, Result};
use colored::*;
use git2::Repository;
use ignore::{
    gitignore::{Gitignore, GitignoreBuilder},
    WalkBuilder,
};
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn open_repository(path: &Path) -> Result<Repository> {
    Repository::open(path).with_context(|| {
        format!(
            "Could not open repository at '{}'",
            path.display().to_string().bold().yellow()
        )
    })
}

const GCS_IGNORE_FILE_PATH: &str = ".gcsignore";

pub struct GcsIgnoreMatcher {
    gcsignore: Option<Gitignore>,
}

impl GcsIgnoreMatcher {
    /// Initialize the matcher, loading all `.gcsignore` files in the repository.
    pub fn new(repo_path: &Path, no_ignore: bool) -> Result<Self> {
        if no_ignore {
            return Ok(Self { gcsignore: None });
        }

        let repo_path = Path::new(repo_path);
        let mut builder = GitignoreBuilder::new(repo_path);

        let regex_pattern = format!(r"(?i)\{}$", GCS_IGNORE_FILE_PATH);

        let gcsignore_files = get_matching_files(repo_path, &regex_pattern)?;

        for gcsignore_file in gcsignore_files {
            builder.add(gcsignore_file);
        }

        let gcsignore = builder.build().expect("Failed to build gcignore files");

        Ok(Self {
            gcsignore: Some(gcsignore),
        })
    }

    /// Check if a file is ignored by any `.gcsignore` files in the repository.
    pub fn is_file_ignored(&self, file_path: &str) -> bool {
        if let Some(gcsignore) = &self.gcsignore {
            let file_path = Path::new(file_path);
            let is_dir = file_path.is_dir();

            gcsignore
                .matched_path_or_any_parents(file_path, is_dir)
                .is_ignore()
        } else {
            false
        }
    }
}

/// Get a list of all files in the repository matching a regex.
pub fn get_matching_files(
    repo_path: &Path,
    pattern: &str,
) -> anyhow::Result<Vec<PathBuf>> {
    let regex = Regex::new(pattern)?;
    let mut matching_files = Vec::new();
    let walker = WalkBuilder::new(repo_path).hidden(false).build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.file_type().is_some_and(|ft| ft.is_file()) {
                    if let Some(path) = entry.path().to_str() {
                        if regex.is_match(path) {
                            matching_files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
            }
        }
    }
    Ok(matching_files)
}
