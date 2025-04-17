# Todo

- [ ] Improve performance
      Note: gcs `fn \S+\(.\*\)' -m` on the linux repo: 62m43.462s on minimal mode without changes

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

- [ ] Add shell completions
  - [x] fish
  - [ ] bash
  - [ ] zsh
  - [ ] nushell
- [x] Ignore `.gcsignore` rules via parameter (--no-ignore)
- [x] Search using a dir path. (-p,--path).
- [ ] Regex search cache to improve performance.
  - [ ] Use a binary file to store the cache.
  - [x] Create file on repo .git folder.
  - [ ] Check if the cache is up to date.
  - [ ] Decide when to clear the cache.
        (e.g. after a certain number of commits or after a certain time).
- [ ] Add flag to filter by additions and deletions
- [ ] Add flag to search the commit message too.
- [ ] Show context lines. (-l,--conlines).

- [x] shell.nix
- [x] flake.nix

# Discarded

- [ ] ~~Implement
  [tree-sitter-highlight](https://crates.io/crates/tree-sitter-highlight) crate
  for diff syntax highlighting~~
- [ ] ~~Regex optimization~~

