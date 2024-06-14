//! Types related to Plutus Redeemers
use crate::plutus_data::{IsPlutusData, PlutusData, PlutusDataError};
use crate::v1::crypto::LedgerBytes;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Piece of information attached to a transaction when redeeming a value from a validator or a
/// minting policy
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Redeemer(pub PlutusData);

impl IsPlutusData for Redeemer {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.clone()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

/// blake2b-256 hash of a datum
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RedeemerHash(pub LedgerBytes);

impl IsPlutusData for RedeemerHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}
