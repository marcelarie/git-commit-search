mod args;
mod commit;
mod git;
mod print;
mod regex_utils;

use args::parse_args;
use commit::walk_commits;
use git::{generate_patch, get_commit_diff, use_diff_tool};
use git2::Repository;
use print::{print_commit, print_minimal_match};
use regex_utils::matches_diff;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (regex_pattern, path, _context_lines, _no_gitignore, diff_tool) =
        parse_args();
    let regex = regex::Regex::new(&regex_pattern)?;
    let repo = Repository::open(&path)?;
    let mut patches = Vec::new();

    let commits = walk_commits(&repo)?;

    for commit in commits {
        let diff = get_commit_diff(&repo, &commit)?;
        let (has_matches, matches) = matches_diff(&diff, &regex);

        if has_matches {
            let patch = generate_patch(&commit, &diff)?;
            patches.push(patch.clone());

            if let Some(ref tool) = diff_tool {
                use_diff_tool(tool, &patch)?;
            } else {
                // Minimal mode
                print_commit(&commit);
                for (current_file, line_number, line) in matches {
                    let line_number = line_number.unwrap_or(0);
                    let line_number_str = if line_number > 0 {
                        line_number.to_string()
                    } else {
                        "-".to_string()
                    };
                    print_minimal_match(&current_file, &line_number_str, &line);
                }
                // Full patch mode
                // Fallback: Print the patch directly
                // println!("{}", patch);
            }
        }
    }

    Ok(())
}
