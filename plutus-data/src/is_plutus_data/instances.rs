use std::collections::{BTreeMap, BTreeSet};

use num_bigint::BigInt;

use crate::{IsPlutusData, PlutusData, PlutusDataError, PlutusType};

use super::aux::{parse_constr, parse_constr_with_tag, parse_fixed_len_constr_fields, parse_list};

impl IsPlutusData for PlutusData {
    fn to_plutus_data(&self) -> PlutusData {
        self.clone()
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        Ok(plutus_data.clone())
    }
}

// MARK: Orphan Instances

impl IsPlutusData for BigInt {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Integer(self.clone())
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Integer(int) => Ok(int.clone()),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Integer,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl IsPlutusData for Vec<u8> {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Bytes(self.clone())
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Bytes(bytes) => Ok(bytes.clone()),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Bytes,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

const BOOL_FALSE_TAG: u32 = 0;
const BOOL_TRUE_TAG: u32 = 1;

impl IsPlutusData for bool {
    fn to_plutus_data(&self) -> PlutusData {
        if *self {
            PlutusData::Constr(BOOL_TRUE_TAG.into(), vec![])
        } else {
            PlutusData::Constr(BOOL_FALSE_TAG.into(), vec![])
        }
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let (tag, fields) = parse_constr(plutus_data)?;
        let [] = parse_fixed_len_constr_fields::<0>(fields)?;
        match tag {
            BOOL_TRUE_TAG => Ok(true),
            BOOL_FALSE_TAG => Ok(false),
            _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                wanted: format!("Constr with tag {BOOL_TRUE_TAG} or {BOOL_FALSE_TAG}"),
                got: tag.to_string(),
            }),
        }
    }
}

impl IsPlutusData for String {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Bytes(self.as_bytes().into())
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Bytes(bytes) => String::from_utf8(bytes.clone()).map_err(|err| {
                PlutusDataError::InternalError(format!(
                    "Couldn't convert Plutus bytes to String: {:?}",
                    err
                ))
            }),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Bytes,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl IsPlutusData for char {
    fn to_plutus_data(&self) -> PlutusData {
        String::from(*self).to_plutus_data()
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        String::from_plutus_data(plutus_data).and_then(|str| {
            let mut chars = str.chars();
            let ch = chars.next();
            let rest = chars.next();
            match (ch, rest) {
                (Some(ch), None) => Ok(ch),
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    got: "string".to_owned(),
                    wanted: "char".to_owned(),
                }),
            }
        })
    }
}

const OPTION_SOME_TAG: u32 = 0;
const OPTION_NONE_TAG: u32 = 1;

impl<T> IsPlutusData for Option<T>
where
    T: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            Some(val) => PlutusData::Constr(OPTION_SOME_TAG.into(), vec![val.to_plutus_data()]),
            None => PlutusData::Constr(OPTION_NONE_TAG.into(), vec![]),
        }
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let (tag, fields) = parse_constr(plutus_data)?;

        match tag {
            OPTION_SOME_TAG => {
                let [data] = parse_fixed_len_constr_fields::<1>(fields)?;
                Ok(Some(T::from_plutus_data(data)?))
            }
            OPTION_NONE_TAG => {
                let [] = parse_fixed_len_constr_fields::<0>(fields)?;
                Ok(None)
            }
            _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                wanted: format!("Constr with tag {OPTION_SOME_TAG} or {OPTION_NONE_TAG}"),
                got: tag.to_string(),
            }),
        }
    }
}

const RESULT_ERR_TAG: u32 = 0;
const RESULT_OK_TAG: u32 = 1;

impl<T, E> IsPlutusData for Result<T, E>
where
    T: IsPlutusData,
    E: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            Err(err) => PlutusData::Constr(RESULT_ERR_TAG.into(), vec![err.to_plutus_data()]),
            Ok(val) => PlutusData::Constr(RESULT_OK_TAG.into(), vec![val.to_plutus_data()]),
        }
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let (tag, fields) = parse_constr(plutus_data)?;
        let [field] = parse_fixed_len_constr_fields::<1>(fields)?;

        match tag {
            RESULT_ERR_TAG => Ok(Err(E::from_plutus_data(field)?)),
            RESULT_OK_TAG => Ok(Ok(T::from_plutus_data(field)?)),
            _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                wanted: format!("Constr with tag {RESULT_ERR_TAG} or {RESULT_OK_TAG}"),
                got: tag.to_string(),
            }),
        }
    }
}

impl<T> IsPlutusData for Vec<T>
where
    T: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        let values = self
            .iter()
            .map(|val| val.to_plutus_data())
            .collect::<Vec<PlutusData>>();

        PlutusData::List(values)
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let list = parse_list(plutus_data)?;
        list.iter().map(T::from_plutus_data).collect()
    }
}

impl<T> IsPlutusData for BTreeSet<T>
where
    T: IsPlutusData + Eq + Ord,
{
    fn to_plutus_data(&self) -> PlutusData {
        let set = self
            .iter()
            .map(|val| val.to_plutus_data())
            .collect::<Vec<PlutusData>>();

        PlutusData::List(set)
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::List(vec) => vec
                .iter()
                .map(|val| T::from_plutus_data(val))
                .collect::<Result<Self, PlutusDataError>>(),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::List,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl<K, V> IsPlutusData for BTreeMap<K, V>
where
    K: IsPlutusData + Eq + Ord,
    V: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        let assoc_map = self
            .iter()
            .map(|(key, val)| (key.to_plutus_data(), val.to_plutus_data()))
            .collect::<Vec<(PlutusData, PlutusData)>>();

        PlutusData::Map(assoc_map)
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Map(dict) => dict
                .iter()
                .map(|(key, val)| Ok((K::from_plutus_data(key)?, V::from_plutus_data(val)?)))
                .collect::<Result<Self, PlutusDataError>>(),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Map,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

const UNIT_TAG: u32 = 0;

impl IsPlutusData for () {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(UNIT_TAG.into(), vec![])
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(plutus_data, UNIT_TAG)?;
        let [] = parse_fixed_len_constr_fields::<0>(fields)?;
        Ok(())
    }
}

const PAIR_TAG: u32 = 0;

impl<A, B> IsPlutusData for (A, B)
where
    A: IsPlutusData,
    B: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(PAIR_TAG),
            vec![self.0.to_plutus_data(), self.1.to_plutus_data()],
        )
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(plutus_data, PAIR_TAG)?;
        let [a, b] = parse_fixed_len_constr_fields::<2>(fields)?;
        Ok((A::from_plutus_data(a)?, B::from_plutus_data(b)?))
    }
}
