use crate::plutus_data::{FromPlutusData, PlutusData, PlutusDataError, ToPlutusData};
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

impl ToPlutusData for Redeemer {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.clone()
    }
}

impl FromPlutusData for Redeemer {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}
