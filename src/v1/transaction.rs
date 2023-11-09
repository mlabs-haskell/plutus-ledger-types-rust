//! Types related to Cardano transactions.
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
use crate::v1::address::Address;
use crate::v1::crypto::LedgerBytes;
use crate::v1::datum::DatumHash;
use crate::v1::interval::PlutusInterval;
use crate::v1::value::Value;
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An input of a transaction
///
/// Also know as `TxOutRef` from Plutus, this identifies a UTxO by its transacton hash and index
/// inside the transaction
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionInput {
    pub transaction_id: TransactionHash,
    pub index: BigInt,
}

impl IsPlutusData for TransactionInput {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.transaction_id.to_plutus_data(),
                self.index.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(TransactionInput {
                        transaction_id: TransactionHash::from_plutus_data(fields[0].clone())?,
                        index: BigInt::from_plutus_data(fields[1].clone())?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
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

/// 32-bytes blake2b256 hash of a transaction body.
///
/// Also known as Transaction ID or `TxID`.
/// Note: Plutus docs might incorrectly state that it uses SHA256.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionHash(pub LedgerBytes);

impl IsPlutusData for TransactionHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

/// An output of a transaction
///
/// This must include a target address, an amount, an optional datum and an optional reference
/// script
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionOutput {
    pub address: Address,
    pub datum_hash: DatumHash,
    pub value: Value,
}

impl IsPlutusData for TransactionOutput {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.address.to_plutus_data(),
                self.datum_hash.to_plutus_data(),
                self.value.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 3)?;
                    Ok(TransactionOutput {
                        address: Address::from_plutus_data(fields[0].clone())?,
                        datum_hash: DatumHash::from_plutus_data(fields[1].clone())?,
                        value: Value::from_plutus_data(fields[2].clone())?,
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

/// POSIX time is measured as the number of milliseconds since 1970-01-01T00:00:00Z
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct POSIXTime(pub BigInt);

impl IsPlutusData for POSIXTime {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

pub type POSIXTimeRange = PlutusInterval<POSIXTime>;

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
