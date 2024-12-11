use colored::*;

const ORANGE_BACKGROUND: (u8, u8, u8) = (255, 165, 0);

pub fn print_commit(commit_id: &str) {
    let (r, g, b) = ORANGE_BACKGROUND;
    print!("\ncommit: {}\n", commit_id.black().bold().on_truecolor(r, g, b));
}
