{ inputs, ... }: {
  perSystem = { config, inputs', system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          inherit (inputs) crane;
          crateName = "plutus-ledger-api";
          devShellHook = config.settings.shell.hook;
          extraSources = [
            inputs'.lbf.packages.lbr-prelude-rust-src
            inputs'.lbf.packages.lbr-prelude-derive-rust-src
          ];
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
