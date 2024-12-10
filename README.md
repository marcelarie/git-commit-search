# git-commit-search

CLI to search Git commit history, with added features and improvements over the
native Git command:

```bash
git log -G"<REGEX>" --all -p --stat
```

Goes through each commit, checks the changes, and finds lines that match the
regex. Then it prints all the matches.

## Features

- Search for specific patterns via regex on all commits and diffs.
- Respects gitignore rules by default.
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

# Search for removed test functions
gcs "^-\s*#\[test\]"

# Find version bumps in Cargo.toml
gcs '+version\s*=\s*"\d+\.\d+\.\d+"'
```

This will search for the string "TODO" in all commits of the specified repository.

## Options

- **`-p, --path`**: The path to the repository (optional, defaults to the
  current directory).
- **`-l, --conlines`**: The number of context lines to display on the top and
  bottom of the match (optional, defaults to 1).

## Dependencies

- [rust](https://www.rust-lang.org/)
- [git](https://git-scm.com/)
