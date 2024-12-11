use colored::*;
use git2::Commit;

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
    file_path: &str,
    line_number: &str,
    line_content: &str,
) {
    println!(
        "{}:{}: {}",
        file_path.bold().green(),
        line_number.bold().blue(),
        line_content.trim_end()
    );
}

pub fn print_minimal_match_result(match_result: (String, Option<u32>, String)) {
    let (current_file, line_number, line) = match_result;
    let line_number = line_number.unwrap_or(0);
    let line_number_str = if line_number > 0 {
        line_number.to_string()
    } else {
        "-".to_string()
    };
    print_minimal_line(&current_file, &line_number_str, &line);
}
