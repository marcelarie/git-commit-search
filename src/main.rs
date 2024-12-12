mod args;
mod commit;
mod git;
mod print;
mod regex_utils;

use colored::*;
use regex_utils::create_regex;
use std::{env, error::Error, path::Path};

use args::parse_args;
use commit::{process_minimal_mode, process_with_diff_tool, walk_commits};
use git::open_repository;

fn run() -> Result<(), Box<dyn Error>> {
    let (regex_pattern, path, _context_lines, _no_gitignore, diff_tool) =
        parse_args();

    let regex = create_regex(regex_pattern)?;
    let repo_path = Path::new(&path);
    let repo = open_repository(repo_path)?;

    let commits = walk_commits(&repo)?;
    let has_diff_tool = diff_tool.is_some();

    if has_diff_tool {
        process_with_diff_tool(commits, &repo, &regex, diff_tool)?;
    } else {
        process_minimal_mode(commits, &repo, &regex, repo_path)?;
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
