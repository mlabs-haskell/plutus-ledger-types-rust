//! Types related to Cardano transactions.
use crate::plutus_data::{parse_constr_with_tag, parse_fixed_len_constr_fields};
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
#[cfg(feature = "chrono")]
pub use crate::v1::transaction::POSIXTimeConversionError;
pub use crate::v1::transaction::{
    DCert, POSIXTime, POSIXTimeRange, ScriptPurpose, TransactionHash, TransactionInput,
};
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    address::{Address, StakingCredential},
    assoc_map::AssocMap,
    crypto::PaymentPubKeyHash,
    datum::{Datum, DatumHash, OutputDatum},
    redeemer::Redeemer,
    script::ScriptHash,
    value::Value,
};

/// An output of a transaction
///
/// This must include the target address, an optional datum, an optional reference script, and the
/// amount of output tokens
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionOutput {
    pub address: Address,
    pub value: Value,
    pub datum: OutputDatum,
    pub reference_script: Option<ScriptHash>,
}

impl IsPlutusData for TransactionOutput {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.address.to_plutus_data(),
                self.value.to_plutus_data(),
                self.datum.to_plutus_data(),
                self.reference_script.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 4)?;
                    Ok(TransactionOutput {
                        address: Address::from_plutus_data(&fields[0])?,
                        value: Value::from_plutus_data(&fields[1])?,
                        datum: OutputDatum::from_plutus_data(&fields[2])?,
                        reference_script: <Option<ScriptHash>>::from_plutus_data(&fields[3])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(data),
            }),
        }
    }
}

/// An input of a pending transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TxInInfo {
    pub reference: TransactionInput,
    pub output: TransactionOutput,
}

impl From<(TransactionInput, TransactionOutput)> for TxInInfo {
    fn from((reference, output): (TransactionInput, TransactionOutput)) -> TxInInfo {
        TxInInfo { reference, output }
    }
}

impl IsPlutusData for TxInInfo {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.reference.to_plutus_data(),
                self.output.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(TxInInfo {
                        reference: TransactionInput::from_plutus_data(&fields[0])?,
                        output: TransactionOutput::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(data),
            }),
        }
    }
}

/// A pending transaction as seen by validator scripts, also known as TxInfo in Plutus
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionInfo {
    pub inputs: Vec<TxInInfo>,
    pub reference_inputs: Vec<TxInInfo>,
    pub outputs: Vec<TransactionOutput>,
    pub fee: Value,
    pub mint: Value,
    pub d_cert: Vec<DCert>,
    pub wdrl: AssocMap<StakingCredential, BigInt>,
    pub valid_range: POSIXTimeRange,
    pub signatories: Vec<PaymentPubKeyHash>,
    pub redeemers: AssocMap<ScriptPurpose, Redeemer>,
    pub datums: AssocMap<DatumHash, Datum>,
    pub id: TransactionHash,
}

impl IsPlutusData for TransactionInfo {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.inputs.to_plutus_data(),
                self.reference_inputs.to_plutus_data(),
                self.outputs.to_plutus_data(),
                self.fee.to_plutus_data(),
                self.mint.to_plutus_data(),
                self.d_cert.to_plutus_data(),
                self.wdrl.to_plutus_data(),
                self.valid_range.to_plutus_data(),
                self.signatories.to_plutus_data(),
                self.redeemers.to_plutus_data(),
                self.datums.to_plutus_data(),
                self.id.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(data, 0)?;
        let [inputs, reference_inputs, outputs, fee, mint, d_cert, wdrl, valid_range, signatories, redeemers, datums, id] =
            parse_fixed_len_constr_fields(fields)?;

        Ok(Self {
            inputs: IsPlutusData::from_plutus_data(inputs)?,
            reference_inputs: IsPlutusData::from_plutus_data(reference_inputs)?,
            outputs: IsPlutusData::from_plutus_data(outputs)?,
            fee: IsPlutusData::from_plutus_data(fee)?,
            mint: IsPlutusData::from_plutus_data(mint)?,
            d_cert: IsPlutusData::from_plutus_data(d_cert)?,
            wdrl: IsPlutusData::from_plutus_data(wdrl)?,
            valid_range: IsPlutusData::from_plutus_data(valid_range)?,
            signatories: IsPlutusData::from_plutus_data(signatories)?,
            redeemers: IsPlutusData::from_plutus_data(redeemers)?,
            datums: IsPlutusData::from_plutus_data(datums)?,
            id: IsPlutusData::from_plutus_data(id)?,
        })
    }
}

/// The context that is presented to the currently-executing script.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ScriptContext {
    pub purpose: ScriptPurpose,
    pub tx_info: TransactionInfo,
}

impl IsPlutusData for ScriptContext {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![self.purpose.to_plutus_data(), self.tx_info.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(data, 0)?;
        let [purpose, tx_info] = parse_fixed_len_constr_fields(fields)?;

        Ok(Self {
            purpose: IsPlutusData::from_plutus_data(purpose)?,
            tx_info: IsPlutusData::from_plutus_data(tx_info)?,
        })
    }
}
