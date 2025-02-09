mod args;
mod git;
mod print;
mod regex_utils;

use colored::*;
use regex_utils::create_regex;
use std::{env, error::Error, path::Path};

use args::{parse_args, ArgsResult};
use git::{
    initialize_cache, open_repository, process_minimal_mode,
    process_with_diff_tool, walk_commits, GcsIgnoreMatcher,
};

fn run() -> Result<(), Box<dyn Error>> {
    let ArgsResult {
        regex_pattern,
        path,
        context_lines: _,
        no_ignore,
        diff_tool,
        show_metadata: _,
        // completion: _,
    } = parse_args();

    let regex = create_regex(regex_pattern)?;
    let repo_path = Path::new(&path);
    let repo = open_repository(repo_path)?;
    let gcsignore_matcher = GcsIgnoreMatcher::new(repo_path, no_ignore)?;
    initialize_cache(repo_path)?;

    let commits = walk_commits(&repo)?;

    if diff_tool.is_some() {
        process_with_diff_tool(
            commits,
            &repo,
            &regex,
            diff_tool,
            gcsignore_matcher,
        )?;
    } else {
        process_minimal_mode(
            commits,
            &repo,
            &regex,
            repo_path,
            gcsignore_matcher,
        )?;
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}\n{}\n", "Error:".red().bold(), error);

        // Shown in debug mode
        // if let Some(cause) = error.source() {
        //     eprintln!("{}\n{}\n", "Caused by:".bold().blue(), cause);
        // }

        println!("Please use --help for more information.");
        std::process::exit(1);
    }
}
