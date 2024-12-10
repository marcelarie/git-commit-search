# git-commit-search

**`gcs`** is a CLI tool designed to enhance and simplify searching
through Git commit history, with additional features and improvements over the
native Git command:

```bash
git log -G"<REGEX>" --all -p --stat
```

## Features

- Search for specific patterns via regex on all commits and diffs.
- Display matching context.

## Installation

Clone the repository and build the tool using `cargo`:

```bash
git clone <repository-url>
cd git-commit-search
cargo build --release
```

## Usage

Run the tool with the desired regex pattern and optional repository path:

```bash
gcs --regex "<REGEX>" --path /path/to/repo
```

### Examples:

```bash
# Search for the text TODO 
gcs -r "TODO" -p ~/my-project

# Search for TODO comments with assignee
gcs -r "TODO\s*\(@\w+\):" -p ~/my-project

# Find password or API key assignments
gcs -r "(?i)(password|api_key)\s*=\s*['\"][^'\"]+['\"]"

# Search for removed test functions
gcs -r "^-\s*#\[test\]"

# Find version bumps in Cargo.toml
gcs -r '+version\s*=\s*"\d+\.\d+\.\d+"'
```

This will search for the string "TODO" in all commits of the specified repository.

## Options

- **`-r, --regex`**: The regex pattern to search for (required).
- **`-p, --path`**: The path to the repository (optional, defaults to the current directory).

## Dependencies

- [rust](https://www.rust-lang.org/)
- [git](https://git-scm.com/)
