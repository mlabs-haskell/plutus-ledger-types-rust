//! Types related to Plutus Redeemers
use crate::plutus_data::{PlutusData, PlutusDataError, IsPlutusData};
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Piece of information attached to a transaction when redeeming a value from a validator or a
/// minting policy
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Redeemer(pub PlutusData);

impl IsPlutusData for Redeemer {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.clone()
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}
