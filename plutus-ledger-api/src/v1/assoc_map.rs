use std::hash::Hash;

#[cfg(feature = "lbf")]
use lbr_prelude::json::{json_array, Json};
use linked_hash_map::LinkedHashMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::plutus_data::{IsPlutusData, PlutusData, PlutusDataError, PlutusType};

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssocMap<K, V>(pub Vec<(K, V)>);

impl<K, V> AssocMap<K, V> {
    pub fn new() -> Self {
        AssocMap(Vec::new())
    }
}

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

impl<K, V> From<Vec<(K, V)>> for AssocMap<K, V> {
    fn from(vec: Vec<(K, V)>) -> Self {
        AssocMap(vec)
    }
}

impl<K, V> From<AssocMap<K, V>> for Vec<(K, V)> {
    fn from(m: AssocMap<K, V>) -> Self {
        m.0
    }
}

impl<K: Hash + Eq, V> From<AssocMap<K, V>> for LinkedHashMap<K, V> {
    fn from(m: AssocMap<K, V>) -> Self {
        m.0.into_iter().collect()
    }
}

impl<K: Hash + Eq, V> From<LinkedHashMap<K, V>> for AssocMap<K, V> {
    fn from(value: LinkedHashMap<K, V>) -> Self {
        AssocMap(value.into_iter().collect())
    }
}

#[cfg(feature = "lbf")]
impl<K: Json, V: Json> Json for AssocMap<K, V> {
    fn to_json(&self) -> serde_json::Value {
        json_array(
            (&self.0)
                .into_iter()
                .map(|(k, v)| json_array(vec![k.to_json(), v.to_json()]))
                .collect(),
        )
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, lbr_prelude::json::Error> {
        let vec_of_vectors: Vec<Vec<serde_json::Value>> = Json::from_json(value)?;
        let vec_of_pairs = vec_of_vectors
            .into_iter()
            .map(|vec| {
                let [k, v]: [serde_json::Value; 2] =
                    TryFrom::try_from(vec).map_err(|vec: Vec<_>| {
                        lbr_prelude::json::Error::UnexpectedArrayLength {
                            got: vec.len(),
                            wanted: 2,
                            parser: "v1::assoc_map::AssocMap".into(),
                        }
                    })?;

                let k = K::from_json(&k)?;
                let v = V::from_json(&v)?;

                Ok((k, v))
            })
            .collect::<Result<Vec<(K, V)>, _>>()?;

        Ok(Self(vec_of_pairs))
    }
}
