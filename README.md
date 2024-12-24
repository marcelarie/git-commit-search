# git-commit-search

CLI tool to search in git commit diff history, with added features and
improvements over the native git command `git log -G"<regex>"`.

Goes through each commit, checks the changes, and finds lines that match the
regex. Then it prints all the matches.

## Features

- Search for specific patterns via regex on all commits and diffs.
- Ignore file (`.gcsignore`) to exclude files or directories from the search,
  works like a `.gitignore` file but for the search.
- Display matching content.

## Installation

Clone the repository and build the tool using `cargo`:

```bash
git clone https://github.com/marcelarie/git-commit-search
cd git-commit-search
cargo build --release
```

## Usage

Run the tool with the desired regex pattern and optional repository path:

```bash
gcs "<REGEX>" --path /path/to/repo
```

### Examples:

```bash
# Search for the text TODO
gcs "TODO" -p ~/my-project

# Search for TODO comments with assignee
gcs "TODO\s*\(@\w+\):" -p ~/my-project

# Find password or API key assignments
gcs "(?i)(password|api_key)\s*=\s*['\"][^'\"]+['\"]"

# Find version bumps in Cargo.toml
gcs '+version\s*=\s*"\d+\.\d+\.\d+"'
```

This will search for the string "TODO" in all commits of the specified repository.

## `.gcsignore` file

The `.gcsignore` file allows you to define patterns for files or directories to
exclude during your searches. This file provides developers with the flexibility
to ignore noisy or irrelevant files, even if they exist in past commit diffs.

To use .gcsignore, create a file named .gcsignore in the root of your repository
and define the patterns you wish to exclude. These patterns work in the same way
as `.gitignore` patterns and can include file names, directory names, or
wildcard expressions.

For example:
```gitignore
# Ignore all markdown files
*.md

# Ignore a specific directory
temp/
```

When `.gcsignore` is present, the CLI will respect these patterns and exclude
matching files or directories from the search.

## Options

- **`-p, --path`**: The path to the repository (optional, defaults to the
  current directory).
- **`-l, --conlines`**: The number of context lines to display on the top and
  bottom of the match (optional, defaults to 1).
- **`--no-ignore`**: Ignore `.gcsignore` rules.

## Todo

- [ ] Improve performance
      gcs 'fn \S+\(.\*\)' -m` on the linux repo: 62m43.462s on minimal mode without changes
  - [ ] Regex optimization
  - [ ] Implement parallel processing and thread pools
  - [ ] Incremental updates (cache)

- [ ] Interactive mode.
  - [ ] Real time regex search.
  - [ ] View the whole commit.
  - [ ] View the whole diff.
  - [ ] Go to next match
  - [ ] Go to prev match
  - [ ] Show file and commit in GitLab or GitHub.
- [ ] Grab multiple matches and use interactive mode on them.
- [ ] Search using a file pattern. (e.g. `*.rs`).
- [ ] Implement
      [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight) crate
      for diff syntax highlighting

- [x] Ignore `.gcsignore` rules via parameter (--no-ignore)
- [x] Search using a dir path. (-p,--path).
- [ ] Add flag to filter by additions and deletions
- [ ] Show context lines. (-l,--conlines).

## Dependencies

- [rust](https://www.rust-lang.org/)
- [git](https://git-scm.com/)
