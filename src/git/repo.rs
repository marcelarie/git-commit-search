use anyhow::{Context, Result};
use colored::*;
use git2::Repository;
use std::path::Path;

pub fn open_repository(path: &Path) -> Result<Repository> {
    Repository::open(path).with_context(|| {
        format!(
            "Could not open repository at '{}'",
            path.display().to_string().bold().yellow()
        )
    })
}

pub fn find_commit_by_oid(
    repo: &Repository,
    oid: git2::Oid,
) -> Result<git2::Commit> {
    repo.find_commit(oid).with_context(|| {
        format!(
            "Could not find commit with OID '{}'",
            oid.to_string().bold().yellow()
        )
    })
}
