use clap::{Arg, Command};

pub fn parse_args() -> (String, String, usize, bool) {
    let matches = Command::new("git-commit-search")
        .version("1.0")
        .about("Search and highlight changes across the entire Git commit history using regex patterns.")
        .arg(
            Arg::new("regex")
                .help("The regex pattern to match in the diff")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .help("Path to the repository")
                .default_value("."),
        )
        .arg(
            Arg::new("context-lines")
                .long("conlines")
                .short('l')
                .help("Number of context lines to display around matches.")
                .value_parser(clap::value_parser!(usize))
                .default_value("1"),
        )
        .arg(
            Arg::new("no-gitignore")
            .long("no-gitignore")
            .help("Search all files, ignoring .gitignore rules.")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("diff-tool")
            .long("diff-tool")
            .short('d')
            .help("External diff tool to use (e.g., delta, colordiff)")
            .env("DIFF_TOOL")
        )
        .arg(
            Arg::new("file-pattern")
            .long("file-pattern")
            .short('f')
            .help("Restrict search to specific file patterns (e.g., *.rs, *.toml).")
        )
        .arg(
            Arg::new("interactive")
            .long("interactive")
            .short('i')
            .help("Enables interactive mode for reviewing matches.")
        )
        .get_matches();

    let regex = matches.get_one::<String>("regex").unwrap().to_string();
    let path = matches.get_one::<String>("path").unwrap().to_string();
    let context_lines = *matches.get_one::<usize>("context-lines").unwrap();
    let no_gitignore = matches.get_flag("no-gitignore");

    (regex, path, context_lines, no_gitignore)
}
