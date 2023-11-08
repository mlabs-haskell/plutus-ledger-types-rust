//! Types related to Plutus scripts
use crate::crypto::LedgerBytes;
use crate::plutus_data::{FromPlutusData, PlutusData, PlutusDataError, ToPlutusData};
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Identifier of a validator script
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ValidatorHash(pub ScriptHash);

impl ToPlutusData for ValidatorHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for ValidatorHash {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Hash of a minting policy script
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct MintingPolicyHash(pub ScriptHash);

impl ToPlutusData for MintingPolicyHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for MintingPolicyHash {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Hash of a Plutus script
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ScriptHash(pub LedgerBytes);

impl ToPlutusData for ScriptHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for ScriptHash {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}
