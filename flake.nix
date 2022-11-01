{
  description = "A CLI to display information about x509 certificates.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nmattia/naersk";
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , naersk
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem
      (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-toolchain =
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [ "rust-src" ];
          };
        naersk-lib = naersk.lib."${system}".override {
          rustc = rust-toolchain;
        };
      in
      rec
      {
        packages.x509-info = naersk-lib.buildPackage {
          pname = "x509-info";
          root = ./.;
          buildInputs = with pkgs; [ ];
        };

        defaultPackage = packages.x509-info;

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-toolchain
            cargo-audit
            cargo-deny
            cargo-cross
            rust-analyzer
          ] ++ pkgs.lib.optionals (pkgs.stdenv.isLinux) (with pkgs; [ cargo-watch ]);

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
