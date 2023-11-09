//! Types related to Cardano transactions.
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
pub use crate::v1::transaction::{POSIXTime, POSIXTimeRange, TransactionHash, TransactionInput};
use crate::v2::address::Address;
use crate::v2::datum::OutputDatum;
use crate::v2::script::ScriptHash;
use crate::v2::value::Value;
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An output of a transaction
///
/// This must include a target address, an amount, an optional datum and an optional reference
/// script
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionOutput {
    pub address: Address,
    pub datum: OutputDatum,
    pub reference_script: Option<ScriptHash>,
    pub value: Value,
}

impl IsPlutusData for TransactionOutput {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.address.to_plutus_data(),
                self.datum.to_plutus_data(),
                self.reference_script.to_plutus_data(),
                self.value.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 4)?;
                    Ok(TransactionOutput {
                        address: Address::from_plutus_data(fields[0].clone())?,
                        datum: OutputDatum::from_plutus_data(fields[1].clone())?,
                        reference_script: <Option<ScriptHash>>::from_plutus_data(
                            fields[2].clone(),
                        )?,
                        value: Value::from_plutus_data(fields[3].clone())?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(&data),
            }),
        }
    }
}

/// An input of a pending transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TxInInfo {
    pub transaction_input: TransactionInput,
    pub resolved: TransactionOutput,
}

impl IsPlutusData for TxInInfo {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.transaction_input.to_plutus_data(),
                self.resolved.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(TxInInfo {
                        transaction_input: TransactionInput::from_plutus_data(fields[0].clone())?,
                        resolved: TransactionOutput::from_plutus_data(fields[1].clone())?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(&data),
            }),
        }
    }
}
