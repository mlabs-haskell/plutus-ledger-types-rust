{ inputs, ... }: {
  perSystem = { config, system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          version = "0";
          crateName = "plutus-data";
          devShellHook = config.settings.shell.hook;
          cargoNextestExtraArgs = "--all-features";
          extraSources = [
            config.packages.is-plutus-data-derive-rust-src
          ];
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
