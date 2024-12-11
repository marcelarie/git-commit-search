use git2::{Diff, DiffFormat};
use regex::Regex;

/// Check if a commit diff matches the regex.
/// Returns a boolean indicating whether a match was found and the matching lines.
pub fn matches_diff(
    diff: &Diff,
    regex: &Regex,
) -> (bool, Vec<(String, Option<u32>, String)>) {
    let mut found_match = false;
    let mut matches = Vec::new();

    diff.print(DiffFormat::Patch, |delta, _hunk, line| {
        let content = std::str::from_utf8(line.content())
            .unwrap_or("")
            .trim_end()
            .to_string();

        let is_hunk_header =
            line.new_lineno().is_none() && line.old_lineno().is_none();

        // Skip hunk headers
        if is_hunk_header {
            return true;
        }

        if regex.is_match(&content) {
            found_match = true;

            if let Some(file_path) = delta.new_file().path() {
                let file_name = file_path.to_string_lossy().to_string();
                let line_number =
                    line.new_lineno().or_else(|| line.old_lineno());

                matches.push((
                    file_name,
                    line_number,
                    content.trim_end().to_string(),
                ));
            }
        }
        true
    })
    .unwrap_or_default();

    (found_match, matches)
}