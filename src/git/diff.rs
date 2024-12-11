use std::fmt::Write;
use git2::{Commit, Diff, DiffFormat, Error, Repository};
use regex::Regex;

/// Walk through all commits in the repository
pub fn walk_commits(repo: &Repository) -> Result<Vec<Commit>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    revwalk
        .filter_map(|oid| oid.ok())
        .map(|oid| repo.find_commit(oid))
        .collect()
}

/// Generate a `Diff` object for a given commit.
///
/// **Rules:**
/// - If the commit has a parent, the diff will compare the parent with the commit.
/// - If the commit has no parent, the diff will compare against an empty tree.
pub fn get_commit_diff<'repo>(
    repo: &'repo Repository,
    commit: &'repo Commit,
) -> Result<Diff<'repo>, Error> {
    let tree = commit.tree()?;

    let diff = match commit.parents().next() {
        Some(parent) => {
            let parent_tree = parent.tree()?;
            repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)
        }
        None => repo.diff_tree_to_tree(None, Some(&tree), None),
    };

    diff
}

/// Check if a commit diff matches the regex.
/// Returns a boolean indicating whether a match was found and the matching lines.
pub fn diff_matches_regex(
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

/// Convert a `git2::Diff` to a full patch string.
pub fn generate_patch(
    diff: &Diff,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut patch = String::new();
    let mut current_hunk: Option<(u32, u32, u32, u32)> = None;
    let mut has_file_header = false;

    diff.print(DiffFormat::Patch, |delta, hunk, line| {
        if !has_file_header {
            let _ = write_file_header(&delta, &mut patch);
            has_file_header = true;
        }

        if let Some(hunk) = hunk {
            let _ = write_hunk_header(&hunk, &mut current_hunk, &mut patch);
        }

        let _ = write_diff_line(&line, &mut patch);
        true
    })?;

    Ok(patch)
}

/// Write the file header to the patch.
pub fn write_file_header(
    delta: &git2::DiffDelta,
    patch: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = delta.new_file().path().unwrap_or_else(|| std::path::Path::new(""));

    writeln!(
        patch,
        "diff --git a/{} b/{}",
        file_path.display(),
        file_path.display()
    )?;

    match delta.status() {
        git2::Delta::Deleted => {
            writeln!(patch, "deleted file mode {:?}", delta.old_file().mode())?;
            writeln!(patch, "--- a/{}", file_path.display())?;
            writeln!(patch, "+++ /dev/null")?;
        }
        git2::Delta::Added => {
            writeln!(patch, "new file mode {:?}", delta.new_file().mode())?;
            writeln!(patch, "--- /dev/null")?;
            writeln!(patch, "+++ b/{}", file_path.display())?;
        }
        _ => {
            writeln!(
                patch,
                "index {}..{} {:?}",
                delta.old_file().id(),
                delta.new_file().id(),
                delta.new_file().mode()
            )?;
            writeln!(patch, "--- a/{}", file_path.display())?;
            writeln!(patch, "+++ b/{}", file_path.display())?;
        }
    }

    Ok(())
}

/// Write the hunk header to the patch if it hasn't been written yet.
pub fn write_hunk_header(
    hunk: &git2::DiffHunk,
    current_hunk: &mut Option<(u32, u32, u32, u32)>,
    patch: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let hunk_info = (
        hunk.old_start(),
        hunk.old_lines(),
        hunk.new_start(),
        hunk.new_lines(),
    );

    if current_hunk.as_ref() != Some(&hunk_info) {
        writeln!(
            patch,
            "@@ -{},{} +{},{} @@",
            hunk.old_start(),
            hunk.old_lines(),
            hunk.new_start(),
            hunk.new_lines()
        )?;
        *current_hunk = Some(hunk_info);
    }

    Ok(())
}

/// Write a diff line to the patch.
pub fn write_diff_line(
    line: &git2::DiffLine,
    patch: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let line_marker = match line.origin() {
        '+' => "+",
        '-' => "-",
        ' ' => " ",
        _ => return Ok(()), // Skip unsupported line origins
    };

    let line_content = std::str::from_utf8(line.content())?.trim_end();
    writeln!(patch, "{}{}", line_marker, line_content)?;

    Ok(())
}
