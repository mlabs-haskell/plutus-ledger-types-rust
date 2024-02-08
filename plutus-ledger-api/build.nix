{ inputs, ... }: {
  perSystem = { config, system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          inherit (inputs) crane;
          crateName = "plutus-ledger-api";
          devShellHook = config.settings.shell.hook;
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
