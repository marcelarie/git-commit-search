use colored::*;
use git2::Repository;
use regex::Regex;
use std::env;

fn print_commit(commit_id: &str) {
    // Use bold text (\x1b[1m), black text (\x1b[30m), and orange background (\x1b[48;2;255;165;0m)
    println!(
        "Commit: \x1b[1m\x1b[30m\x1b[48;2;255;165;0m {} \x1b[0m",
        commit_id
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <regex_pattern>", args[0]);
        std::process::exit(1);
    }
    let regex = Regex::new(&args[1])?;

    let repo = Repository::open(".")?;

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

const DARK_GREEN: (u8, u8, u8) = (0, 60, 0);
const DARK_RED: (u8, u8, u8) = (80, 0, 0);

enum ChangeType {
    Addition,
    Deletion,
}

impl ChangeType {
    fn format_line(&self, content: &str) -> String {

        match self {
            ChangeType::Addition => {
                let (r, g, b) = DARK_GREEN;

                format!("+{}", content)
                    .as_str()
                    .on_truecolor(r, g, b)
                    .truecolor(255, 255, 255) // White text
                    .bold()
                    .to_string()
            }
            ChangeType::Deletion => {
                let (r, g, b) = DARK_RED;

                format!("-{}", content)
                    .as_str()
                    .on_truecolor(r, g, b)
                    .truecolor(255, 255, 255) // White text
                    .bold()
                    .to_string()
            }
        }
    }
}

fn print_diff(diff: &git2::Diff, regex: &Regex) -> Result<(), git2::Error> {
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        let content = String::from_utf8_lossy(line.content());

        if regex.is_match(&content) {
            if let Some(change_type) = match line.origin() {
                '+' => Some(ChangeType::Addition),
                '-' => Some(ChangeType::Deletion),
                _ => None,
            } {
                println!("{}", change_type.format_line(&content));
            } else {
                // Default for unchanged or context lines
                println!("{}", content);
            }
        }
        true
    })?;
    Ok(())
}
