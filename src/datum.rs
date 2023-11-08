use crate::crypto::LedgerBytes;
use crate::plutus_data::{
    verify_constr_fields, FromPlutusData, PlutusData, PlutusDataError, PlutusType, ToPlutusData,
};
#[cfg(feature = "lbf")]
use lbr_prelude::json::{self, Error, Json};
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Optional datum of a transaction
///
/// In case an inline datum is used, the data is embedded inside the transaction body, so it can be
/// directly retrieved. In case of a datum hash, an off-chain indexer is required to find the
/// associated datum by its hash.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputDatum {
    None,
    DatumHash(DatumHash),
    InlineDatum(Datum),
}

impl ToPlutusData for OutputDatum {
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            OutputDatum::None => PlutusData::Constr(BigInt::from(0), vec![]),
            OutputDatum::DatumHash(dat_hash) => {
                PlutusData::Constr(BigInt::from(1), vec![dat_hash.to_plutus_data()])
            }
            OutputDatum::InlineDatum(datum) => {
                PlutusData::Constr(BigInt::from(2), vec![datum.to_plutus_data()])
            }
        }
    }
}

impl FromPlutusData for OutputDatum {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(OutputDatum::None)
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(OutputDatum::DatumHash(DatumHash::from_plutus_data(
                        fields[0].clone(),
                    )?))
                }
                Ok(2) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(OutputDatum::InlineDatum(Datum::from_plutus_data(
                        fields[0].clone(),
                    )?))
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be between 0..2".to_owned(),
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

#[cfg(feature = "lbf")]
impl Json for OutputDatum {
    fn to_json(&self) -> Result<serde_json::Value, Error> {
        match self {
            OutputDatum::None => Ok(json::sum_constructor(
                "NoOutputDatum",
                Vec::with_capacity(0),
            )),
            OutputDatum::DatumHash(dat_hash) => Ok(json::sum_constructor(
                "OutputDatumHash",
                vec![dat_hash.to_json()?],
            )),
            OutputDatum::InlineDatum(datum) => {
                Ok(json::sum_constructor("OutputDatum", vec![datum.to_json()?]))
            }
        }
    }

    fn from_json(value: serde_json::Value) -> Result<Self, Error> {
        json::sum_parser(&value).and_then(|obj| match obj {
            ("NoOutputDatum", ctor_fields) => match &ctor_fields[..] {
                [] => Ok(OutputDatum::None),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 0,
                    got: ctor_fields.len(),
                }),
            },
            ("OutputDatumHash", ctor_fields) => match &ctor_fields[..] {
                [dat_hash] => Ok(OutputDatum::DatumHash(Json::from_json(dat_hash.clone())?)),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 1,
                    got: ctor_fields.len(),
                }),
            },
            ("OutputDatum", ctor_fields) => match &ctor_fields[..] {
                [datum] => Ok(OutputDatum::InlineDatum(Json::from_json(datum.clone())?)),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 1,
                    got: ctor_fields.len(),
                }),
            },
            _ => Err(Error::UnexpectedJsonInvariant {
                wanted: "constructor names (PubKeyCredential, OutputDatumHash, OutputDatum)"
                    .to_owned(),
                got: "unknown constructor name".to_owned(),
            }),
        })
    }
}

/// blake2b-256 hash of a datum
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DatumHash(pub LedgerBytes);

impl ToPlutusData for DatumHash {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for DatumHash {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Piece of information associated with a UTxO encoded into a PlutusData type.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Datum(pub PlutusData);

impl ToPlutusData for Datum {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.clone()
    }
}

impl FromPlutusData for Datum {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}
