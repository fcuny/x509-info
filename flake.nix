{
  description = "A CLI to display information about x509 certificates.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , rust-overlay
    , pre-commit-hooks
    }:
    let
      # Borrow project metadata from the Rust config
      meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      inherit (meta) name version;

      overlays = [
        # Rust helpers
        (import rust-overlay)
        # Build Rust toolchain using helpers from rust-overlay
        (self: super: {
          # This supplies cargo, rustc, rustfmt, etc.
          rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];
    in
    flake-utils.lib.eachDefaultSystem
      (system:
      let
        pkgs = import nixpkgs { inherit overlays system; };
      in
      {
        packages = rec {
          default = x509-info;
          x509-info = pkgs.rustPlatform.buildRustPackage {
            pname = name;
            inherit version;
            src = ./.;
            release = true;
            cargoLock.lockFile = ./Cargo.lock;
          };
        };

        checks = {
          pre-commit = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              clippy = {
                enable = true;
                entry = pkgs.lib.mkForce "cargo clippy -- -D warnings";
              };
              nixpkgs-fmt = {
                enable = true;
              };
              rustfmt = {
                enable = true;
                entry = pkgs.lib.mkForce "cargo fmt -- --check --color always";
              };
            };
          };
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchain
            cargo-audit
            cargo-deny
            rust-analyzer
          ];

          inherit (self.checks.${system}.pre-commit) shellHook;
        };
      })
    // {
      overlay = final: prev: {
        x509-info = self.defaultPackage.${prev.system};
      };
    };
}

