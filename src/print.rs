use colored::*;
use git2::Commit;

use crate::regex_utils::RegexMatch;

pub fn print_commit(commit: &Commit) {
    let commit_id = commit.id().to_string();
    print!("\ncommit: {}\n", commit_id.truecolor(255, 165, 0)); // Bright orange
}

fn print_minimal_line(
    match_text: &str,
    file_path: &str,
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

    println!(
        "{}:{}:{} {}",
        file_path.truecolor(204, 204, 204), // Light grey
        line_number.truecolor(102, 153, 204), // Azure blue
        change_type,
        line_content.trim_end()
    );
}

pub fn print_minimal_match_result(match_result: RegexMatch) {
    let current_file = &match_result.file_name;
    let line_number = match_result.line_number;

    let line = &match_result.line_content;
    let match_text = &match_result.matched_text;
    let change_type = &match_result.line_change_type;

    let line_number = line_number.unwrap_or(0);
    let line_number_str = if line_number > 0 {
        line_number.to_string()
    } else {
        "-".to_string()
    };
    print_minimal_line(
        match_text,
        current_file,
        &line_number_str,
        line,
        change_type,
    )
}
