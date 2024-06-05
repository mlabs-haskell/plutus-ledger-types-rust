{ inputs, withSystem, ... }: {
  imports = [
    inputs.hci-effects.flakeModule # Adds hercules-ci and herculesCI options
  ];

  hercules-ci.flake-update = {
    enable = true;
    updateBranch = "updated-flake-lock";
    # Next two parameters should always be set explicitly
    createPullRequest = true;
    autoMergeMethod = "merge";
    when = {
      # Perform update by Sundays at 12:45
      minute = 45;
      hour = 12;
      dayOfWeek = "Sun";
    };
  };

  herculesCI = herculesArgs: {
    onPush.default = {
      outputs.effects = withSystem "x86_64-linux"
        ({ hci-effects, config, ... }:
          hci-effects.runIf
            (herculesArgs.config.repo.branch != null && (builtins.match "v[0-9]+" herculesArgs.config.repo.branch) != null)
            (hci-effects.cargoPublish
              {
                src = config.packages.plutus-ledger-api-rust-src;
                secretName = "crates-io-token";
              })
        );
    };

    ciSystems = [ "x86_64-linux" "x86_64-darwin" ];
  };
}

