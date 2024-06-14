//! Types related to Plutus Datums
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
pub use crate::v1::datum::{Datum, DatumHash};
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Optional datum of a transaction
///
/// In case an inline datum is used, the data is embedded inside the transaction body, so it can be
/// directly retrieved. In case of a datum hash, an off-chain indexer is required to find the
/// associated datum by its hash.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputDatum {
    None,
    DatumHash(DatumHash),
    InlineDatum(Datum),
}

impl IsPlutusData for OutputDatum {
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

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(OutputDatum::None)
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(OutputDatum::DatumHash(DatumHash::from_plutus_data(
                        &fields[0],
                    )?))
                }
                Ok(2) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(OutputDatum::InlineDatum(Datum::from_plutus_data(
                        &fields[0],
                    )?))
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be between 0..2".to_owned(),
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
