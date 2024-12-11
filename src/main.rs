mod args;
mod commit;
mod git;
mod regex_utils;

use args::parse_args;
use commit::{print_commit, walk_commits};
use git::{generate_patch, get_commit_diff};
use git2::Repository;
use regex_utils::matches_diff;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex_pattern, path, _context_lines, _no_gitignore, _diff_tool) =
        parse_args();
    let regex = Regex::new(&regex_pattern)?;
    let repo = Repository::open(&path)?;

    let commits = walk_commits(&repo)?;

    for commit in commits {
        let diff = get_commit_diff(&repo, &commit)?;
        let (matches, _) = matches_diff(&diff, &regex);

        if matches {
            print_commit(&commit);
            let patch = generate_patch(&diff)?;
            println!("{}", patch);
        }
    }

    Ok(())
}
