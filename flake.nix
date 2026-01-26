{
  description = "duramen";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";

      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  # nix flake show
  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:

    let
      perSystem = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;

      systemPkgs = perSystem (
        system:

        import nixpkgs {
          inherit system;

          overlays = [
            rust-overlay.overlays.default
          ];
        }
      );

      perSystemPkgs = f: perSystem (system: f (systemPkgs.${system}));
    in
    {
      devShells = perSystemPkgs (pkgs: {
        # nix develop
        default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "duramen-shell";

          env = {
            # Nix
            NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
              "-Z threads=0"
            ];
            RUSTDOCFLAGS = pkgs.lib.concatStringsSep " " [
              "--deny warnings"
              "--html-in-header docs/arborium.html"
            ];
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override {
              targets = [
                "thumbv7m-none-eabi"
                "wasm32-unknown-unknown"
              ];

              extensions = [
                "clippy"
                "llvm-tools"
                "rust-analyzer"
                "rust-src"
                "rustfmt"
              ];
            })
            wild
            sccache
            tombi
            cargo-deny
            cargo-fuzz
            cargo-hack
            cargo-insta
            cargo-llvm-cov
            cargo-nextest
            cargo-outdated
            cargo-semver-checks
            cargo-codspeed
            cargo-shear
            cargo-show-asm
            vscode-extensions.vadimcn.vscode-lldb.adapter

            # GitHub
            zizmor

            # Spellchecking
            typos
            typos-lsp

            # Nix
            nixfmt
            nixd
            nil
          ];
        };

        # nix develop .#ci
        ci = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "duramen-ci-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
            ];
            RUSTDOCFLAGS = pkgs.lib.concatStringsSep " " [
              "--deny warnings"
            ];
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            rust-bin.nightly.latest.rustfmt
            (rust-bin.stable.latest.minimal.override {
              extensions = [
                "clippy"
              ];
            })
            wild
            sccache
            tombi
            cargo-deny
            cargo-hack
            cargo-nextest

            # GitHub
            zizmor

            # Spellchecking
            typos
          ];
        };

        # nix develop .#ci-nightly
        ci-nightly = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "duramen-ci-nightly-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
              "-Z threads=0"
            ];
            RUSTDOCFLAGS = pkgs.lib.concatStringsSep " " [
              "--deny warnings"
              "--html-in-header docs/arborium.html"
            ];
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override {
              extensions = [
                "llvm-tools"
                "rust-src"
              ];
            })
            wild
            sccache
            cargo-codspeed
            cargo-fuzz
            cargo-llvm-cov
          ];
        };

        # nix develop .#ci-msrv
        ci-msrv = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "duramen-ci-msrv-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
            ];
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable."1.93.0".minimal.override {
              targets = [
                "thumbv7m-none-eabi"
                "wasm32-unknown-unknown"
              ];
            })
            wild
            sccache
            cargo-hack
          ];
        };
      });
    };
}
