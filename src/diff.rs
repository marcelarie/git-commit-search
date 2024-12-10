use crate::utils::ChangeType;
use git2::Diff;
use regex::Regex;

pub fn print_diff(diff: &Diff, regex: &Regex) -> Result<(), git2::Error> {
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        let content = String::from_utf8_lossy(line.content());

        if regex.is_match(&content) {
            if let Some(change_type) = match line.origin() {
                '+' => Some(ChangeType::Addition),
                '-' => Some(ChangeType::Deletion),
                _ => None,
            } {
                println!("{}", change_type.format_line(&content));
            } else {
                println!("{}", content);
            }
        }
        true
    })?;
    Ok(())
}
