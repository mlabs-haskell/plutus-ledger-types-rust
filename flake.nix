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
        perSystem = { self', pkgs, system, config, ... }:
          let
            overlays = [ (import rust-overlay) ];
            crateName = "plutus-ledger-api";
            rustWithTools = pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rustfmt" "rust-analyzer" "clippy" "rust-src" ];
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
            debug = true;
            _module.args.pkgs = import nixpkgs {
              inherit system overlays;
            };

            pre-commit.settings.hooks = {
              rustfmt.enable = true;
              taplo.enable = true;
              nixpkgs-fmt.enable = true;
              deadnix.enable = true;
              markdownlint.enable = true;
            };

            devShells.default = craneLib.devShell {
              checks = self'.checks;
              packages = [ config.pre-commit.settings.package ];
              shellHook = config.pre-commit.installationScript;
            };

            packages.default = craneLib.buildPackage (commonArgs // {
              inherit cargoArtifacts;
              doTest = false;
            });


            checks."${crateName}-test" = craneLib.cargoNextest (commonArgs // {
              inherit cargoArtifacts;
              cargoExtraArgs = "--features lbf";
            });

            checks."${crateName}-clippy" = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
            });
          };
        hercules-ci.flake-update = {
          enable = true;
          updateBranch = "updated-flake-lock";
          # Next two parameters should always be set explicitly
          createPullRequest = true;
          autoMergeMethod = "merge";
          when = {
            # Perform update by Sundays at 12:45
            minute = 25;
            hour = 16;
            dayOfWeek = "Mon";
          };
        };
      };

}
