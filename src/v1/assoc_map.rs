use crate::plutus_data::{IsPlutusData, PlutusData, PlutusDataError, PlutusType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocMap<K, V>(pub Vec<(K, V)>);

impl<K: IsPlutusData, V: IsPlutusData> IsPlutusData for AssocMap<K, V> {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Map(
            (&self.0)
                .into_iter()
                .map(|(k, v)| (k.to_plutus_data(), v.to_plutus_data()))
                .collect(),
        )
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Map(pairs) => pairs
                .into_iter()
                .map(|(k, v)| Ok((K::from_plutus_data(k)?, V::from_plutus_data(v)?)))
                .collect::<Result<Vec<(K, V)>, PlutusDataError>>()
                .map(Self),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                got: From::from(plutus_data),
                wanted: PlutusType::Map,
            }),
        }
    }
}
