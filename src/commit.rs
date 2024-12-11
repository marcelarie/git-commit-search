use git2::{Commit, Repository};

// Simple of print commit with metadata
// pub fn print_commit(commit: &Commit) {
//     println!("commit {}", commit.id());
//     if let Some(author) = commit.author().name() {
//         println!("Author: {}", author);
//     }
//     if let Some(email) = commit.author().email() {
//         println!("Email: {}", email);
//     }
//     if let Some(message) = commit.message() {
//         println!("\n    {}\n", message.trim_end());
//     }
// }

/// Walk through all commits in the repository
pub fn walk_commits(repo: &Repository) -> Result<Vec<Commit>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    revwalk
        .filter_map(|oid| oid.ok())
        .map(|oid| repo.find_commit(oid))
        .collect()
}
