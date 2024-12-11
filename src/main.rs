mod args;

use args::parse_args;
use git2::Repository;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex, path, context_lines, no_gitignore, diff_tool) = parse_args();

    let regex = Regex::new(&regex)?;
    let repo = Repository::open(&path)?;

    // Walk through commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_id = commit.id();
        let tree = commit.tree()?;

        let diff = match commit.parents().next() {
            Some(parent) => {
                let parent_tree = parent.tree()?;
                repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?
            }
            None => repo.diff_tree_to_tree(None, Some(&tree), None)?,
        };

        // rest of the code
    }

    Ok(())
}
