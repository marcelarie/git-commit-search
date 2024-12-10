use crate::{commit::print_commit, utils::ChangeType};
use git2::{Diff, Oid};
use regex::Regex;

pub fn print_commit_content(
    diff: &Diff,
    regex: &Regex,
    commit_id: Oid,
    context_lines: usize,
) -> Result<(), git2::Error> {
    let mut lines_buffer = Vec::new();
    let mut post_match_buffer = 0; // Counter for lines after a match

    diff.print(git2::DiffFormat::Patch, |_, _, line| {
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
                print_commit(&commit_id.to_string());

                // Print context lines before the match
                for buffered_line in &lines_buffer {
                    println!(" {}", buffered_line);
                }

                // Prints the match
                println!("{}", change_type.format_line(&content));

                // Prepare to print context lines after the match
                post_match_buffer = context_lines;

                lines_buffer.clear()
            }
        } else {
            // Add the current line to the buffer and limit the buffer size
            if lines_buffer.len() >= context_lines {
                lines_buffer.remove(0); // Remove the oldest line to maintain size
            }

            lines_buffer.push(content.to_string());
        }

        true // Continue processing lines
    })?;
    Ok(())
}
