use colored::*;
use git2::Commit;

const ORANGE_BACKGROUND: (u8, u8, u8) = (255, 165, 0);

pub fn print_commit(commit: &Commit) {
    let commit_id = commit.id().to_string();
    let (r, g, b) = ORANGE_BACKGROUND;
    print!("commit: {}\n\n", commit_id.black().bold().on_truecolor(r, g, b));
}

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
//
