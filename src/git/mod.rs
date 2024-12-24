pub mod diff;
pub mod repo;

pub use diff::generate_patch;
pub use diff::get_commit_diff;
pub use diff::use_diff_tool;
pub use repo::open_repository;
