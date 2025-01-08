use std::sync::OnceLock;

use clap::{Arg, Command};
#[allow(warnings)]
pub struct ArgsResult {
    pub regex_pattern: String,
    pub path:          String,
    pub context_lines: usize,
    pub no_ignore:  bool,
    pub diff_tool:     Option<String>,
    pub show_metadata: bool,
    pub completion: String,
    // pub file_pattern:  Option<String>,
    // pub interactive:   bool,
}

pub static SHOW_METADATA_GLOBAL: OnceLock<bool> = OnceLock::new();
pub static REPO_PATH_GLOBAL: OnceLock<String> = OnceLock::new();
pub static NO_IGNORE_GLOBAL: OnceLock<bool> = OnceLock::new();

pub fn has_show_metadata_mode() -> bool {
    *SHOW_METADATA_GLOBAL.get().unwrap_or(&false)
}

#[allow(dead_code)]
pub fn get_repo_path() -> String {
    REPO_PATH_GLOBAL.get().unwrap_or(&String::new()).to_string()
}

pub fn has_no_ignore() -> bool {
    *NO_IGNORE_GLOBAL.get().unwrap_or(&false)
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
            Arg::new("no-ignore")
            .long("no-ignore")
            .help("Search all files, ignoring .gcsignore rules.")
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
        .arg(
            Arg::new("completion")
            .long("completion")
            .help("Generates shell completion scripts (bash, zsh, fish).")
        )
        .get_matches();

    let regex_pattern = matches.get_one::<String>("regex").unwrap().to_string();
    let path = matches.get_one::<String>("path").unwrap().to_string();
    let context_lines = *matches.get_one::<usize>("context-lines").unwrap();
    let diff_tool = matches.get_one::<String>("diff-tool").cloned();
    let no_ignore = matches.get_flag("no-ignore");
    let show_metadata = matches.get_flag("show-metadata");
    let completion = matches.get_one::<String>("completion").unwrap().to_string();

    SHOW_METADATA_GLOBAL.get_or_init(|| show_metadata);
    REPO_PATH_GLOBAL.get_or_init(|| path.clone());
    NO_IGNORE_GLOBAL.get_or_init(|| no_ignore);

    ArgsResult {
        regex_pattern,
        path,
        context_lines,
        no_ignore,
        diff_tool,
        show_metadata,
        completion
    }
}
