mod args;
mod commit;
mod git;
mod print;
mod regex_utils;

use std::{env, path::Path};

use args::parse_args;
use commit::walk_commits;
use git::{generate_patch, get_commit_diff, use_diff_tool};
use git2::Repository;
use print::{print_commit, print_minimal_match_result};
use regex_utils::matches_diff;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex_pattern, path, _context_lines, _no_gitignore, diff_tool) =
        parse_args();

    let regex = regex::Regex::new(&regex_pattern)?;
    let path = Path::new(&path);
    let repo = Repository::open(path)?;

    let mut patches = Vec::new();
    let has_diff_tool = diff_tool.is_some();

    let commits = walk_commits(&repo)?;

    for commit in commits {
        let diff = get_commit_diff(&repo, &commit)?;
        let (has_matches, matches) = matches_diff(&diff, &regex);

        if has_diff_tool {
            if has_matches {
                let patch = generate_patch(&commit, &diff)?;
                patches.push(patch.clone());
            }
        } else {
            // Minimal mode
            if has_matches {
                print_commit(&commit);
                for match_result in matches {
                    print_minimal_match_result(match_result, path);
                }
            }
        }
    }

    // Show diff mode
    if let Some(ref tool) = diff_tool {
        let all_patches = patches.join("\n");
        use_diff_tool(tool, &all_patches)?;
    }

    Ok(())
}
