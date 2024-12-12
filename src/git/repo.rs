use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path;

pub fn open_repository(path: &Path) -> Result<Repository> {
    Repository::open(path).with_context(|| {
        format!("Could not open repository at '{}'", path.display())
    })
}
