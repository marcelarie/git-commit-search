use anyhow::Result;
use git2::{Commit, Repository};
use rayon::prelude::*;
use regex::Regex;
use std::error::Error;
use std::path::Path;

use crate::args::has_show_metadata_mode;
use crate::git::repo::find_commit_by_oid;
use crate::git::{
    generate_patch, get_commit_diff, open_repository, use_diff_tool,
};
use crate::print::{print_commit, print_minimal_match_result};
use crate::regex_utils::matches_diff;

/// Walk through all commits in the repository
pub fn walk_commits(repo: &Repository) -> Result<Vec<Commit>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    revwalk
        .filter_map(|oid| oid.ok())
        .map(|oid| repo.find_commit(oid))
        .collect()
}

pub fn process_with_diff_tool(
    commits: Vec<Commit>,
    repo_path: &Path,
    regex: &Regex,
    diff_tool: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Extract OIDs for parallel processing
    let commit_ids: Vec<_> = commits.iter().map(|commit| commit.id()).collect();

    let patches: Vec<_> =
        commit_ids
        .par_iter()
        .filter_map(|oid| {
            let repo = open_repository(repo_path).ok()?;

            let commit = find_commit_by_oid(&repo, *oid).ok()?;
            let diff = get_commit_diff(&repo, &commit).ok()?;
            if matches_diff(&diff, regex).0 {
                generate_patch(&commit, &diff).ok()
            } else {
                None
            }
        })
        .collect();

    if let Some(ref tool) = diff_tool {
        let all_patches = patches.join("\n");
        use_diff_tool(tool, &all_patches)?;
    }

    Ok(())
}

pub fn process_minimal_mode(
    commits: Vec<Commit>,
    repo_path: &Path,
    regex: &Regex,
) -> Result<()> {
    let commit_ids: Vec<_> = commits.iter().map(|commit| commit.id()).collect();

    commit_ids.par_iter().try_for_each(|&oid| -> Result<()> {
        let repo = open_repository(repo_path)?;
        let commit = find_commit_by_oid(&repo, oid)?;

        let diff = get_commit_diff(&repo, &commit)?;
        let (has_matches, matches) = matches_diff(&diff, regex);

        if has_matches {
            let show_metadata = has_show_metadata_mode();
            print_commit(&commit, show_metadata);

            for match_result in matches {
                print_minimal_match_result(match_result, repo_path);
            }
        }

        Ok(())
    })?;

    Ok(())
}
