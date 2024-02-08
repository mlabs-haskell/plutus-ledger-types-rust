# Repo wide settings
{ lib, flake-parts-lib, inputs, ... }: {

  options = {

    perSystem = flake-parts-lib.mkPerSystemOption
      ({ pkgs, system, config, ... }: {
        options.settings = {

          shell = {

            tools = lib.mkOption {
              type = lib.types.listOf lib.types.package;
              description = "Tools to include in all devShells";
            };

            hook = lib.mkOption {
              type = lib.types.str;
              description = "Shell script to invoke in all devShells";
            };
          };

        };


        config = {

          settings = {


            shell = {

              tools = [
                pkgs.nil
                inputs.pre-commit-hooks.outputs.packages.${system}.deadnix
                inputs.pre-commit-hooks.outputs.packages.${system}.nixpkgs-fmt
                inputs.pre-commit-hooks.outputs.packages.${system}.shellcheck
              ];

              hook = ''
                export LC_CTYPE=C.UTF-8;
                export LC_ALL=C.UTF-8;
                export LANG=C.UTF-8;
                ${config.pre-commit.installationScript}
              '';
            };
          };
        };

      });

  };

}
