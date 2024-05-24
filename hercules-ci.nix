{ inputs, ... }: {
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

  herculesCI = { config, ... }: {
    onPush.plutus-ledger-api-publish = {
      enable =
        config.repo.branch != null && (builtins.match "v[0-9]+" config.repo.branch) != null;
      outputs = {
        effects.cargo-publish = {
          secretName = "cargo-api-token";
          extraPublishArgs = [
            "--manifest-path ./plutus-ledger-api/Cargo.toml"
            "--dry-run"
          ];
        };
      };
    };


    ciSystems = [ "x86_64-linux" "x86_64-darwin" ];
  };

}
