mod args;
mod commit;
mod diff;
mod utils;

use args::parse_args;
use commit::print_commit;
use diff::print_diff;
use git2::Repository;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex, path) = parse_args();

    // Compile the regex
    let regex = Regex::new(&regex)?;

    // Open the repository
    let repo = Repository::open(path)?;

    // Walk through commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        print_commit(&commit.id().to_string());
        let tree = commit.tree()?;

        if let Some(parent) = commit.parents().next() {
            let parent_tree = parent.tree()?;
            let diff =
                repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            print_diff(&diff, &regex)?;
        } else {
            let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
            print_diff(&diff, &regex)?;
        }
    }

    Ok(())
}
