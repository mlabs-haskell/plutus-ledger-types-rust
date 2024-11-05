<!-- markdownlint-disable MD024 -->
# Changelog

This changelog is based on [Keep A
Changelog](https://keepachangelog.com/en/1.1.0).

## v2.0.0

### Added

- Added cardano-serialization-lib conversion traits (`ToCSL` and `FromCSL`)
- Added v3 plutus ledger types ([#57](https://github.com/mlabs-haskell/plutus-ledger-api-rust/pull/57))
- Added the ability to derive `IsPlutusData` instances ([#56](https://github.com/mlabs-haskell/plutus-ledger-api-rust/pull/56))
- Added a few utility functions for Values
- Added Display and Debug implementations for CurrencySymbol, TokenName, Value, AddressWithExtraInfo

### Changed

- Fixed `serde` serialization of Plutus `Value`s
- Updated cardano-serialization-lib to Conway compatible 12.1.1

## v1.0.0

### Added

- Added golden tests ([#45](https://github.com/mlabs-haskell/plutus-ledger-api-rust/pull/45))

## v0.2.1

### Changed

- Use published lbr-prelude and lbr-prelude-derive ([#42](https://github.com/mlabs-haskell/plutus-ledger-api-rust/pull/42))

## v0.2.0

Start of this Changelog

## v0.1.0

MVP version including all Plutus Ledger types
