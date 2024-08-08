{ inputs, ... }: {
  perSystem = { config, system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          version = "0";
          crateName = "plutus-ledger-api";
          devShellHook = config.settings.shell.hook;
          cargoNextestExtraArgs = "--all-features";
          extraSourceFilters = [
            (path: _type: builtins.match ".*golden$" path != null)
          ];
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
