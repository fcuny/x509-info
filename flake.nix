{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts = { url = "github:hercules-ci/flake-parts"; inputs.nixpkgs-lib.follows = "nixpkgs"; };
    treefmt-nix = { url = "github:numtide/treefmt-nix"; inputs.nixpkgs.follows = "nixpkgs"; };
    devshell.url = "github:numtide/devshell";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-darwin" ];
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.pre-commit-hooks.flakeModule
        inputs.devshell.flakeModule
      ];

      perSystem = { pkgs, lib, config, ... }:
        let
          src = ./.;
          package = {
            # Replace the following throws with strings with the appropriate values
            name = "x509-info";
            version = "0.1.0";
            # Since this is a FOD, it must first fail (empty hash) for it to tell you the correct hash
            vendorHash = null;
          };
        in
        {
          packages = {
            ${package.name} = pkgs.buildGoModule {
              pname = package.name;
              inherit (package)
                version
                vendorHash;
              inherit src;
            };
            default = config.packages.${package.name};
          };

          formatter = pkgs.treefmt;

          devshells.default = {
            commands = [
              {
                name = "build";
                category = "dev";
                help = "Build the binary";
                command = "make";
              }
            ];
            packages = with pkgs; [
              go_1_21
              gopls
              golangci-lint
            ];
            devshell.startup = {
              pre-commit.text = config.pre-commit.installationScript;
            };
          };

          treefmt = {
            projectRootFile = "go.mod";
            programs.gofmt.enable = true;
            programs.nixpkgs-fmt.enable = true;
          };

          pre-commit = {
            settings = {
              hooks = {
                treefmt.enable = true;
              };
            };
          };
        };
    };
}
