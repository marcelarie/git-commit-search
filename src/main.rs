mod args;
mod commit;
mod diff;
mod utils;

use args::parse_args;
use diff::print_diff;
use git2::Repository;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex, path) = parse_args();

    let regex = Regex::new(&regex)?;
    let repo = Repository::open(path)?;

    // Walk through commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_id = commit.id();

        let tree = commit.tree()?;

        if let Some(parent) = commit.parents().next() {
            let parent_tree = parent.tree()?;
            let diff =
                repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            print_diff(&diff, &regex, commit_id)?;
        } else {
            let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
            print_diff(&diff, &regex, commit_id)?;
        }
    }

    Ok(())
}
