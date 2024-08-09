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

  hercules-ci.github-pages.branch = "main";

  perSystem = { config, ... }: {
    hercules-ci.github-pages.settings.contents = config.packages.plutus-ledger-api-rust-github-pages;
  };

  herculesCI = herculesArgs: {
    onPush.default = {
      outputs.effects = withSystem "x86_64-linux"
        ({ hci-effects, config, ... }:
          hci-effects.runIf
            (herculesArgs.config.repo.tag != null && (builtins.match "v([0-9])+(\.[0-9]+)*(-[a-zA-Z]+)*" herculesArgs.config.repo.tag) != null)
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

