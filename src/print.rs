use colored::*;
use git2::Commit;

use crate::regex_utils::RegexMatch;

const ORANGE_BACKGROUND: (u8, u8, u8) = (255, 165, 0);

pub fn print_commit(commit: &Commit) {
    let commit_id = commit.id().to_string();
    let (r, g, b) = ORANGE_BACKGROUND;
    print!(
        "\ncommit: {}\n\n",
        commit_id.black().bold().truecolor(r, g, b)
    );
}

fn print_minimal_line(
    match_text: &str,
    file_path: &str,
    line_number: &str,
    line_content: &str,
) {
    let highlighted_match = match_text.red().bold().to_string();
    let line_content = line_content.replace(match_text, &highlighted_match);

    println!(
        "{}:{}: {}",
        file_path.bold().green(),
        line_number.bold().blue(),
        line_content.trim_end()
    );
}

pub fn print_minimal_match_result(match_result: RegexMatch) {
    let current_file = &match_result.file_name;
    let line_number = match_result.line_number;
    let line = &match_result.line_content;
    let match_text = &match_result.matched_text;

    let line_number = line_number.unwrap_or(0);
    let line_number_str = if line_number > 0 {
        line_number.to_string()
    } else {
        "-".to_string()
    };
    print_minimal_line(match_text, current_file, &line_number_str, line)
}
