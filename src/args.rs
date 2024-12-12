use std::sync::OnceLock;

use clap::{Arg, Command};

#[allow(warnings)]
pub struct ArgsResult {
    pub regex_pattern: String,
    pub path:          String,
    pub context_lines: usize,
    pub no_gitignore:  bool,
    pub diff_tool:     Option<String>,
    pub show_metadata: bool,
    // pub file_pattern:  Option<String>,
    // pub interactive:   bool,
}

pub static SHOW_METADATA_GLOBAL: OnceLock<bool> = OnceLock::new();

pub fn has_show_metadata_mode() -> bool {
    *SHOW_METADATA_GLOBAL.get().unwrap_or(&false)
}

pub fn parse_args() -> ArgsResult {
    let matches = Command::new("git-commit-search")
        .version("1.0")
        .about("Search and highlight changes across the entire Git commit history using regex patterns.")
        .arg( Arg::new("regex")
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
            Arg::new("show-metadata")
            .long("show-metadata")
            .short('m')
            .help("Show commit metadata (author, email, message).")
            .action(clap::ArgAction::SetTrue)
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

    let regex_pattern = matches.get_one::<String>("regex").unwrap().to_string();
    let path = matches.get_one::<String>("path").unwrap().to_string();
    let context_lines = *matches.get_one::<usize>("context-lines").unwrap();
    let diff_tool = matches.get_one::<String>("diff-tool").cloned();
    let no_gitignore = matches.get_flag("no-gitignore");
    let show_metadata = matches.get_flag("show-metadata");

    SHOW_METADATA_GLOBAL.get_or_init(|| show_metadata);

    ArgsResult {
        regex_pattern,
        path,
        context_lines,
        no_gitignore,
        diff_tool,
        show_metadata,
    }
}
