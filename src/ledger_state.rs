use crate::plutus_data::{FromPlutusData, PlutusData, PlutusDataError, ToPlutusData};
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Number of epochs elapsed since genesis
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Epoch(pub BigInt);

impl ToPlutusData for Epoch {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for Epoch {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Number of slots elapsed since genesis
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Slot(pub BigInt);

impl ToPlutusData for Slot {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for Slot {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Kind of a network
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum NetworkKind {
    Testnet,
    Mainnet,
}

/// Network kind and id (in case of a testnet)
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum NetworkId {
    Testnet(BigInt),
    Mainnet,
}
