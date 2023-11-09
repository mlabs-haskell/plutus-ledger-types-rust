{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";
    hercules-ci-effects.url = "github:hercules-ci/hercules-ci-effects";
  };

  outputs = inputs@{ flake-parts, rust-overlay, nixpkgs, ... }:
    flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = [
          "x86_64-linux"
          "x86_64-darwin"
        ];
        imports = [
          inputs.pre-commit-hooks-nix.flakeModule
          inputs.hercules-ci-effects.flakeModule
        ];
        perSystem = { self', pkgs, system, ... }:
          let
            overlays = [ (import rust-overlay) ];
            crateName = "plutus-ledger-api";
            rustWithTools = pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rustfmt" "rust-analyzer" "clippy" ];
            };
            craneLib = inputs.crane.lib.${system}.overrideToolchain rustWithTools;
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            commonArgs = {
              inherit src;
              strictDeps = true;
            };
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          in
          {
            _module.args.pkgs = import nixpkgs {
              inherit system overlays;
            };

            devShells.default = craneLib.devShell { checks = self'.checks; };

            packages.default = craneLib.buildPackage (commonArgs // {
              inherit cargoArtifacts;
              doTest = false;
            });

            pre-commit.settings.hooks.rustfmt.enable = true;

            checks."${crateName}-test" = craneLib.cargoNextest (commonArgs // {
              inherit cargoArtifacts;
              cargoExtraArgs = "--features lbf";
            });

            checks."${crateName}-clippy" = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
            });
          };
      };

}
