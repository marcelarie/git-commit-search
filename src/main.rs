mod args;
mod commit;
mod git;

use args::parse_args;
use commit::print_commit;
use git::{diff_matches_regex, generate_patch, get_commit_diff, walk_commits};
use git2::Repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex_pattern, path, _context_lines, _no_gitignore, _diff_tool) =
        parse_args();
    let regex = regex::Regex::new(&regex_pattern)?;
    let repo = Repository::open(&path)?;

    let commits = walk_commits(&repo)?;

    for commit in commits {
        let diff = get_commit_diff(&repo, &commit)?;

        if diff_matches_regex(&diff, &regex).0 {
            print_commit(&commit);
            let patch = generate_patch(&diff)?;
            println!("{}", patch);
        }
    }

    Ok(())
}
