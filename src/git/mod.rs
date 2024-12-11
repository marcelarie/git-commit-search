pub mod diff;

pub use diff::generate_patch;
pub use diff::walk_commits;
pub use diff::get_commit_diff;
pub use diff::diff_matches_regex;
