{
  description = "A CLI to display information about x509 certificates.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
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
    , crane
    , pre-commit-hooks
    }:

    flake-utils.lib.eachDefaultSystem
      (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideScope' (_: _: {
          cargo = rust-toolchain;
          clippy = rust-toolchain;
          rustc = rust-toolchain;
          rustfmt = rust-toolchain;
        });

        src = ./.;

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };

        x509-info = craneLib.buildPackage {
          inherit cargoArtifacts src;
        };
      in
      {
        packages.default = x509-info;
        apps.default = flake-utils.lib.mkApp {
          drv = x509-info;
        };

        checks = {
          pre-commit = pre-commit-hooks.lib.${system}.run {
            inherit src;
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
            rust-toolchain
            cargo-deny
          ];

          inherit (self.checks.${system}.pre-commit) shellHook;
        };
      })
    // {
      overlay = final: prev: {
        x509-info = self.packages.${prev.system}.default;
      };
    };
}

