use anyhow::{Context, Result};
use colored::*;
use git2::Repository;
use ignore::gitignore::GitignoreBuilder;
use std::path::Path;

pub fn open_repository(path: &Path) -> Result<Repository> {
    Repository::open(path).with_context(|| {
        format!(
            "Could not open repository at '{}'",
            path.display().to_string().bold().yellow()
        )
    })
}

const GIT_IGNORE_FILE_PATH: &str = ".gitignore";

pub fn is_file_ignored(repo_path: &str, file_path: &str) -> bool {
    let mut builder = GitignoreBuilder::new(repo_path);
    builder.add(GIT_IGNORE_FILE_PATH);
    let gitignore = builder.build().expect("Failed to build gitignore");

    let file_path = Path::new(file_path);
    let is_dir = file_path.is_dir();

    let matched = gitignore.matched_path_or_any_parents(file_path, is_dir);

    matched.is_ignore()
}
