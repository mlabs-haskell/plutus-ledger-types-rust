use num_bigint::BigInt;

use crate::plutus_data::{
    parse_constr_with_tag, parse_fixed_len_constr_fields, IsPlutusData, PlutusData, PlutusDataError,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Tuple<T, U>(pub (T, U));

impl<T: IsPlutusData, U: IsPlutusData> IsPlutusData for Tuple<T, U> {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0u32),
            vec![self.0 .0.to_plutus_data(), self.0 .1.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(data, 0)?;
        let [field_0, field_1] = parse_fixed_len_constr_fields(fields)?;

        Ok(Self((
            IsPlutusData::from_plutus_data(field_0)?,
            IsPlutusData::from_plutus_data(field_1)?,
        )))
    }
}
