{
  description = "git-commit-search dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer
          bacon
        ];

        shellHook = ''
          export RUST_BACKTRACE=1
          echo "🦀 Rust $(rustc --version) is ready"
        '';
      };
    });
}
