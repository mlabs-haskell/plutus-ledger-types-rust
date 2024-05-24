# Developer guidelines

## Versioning and changelogging

Packages in this repository must be versioned using [PVP][pvp] for Haskell and
PureScript packages, and [Semantic Versioning 2.0.0][semver] for other languages.

Most importantly, minor and patch changes must not include any breaking changes:
no entity is removed, and there's no change in type definitions and functionality
of preexisting exported entities. If any of this occurs, a major version must be
bumped. Disregarding this rule can end up in breaking client package auto-updates.

Any changes must be logged in `CHANGELOG.md`, which must comply with [Keep A
Changelog](https://keepachangelog.com/en/1.1.0/) requirements. Each entry should
also provide a link to the GitHub issue and/or Pull Request that corresponds to
the entry.

An example entry is below:

```lang-none
* Something is fixed
  [#123](https://github.com/mlabs/plutus-ledger-api-rust/issues/123)
```

[pvp]: https://pvp.haskell.org/
[semver]: https://semver.org/
