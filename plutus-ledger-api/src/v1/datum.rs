//! Types related to Plutus Datums
use crate::plutus_data::{IsPlutusData, PlutusData, PlutusDataError};
use crate::v1::crypto::LedgerBytes;
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// blake2b-256 hash of a datum
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DatumHash(pub LedgerBytes);

impl IsPlutusData for DatumHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Piece of information associated with a UTxO encoded into a PlutusData type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Datum(pub PlutusData);

impl IsPlutusData for Datum {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.clone()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}
