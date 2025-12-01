{
  description = "Rust development environment for Advent of Code 2025";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShell = pkgs.mkShell {
        name = "aoc-2025-rust-env";

        # Environment packages
        buildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer

          # super useful tools
          cargo-edit # cargo add, cargo rm, cargo upgrade
          cargo-watch # auto-run on file change
          cargo-nextest # faster tests
        ];

        # Optional env vars for Rust
        RUST_BACKTRACE = "1";
      };
    });
}
