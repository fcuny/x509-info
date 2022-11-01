{
  description = "A CLI to display information about x509 certificates.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , rust-overlay
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

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchain
            cargo-audit
            cargo-deny
            rust-analyzer
          ];

          shellHook = ''
            cargo --version
          '';
        };
      })
    // {
      overlay = final: prev: {
        x509-info = self.defaultPackage.${prev.system};
      };
    };
}

