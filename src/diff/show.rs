use std::path::Path;

use crate::{commit::print_commit, utils::ChangeType};
use colored::*;
use git2::{Diff, Oid};
use ignore::gitignore::GitignoreBuilder;
use regex::Regex;

const GIT_IGNORE_FILE_PATH: &str = ".gitignore";

fn is_ignored(repo_path: &str, file_path: &str) -> bool {
    let mut builder = GitignoreBuilder::new(repo_path);
    builder.add(GIT_IGNORE_FILE_PATH);
    let gitignore = builder.build().expect("Failed to build gitignore");

    let path = Path::new(file_path);
    let is_dir = path.is_dir();

    let matched = gitignore.matched(path, is_dir);
    matched.is_ignore()
}

pub fn print_commit_content(
    diff: &Diff,
    regex: &Regex,
    commit_id: Oid,
    context_lines: usize,
    repo_path: &str,
    no_gitignore: bool,
) -> Result<(), git2::Error> {
    let mut lines_buffer = Vec::new();
    let mut post_match_buffer = 0; // Counter for lines after a match
    let mut printed_commit = false; // Tracks if the commit has been printed


    diff.print(git2::DiffFormat::Patch, |delta, _, line| {
        let content = String::from_utf8_lossy(line.content());

        if post_match_buffer > 0 {
            // Print context lines after a match
            println!(" {}", content);
            post_match_buffer -= 1;
            return true;
        }

        if regex.is_match(&content) {
            if let Some(change_type) = match line.origin() {
                '+' => Some(ChangeType::Addition),
                '-' => Some(ChangeType::Deletion),
                _ => None,
            } {
                if let Some(path) = delta.new_file().path() {
                    let file_path = path.display();
                    let should_ignore_file =
                        is_ignored(repo_path, &file_path.to_string());

                    if !no_gitignore && should_ignore_file {
                        return true;
                    }

                    if !printed_commit {
                        print_commit(&commit_id.to_string());
                        printed_commit = true;
                    }

                    let pretty_file_path =
                        format!("{}", file_path).bold().to_string();
                    if let Some(line_number) = line.new_lineno() {
                        let line_number =
                            format!("{}", line_number).bold().red().to_string();

                        println!("path: {}:{}:", pretty_file_path, line_number);
                    } else {
                        println!("path: {}:", pretty_file_path);
                    }
                }

                // Print context lines before the match
                if !lines_buffer.is_empty() && context_lines > 0 {
                    for buffered_line in &lines_buffer {
                        println!(" {}", buffered_line);
                    }
                }

                // Prints the match
                println!("{}", change_type.format_line(&content));

                // Prepare to print context lines after the match
                post_match_buffer = context_lines;

                lines_buffer.clear()
            }
        } else {
            // Add the current line to the buffer and limit the buffer size
            if !lines_buffer.is_empty() && lines_buffer.len() >= context_lines {
                lines_buffer.remove(0); // Remove the oldest line to maintain size
            }

            lines_buffer.push(content.to_string());
        }

        true // Continue processing lines
    })?;
    Ok(())
}
