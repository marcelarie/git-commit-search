pub fn print_commit(commit_id: &str) {
    println!(
        "Commit: \x1b[1m\x1b[30m\x1b[48;2;255;165;0m {} \x1b[0m",
        commit_id
    );
}
