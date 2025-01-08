use std::path::Path;

use colored::*;
use git2::Commit;

use crate::env::{current_dir, var};
use crate::regex_utils::RegexMatch;

pub fn print_commit(commit: &Commit, with_metadata: bool) {
    let commit_id = commit.id().to_string();
    print!("\ncommit: {}\n", commit_id.truecolor(255, 165, 0)); // Bright orange

    if with_metadata {
        if let Some(author) = commit.author().name() {
            println!("Author: {}", author);
        }
        if let Some(email) = commit.author().email() {
            println!("Email: {}", email);
        }
        if let Some(message) = commit.message() {
            println!("\n    {}\n", message.trim_end());
        }
    }
}

fn print_minimal_line(
    match_text: &str,
    path: &str,
    file_name: &str,
    line_number: &str,
    line_content: &str,
    change_type: &char,
) {
    let highlighted_match = match_text.red().to_string();
    let line_content = line_content.replace(match_text, &highlighted_match);

    let change_type = match change_type {
        '+' => &format!("{}:", &"+".bold().green()),
        '-' => &format!("{}:", &"-".bold().red()),
        _ => "",
    };

    let file = format!("{}{}", path, file_name);

    println!(
        "{}:{}:{} {}",
        file.truecolor(204, 204, 204), // Light grey
        line_number.truecolor(102, 153, 204), // Azure blue
        change_type,
        line_content.trim_end()
    );
}

pub fn print_minimal_match_result(match_result: RegexMatch, path: &Path) {
    let current_file = &match_result.file_name;
    let line_number = match_result.line_number;

    let line = &match_result.line_content;
    let match_text = &match_result.matched_text;
    let change_type = &match_result.line_change_type;

    // TODO: Refactor this code for path
    let simplified_path = if path.to_str().unwrap() == "." {
        "".to_string()
    } else {
        let absolute_path = path.canonicalize().unwrap();
        let current_dir = current_dir().unwrap();
        let _is_external_repo = absolute_path != current_dir;

        absolute_path
            .display()
            .to_string()
            .replace(&var("HOME").unwrap(), "~")
            + "/"
    };

    let line_number = line_number.unwrap_or(0);
    let line_number_str = if line_number > 0 {
        line_number.to_string()
    } else {
        "-".to_string()
    };
    print_minimal_line(
        match_text,
        &simplified_path,
        current_file,
        &line_number_str,
        line,
        change_type,
    )
}
