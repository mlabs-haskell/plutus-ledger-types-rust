{ inputs, ... }: {
  imports = [
    inputs.pre-commit-hooks.flakeModule
    inputs.flake-lang.flakeModules.rustMonorepoPreCommit
  ];
  perSystem = { config, ... }:
    {
      devShells.dev-pre-commit = config.pre-commit.devShell;
      devShells.default = config.pre-commit.devShell;

      pre-commit = {
        settings = {
          hooks = {
            nixpkgs-fmt.enable = true;
            deadnix.enable = true;
            statix.enable = true;
            shellcheck.enable = true;
            typos.enable = true;
            markdownlint.enable = true;
            rustfmt-monorepo.enable = true;
          };

        };
      };
    };
}
