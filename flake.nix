{
  description = "Plutus Ledger types and utilities implemented in Rust";
  inputs = {
    lbf.url = "github:mlabs-haskell/lambda-buffers";

    flake-lang.follows = "lbf/flake-lang";
    nixpkgs.follows = "lbf/nixpkgs";

    # Flakes as modules, using this extensively to organize the repo into modules (build.nix files)
    flake-parts.follows = "lbf/flake-parts";

    # Hercules CI effects
    hci-effects.url = "github:szg251/hercules-ci-effects?ref=cargo-publish";

    # Code quality automation
    pre-commit-hooks.follows = "lbf/pre-commit-hooks";
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
