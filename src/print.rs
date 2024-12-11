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

pub fn print_minimal_match(
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
