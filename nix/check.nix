{ pkgs, toolchain }:

let
  inherit (pkgs) writeShellApplication;
in
{

  # Format
  check-rustfmt = (writeShellApplication {
    name = "check-rustfmt";
    runtimeInputs = [ toolchain ];
    text = "cargo fmt --check";
  });

  # Spelling
  check-spelling = (writeShellApplication {
    name = "check-spelling";
    runtimeInputs = with pkgs; [ git codespell ];
    text = ''
      codespell \
        --ignore-words-list="crate" \
        --skip="./target,.git" \
        .
    '';
  });

  # NixFormatting
  check-nixpkgs-fmt = (writeShellApplication {
    name = "check-nixpkgs-fmt";
    runtimeInputs = with pkgs; [ git nixpkgs-fmt findutils ];
    text = ''
      nixpkgs-fmt --check .
    '';
  });

  # Semver
  check-semver = (writeShellApplication {
    name = "check-semver";
    runtimeInputs = with pkgs; [ cargo-semver-checks ];
    text = ''
      cargo-semver-checks semver-checks check-release
    '';
  });
}
