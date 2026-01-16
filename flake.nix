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

            (final: prev: {
              # FIXME: https://github.com/NixOS/nixpkgs/pull/480112
              gungraun-runner = final.callPackage ./pkgs/gungraun-runner { };
            })
          ];
        }
      );

      perSystemPkgs = f: perSystem (system: f (systemPkgs.${system}));
    in
    {
      devShells = perSystemPkgs (pkgs: {
        # nix develop
        default = pkgs.mkShell {
          name = "duramen-shell";

          env = {
            # Nix
            NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native";
            RUSTDOCFLAGS = "-D warnings --html-in-header docs/arborium.html";
            CARGO_INCREMENTAL = "0";
            INSTA_TEST_RUNNER = "nextest";
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
            sccache
            taplo
            cargo-deny
            cargo-fuzz
            cargo-hack
            cargo-insta
            cargo-llvm-cov
            cargo-nextest
            cargo-outdated
            # FIXME: https://nixpkgs-tracker.ocfox.me/?pr=480054
            # cargo-semver-checks
            cargo-shear
            vscode-extensions.vadimcn.vscode-lldb.adapter

            # Build
            protobuf

            # Benchmarking
            gungraun-runner

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
        ci = pkgs.mkShell {
          name = "duramen-ci-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTDOCFLAGS = "-D warnings";
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
            sccache
            cargo-deny
            cargo-hack
            cargo-nextest

            # Build
            protobuf

            # GitHub
            zizmor

            # Spellchecking
            typos
          ];
        };

        # nix develop .#ci-nightly
        ci-nightly = pkgs.mkShell {
          name = "duramen-ci-nightly-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTDOCFLAGS = "-D warnings --html-in-header docs/arborium.html";
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
            sccache
            cargo-fuzz
            cargo-llvm-cov

            # Build
            protobuf

            # Benchmarking
            gungraun-runner
          ];
        };

        # nix develop .#ci-msrv
        ci-msrv = pkgs.mkShell {
          name = "duramen-ci-msrv-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable."1.89.0".minimal.override {
              targets = [
                "thumbv7m-none-eabi"
                "wasm32-unknown-unknown"
              ];
            })
            sccache
            cargo-hack

            # Build
            protobuf
          ];
        };
      });
    };
}
