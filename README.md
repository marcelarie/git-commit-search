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

```bash
cargo install --git https://github.com/marcelarie/git-commit-search
```

or clone the repository and build the tool using `cargo`:

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

## Ignoring Files

Create a `.gcsignore` file in any directory of the project to exclude files from searches.
Like `.gitignore`, it affects the directory it's in and all subdirectories:

For example:

```gitignore
# Ignore all markdown files
*.md

# Ignore a specific directory
temp/
```

## Options

- **`-p, --path`**: The path to the repository (optional, defaults to the
  current directory).
- **`-l, --conlines`**: The number of context lines to display on the top and
  bottom of the match (optional, defaults to 1).
- **`--no-ignore`**: Ignore `.gcsignore` rules.

## On the Roadmap

The main features and improvements planned for the tool are listed in the [TODO](TODO.md) file.

The three main goals are:

- Add ranges, from commit A to B, or from HEAD~1 to HEAD\~100
- Use cache if last commit did not change
- Be really fast.
- Have an interactive mode with real-time search.
- Syntax highlighting and good output formatting.

## Development

This project provides a reproducible development shell via [Nix Flakes](https://nix.dev/concepts/flakes.html). 
Once you have Nix installed with flakes enabled, you don’t need to install Rust
or any other tooling locally.

```bash
nix development 
```

## Dependencies

- [rust](https://www.rust-lang.org/)
- [git](https://git-scm.com/)
