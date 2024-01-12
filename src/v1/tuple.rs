#[cfg(feature = "lbf")]
use lbr_prelude::json::{json_array, Json};
use num_bigint::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::plutus_data::{
    parse_constr_with_tag, parse_fixed_len_constr_fields, IsPlutusData, PlutusData, PlutusDataError,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tuple<T, U>(pub T, pub U);

impl<T: IsPlutusData, U: IsPlutusData> IsPlutusData for Tuple<T, U> {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0u32),
            vec![self.0.to_plutus_data(), self.1.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(data, 0)?;
        let [field_0, field_1] = parse_fixed_len_constr_fields(fields)?;

        Ok(Self(
            IsPlutusData::from_plutus_data(field_0)?,
            IsPlutusData::from_plutus_data(field_1)?,
        ))
    }
}

#[cfg(feature = "lbf")]
impl<T: Json, U: Json> Json for Tuple<T, U> {
    fn to_json(&self) -> serde_json::Value {
        json_array(vec![self.0.to_json(), self.1.to_json()])
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, lbr_prelude::json::Error> {
        let vec: Vec<serde_json::Value> = Vec::from_json(value)?;
        let [k, v]: [serde_json::Value; 2] = TryFrom::try_from(vec).map_err(|vec: Vec<_>| {
            lbr_prelude::json::Error::UnexpectedArrayLength {
                got: vec.len(),
                wanted: 2,
                parser: "v1::tuple::Tuple".into(),
            }
        })?;
        Ok(Self(T::from_json(&k)?, U::from_json(&v)?))
    }
}
