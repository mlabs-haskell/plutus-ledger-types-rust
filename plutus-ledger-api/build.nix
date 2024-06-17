{ inputs, ... }: {
  perSystem = { config, system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          version = "0.2.0";
          crateName = "plutus-ledger-api";
          devShellHook = config.settings.shell.hook;
          cargoNextestExtraArgs = "--all-features";
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
