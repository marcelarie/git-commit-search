use git2::{Commit, Repository};
use regex::Regex;
use std::path::Path;

use crate::args::has_show_metadata_mode;
use crate::git::{generate_patch, get_commit_diff, use_diff_tool};
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

// TODO: Unify commit printing between diff_tool and minimal modes
pub fn process_with_diff_tool(
    commits: Vec<Commit>,
    repo: &Repository,
    regex: &Regex,
    diff_tool: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut patches = Vec::new();

    for commit in commits {
        let diff = get_commit_diff(repo, &commit)?;

        if let (true, _) = matches_diff(&diff, regex) {
            let patch = generate_patch(&commit, &diff)?;
            patches.push(patch);
        }
    }

    if let Some(ref tool) = diff_tool {
        let all_patches = patches.join("\n");
        use_diff_tool(tool, &all_patches)?;
    }

    Ok(())
}

pub fn process_minimal_mode(
    commits: Vec<Commit>,
    repo: &Repository,
    regex: &Regex,
    repo_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for commit in commits {
        let diff = get_commit_diff(repo, &commit)?;
        let (has_matches, matches) = matches_diff(&diff, regex);

        if has_matches {
            let show_metadata = has_show_metadata_mode();
            print_commit(&commit, show_metadata);
            for match_result in matches {
                print_minimal_match_result(match_result, repo_path);
            }
        }
    }

    Ok(())
}
