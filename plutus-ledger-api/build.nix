{ inputs, lib, ... }: {
  perSystem = { config, system, pkgs, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          version = "2";
          crateName = "plutus-ledger-api";
          devShellHook = config.settings.shell.hook;
          cargoNextestExtraArgs = "--all-features";
          extraSourceFilters = [
            (path: _type: builtins.match ".*golden$" path != null)
          ];
        };

      plutus-ledger-api-rust-github-pages = pkgs.stdenv.mkDerivation {
        name = "plutus-ledger-api-github-pages";
        src = rustFlake.packages.plutus-ledger-api-rust-doc;
        buildPhase = ''
          mkdir $out
          cp -r -L -v $src/share/doc/* $out/
          echo '<meta http-equiv="refresh" content="0; url=plutus_ledger_api">' > $out/index.html
        '';
      };
    in
    lib.mkMerge
      [
        {
          inherit (rustFlake) packages checks devShells;
        }
        {
          packages = {
            inherit plutus-ledger-api-rust-github-pages;
          };
        }
      ];
}
