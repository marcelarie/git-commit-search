use anyhow::{Context, Result};
use git2::{Diff, DiffFormat};
use regex::Regex;

use crate::{args::has_no_ignore, git::repo::GcsIgnoreMatcher};

#[derive(Debug, Clone)]
pub struct RegexMatch {
    pub matched_text:     String,
    pub file_name:        String,
    pub line_number:      Option<u32>,
    pub line_content:     String,
    pub line_change_type: char,
}

/// Check if a commit diff matches the regex.
/// Returns a boolean indicating whether a match was found and the matching lines.
pub fn matches_diff(
    diff: &Diff,
    regex: &Regex,
    gcs_ignore_matcher: &GcsIgnoreMatcher,
) -> (bool, Vec<RegexMatch>) {
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

        // Skip context lines
        let line_origin = line.origin();
        if line_origin != '+' && line_origin != '-' {
            return true;
        }

        if let Some(match_result) = regex.find(&content) {
            let match_text = match_result.as_str().to_string();

            if let Some(file_path) = delta.new_file().path() {
                let file_name = file_path.to_string_lossy().to_string();

                if !has_no_ignore() {
                    let is_ignored =
                        gcs_ignore_matcher.is_file_ignored(&file_name);
                    if is_ignored {
                        return true;
                    }
                }

                // File is not ignored, mark as found
                found_match = true;

                let line_number =
                    line.new_lineno().or_else(|| line.old_lineno());

                matches.push(RegexMatch {
                    matched_text: match_text,
                    file_name,
                    line_number,
                    line_content: content.trim_end().to_string(),
                    line_change_type: line.origin(),
                });
            }
        }

        true
    })
    .unwrap_or_default();

    (found_match, matches)
}

pub fn create_regex(pattern: String) -> Result<Regex> {
    Regex::new(&pattern).with_context(|| {
        format!("Could not create regex from pattern: {}", pattern)
    })
}
