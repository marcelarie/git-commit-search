## [unreleased]

### 🚀 Features

- Search by regex and print diff
- Add clap for argument parsing and refactor
- Add context lines to the output and refactor
- Print file path and line number for diff
- Change binary name to gcs
- Make the regex param piositional
- Add context lines as a cli argument
- Delete long separator
- Implement gitignore support
- Add no-gitignore argument
- Add no-gitignore flag to ignore .gitignore rules
- Get the terminal width and use it to pad the diff lines
- Add diff-tool arg
- Improve diff_tool arg
- Add two printing modes
- Improve the minimal mode and show all patches at once in full mode
- Highlight match text in minimal print
- Improve minimal print colors and show line change type
- Show full path in minimal output if path is provided as argument
- Add show metadata flag
- Create a global variable to store the repo path
- Create no gitignore mode global variable
- Add gitignore support
- Handle multiple and nested .gitignore files
- Create cache file on repo initialization
- Add fish completions

### 🐛 Bug Fixes

- Show only commit info from matches
- Handle 0 context lines
- Use format! to create file_path
- Minimal commit print
- Ignore hunk header in matches
- Ignore context lines in matches_diff to prevent false matches

### 🚜 Refactor

- Expand diff module and refactor
- Delete old code
- Update logic to print commit info as patch for matching diffs
- Separate functions by concerns and create regex utils mod
- Move open repository logic to git/repo mod
- Refactor the main logic to handle different modes and errors
- Add .gcsignore file to exclude files or directories from the search
- Move commit.rs to git/commit.rs
