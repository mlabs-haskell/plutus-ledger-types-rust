{ inputs, ... }: {
  perSystem = { config, inputs', system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          crateName = "plutus-ledger-api";
          version = "0.2.0";
          devShellHook = config.settings.shell.hook;
          extraSources = [
            inputs'.lbf.packages.lbr-prelude-rust-src
            inputs'.lbf.packages.lbr-prelude-derive-rust-src
          ];
          cargoNextestExtraArgs = "--all-features";
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
