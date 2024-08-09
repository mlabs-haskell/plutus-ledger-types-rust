{
  description = "Plutus Ledger types and utilities implemented in Rust";
  inputs = {
    flake-lang.url = "github:mlabs-haskell/flake-lang.nix/cargo-doc";
    nixpkgs.follows = "flake-lang/nixpkgs";

    # Flakes as modules, using this extensively to organize the repo into modules (build.nix files)
    flake-parts.follows = "flake-lang/flake-parts";

    # Hercules CI effects
    hci-effects.url = "github:hercules-ci/hercules-ci-effects";

    # Code quality automation
    pre-commit-hooks.follows = "flake-lang/pre-commit-hooks";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./pkgs.nix
        ./settings.nix
        ./pre-commit.nix
        ./hercules-ci.nix

        ./plutus-ledger-api/build.nix
      ];
      debug = true;
      systems = [ "x86_64-linux" "x86_64-darwin" ];
    };
}
