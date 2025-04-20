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
      opensslFull = pkgs.openssl.override {static = false;};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer
          bacon
          opensslFull.dev
          opensslFull.out
        ];

        shellHook = ''
          export RUST_BACKTRACE=1
          export OPENSSL_NO_VENDOR=1
          export OPENSSL_DIR=${opensslFull.dev}
          export OPENSSL_LIB_DIR=${opensslFull.out}/lib
          export OPENSSL_INCLUDE_DIR=${opensslFull.dev}/include
          echo "🦀 Rust $(rustc --version) is ready"
        '';
      };
    });
}
