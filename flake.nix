{
  description = "LambdaBuffers Cardano Demo";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    # Flakes as modules, using this extensively to organize the repo into modules (build.nix files)
    flake-parts.url = "github:hercules-ci/flake-parts";

    # Hercules CI effects
    hci-effects.url = "github:hercules-ci/hercules-ci-effects";

    # Code quality automation
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";

    flake-lang = {
      url = "github:mlabs-haskell/flake-lang.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
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
