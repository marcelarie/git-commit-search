use git2::{Diff, DiffFormat};
use regex::Regex;

/// Check if a commit diff matches the regex.
/// Returns a boolean indicating whether a match was found and the matching lines.
pub fn matches_diff(
    diff: &Diff,
    regex: &Regex,
) -> (bool, Vec<(String, String)>) {
    let mut found_match = false;
    let mut matches = Vec::new();

    diff.print(DiffFormat::Patch, |delta, _hunk, line| {
        let content = std::str::from_utf8(line.content())
            .unwrap_or("")
            .trim_end()
            .to_string();

        if regex.is_match(&content) {
            found_match = true;

            if let Some(file_path) = delta.new_file().path() {
                let file_name = file_path.to_string_lossy().to_string();
                matches.push((file_name, content.trim_end().to_string()));
            }
        }
        true
    })
    .unwrap_or_default();

    (found_match, matches)
}
