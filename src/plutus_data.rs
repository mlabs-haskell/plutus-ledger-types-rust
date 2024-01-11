//! Plutus Data related types and traits
#[cfg(feature = "lbf")]
use data_encoding::HEXLOWER;
#[cfg(feature = "lbf")]
use lbr_prelude::error::Error;
#[cfg(feature = "lbf")]
use lbr_prelude::json::{
    case_json_constructor, case_json_object, json_constructor, json_object, Json,
};
use num_bigint::BigInt;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::{empty, once};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Data representation of on-chain data such as Datums and Redeemers
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PlutusData {
    Constr(BigInt, Vec<PlutusData>),
    Map(Vec<(PlutusData, PlutusData)>),
    List(Vec<PlutusData>),
    Integer(BigInt),
    Bytes(Vec<u8>),
}

impl PlutusData {
    pub fn constr(tag: u32, fields: Vec<PlutusData>) -> Self {
        PlutusData::Constr(BigInt::from(tag), fields)
    }

    pub fn map(fields: Vec<(PlutusData, PlutusData)>) -> Self {
        PlutusData::Map(fields)
    }

    pub fn list(fields: Vec<PlutusData>) -> Self {
        PlutusData::List(fields)
    }

    pub fn integer(value: u32) -> Self {
        PlutusData::Integer(BigInt::from(value))
    }

    pub fn bytes(value: Vec<u8>) -> Self {
        PlutusData::Bytes(value)
    }
}

#[cfg(feature = "lbf")]
impl Json for PlutusData {
    fn to_json(&self) -> serde_json::Value {
        match self {
            PlutusData::Constr(index, fields) => json_constructor(
                "Constr",
                vec![json_object(vec![
                    ("index".to_string(), index.to_json()),
                    ("fields".to_string(), fields.to_json()),
                ])],
            ),
            PlutusData::Map(map) => json_constructor("Map", vec![map.to_json()]),
            PlutusData::List(list) => json_constructor("List", vec![list.to_json()]),
            PlutusData::Integer(int) => json_constructor("Integer", vec![int.to_json()]),
            PlutusData::Bytes(bytes) => {
                json_constructor("Bytes", vec![String::to_json(&HEXLOWER.encode(bytes))])
            }
        }
    }

    fn from_json(value: &serde_json::Value) -> Result<PlutusData, Error> {
        case_json_constructor(
            "PlutusV1.PlutusData",
            vec![
                (
                    "Constr",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [val] => case_json_object(
                            |obj| {
                                let index = obj.get("index").ok_or(Error::UnexpectedFieldName {
                                    wanted: "index".to_owned(),
                                    got: obj.keys().cloned().collect(),
                                    parser: "PlutusV1.PlutusData".to_owned(),
                                })?;

                                let fields =
                                    obj.get("fields").ok_or(Error::UnexpectedFieldName {
                                        wanted: "fields".to_owned(),
                                        got: obj.keys().cloned().collect(),
                                        parser: "PlutusV1.PlutusData".to_owned(),
                                    })?;
                                Ok(PlutusData::Constr(
                                    BigInt::from_json(index)?,
                                    <Vec<PlutusData>>::from_json(fields)?,
                                ))
                            },
                            val,
                        ),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "PlutusV1.PlutusData".to_owned(),
                        }),
                    }),
                ),
                (
                    "Map",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [val] => Ok(PlutusData::Map(Json::from_json(val)?)),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "PlutusV1.PlutusData".to_owned(),
                        }),
                    }),
                ),
                (
                    "List",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [val] => Ok(PlutusData::List(Json::from_json(val)?)),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "PlutusV1.PlutusData".to_owned(),
                        }),
                    }),
                ),
                (
                    "Integer",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [val] => Ok(PlutusData::Integer(Json::from_json(val)?)),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "PlutusV1.PlutusData".to_owned(),
                        }),
                    }),
                ),
                (
                    "Bytes",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [val] => {
                            let bytes = String::from_json(val).and_then(|str| {
                                HEXLOWER.decode(&str.into_bytes()).map_err(|_| {
                                    Error::UnexpectedJsonInvariant {
                                        wanted: "base16 string".to_owned(),
                                        got: "unexpected string".to_owned(),
                                        parser: "Plutus.V1.Bytes".to_owned(),
                                    }
                                })
                            })?;
                            Ok(PlutusData::Bytes(bytes))
                        }
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "PlutusV1.PlutusData".to_owned(),
                        }),
                    }),
                ),
            ],
            value,
        )
    }
}

/// Deserialise a Plutus data using parsers for each variant
pub fn case_plutus_data<'a, T>(
    ctor_case: impl FnOnce(&'a BigInt) -> Box<dyn 'a + FnOnce(&'a Vec<PlutusData>) -> T>,
    list_case: impl FnOnce(&'a Vec<PlutusData>) -> T,
    int_case: impl FnOnce(&'a BigInt) -> T,
    other_case: impl FnOnce(&'a PlutusData) -> T,
    pd: &'a PlutusData,
) -> T {
    match pd {
        PlutusData::Constr(tag, args) => ctor_case(&tag)(&args),
        PlutusData::List(args) => list_case(&args),
        PlutusData::Integer(i) => int_case(&i),
        other => other_case(&other),
    }
}

pub trait IsPlutusData {
    fn to_plutus_data(&self) -> PlutusData;

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError>
    where
        Self: Sized;
}

#[derive(Clone, Debug)]
pub enum PlutusType {
    Constr,
    Map,
    List,
    Integer,
    Bytes,
}

impl From<&PlutusData> for PlutusType {
    fn from(plutus_data: &PlutusData) -> Self {
        match plutus_data {
            PlutusData::Constr(_, _) => PlutusType::Constr,
            PlutusData::Map(_) => PlutusType::Map,
            PlutusData::List(_) => PlutusType::List,
            PlutusData::Integer(_) => PlutusType::Integer,
            PlutusData::Bytes(_) => PlutusType::Bytes,
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum PlutusDataError {
    #[error("Expected a PlutusData type {wanted:?}, but got {got:?}")]
    UnexpectedPlutusType { got: PlutusType, wanted: PlutusType },
    #[error("Expected a PlutusData type as {wanted:?}, but got {got:?}")]
    UnexpectedPlutusInvariant { got: String, wanted: String },
    #[error("Expected a Plutus List with {wanted:?} elements, but got {got:?} elements")]
    UnexpectedListLength { got: usize, wanted: usize },
    #[error("Some internal error happened: {0}")]
    InternalError(String),
}

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

impl IsPlutusData for bool {
    fn to_plutus_data(&self) -> PlutusData {
        if *self {
            PlutusData::Constr(BigInt::from(1), Vec::with_capacity(0))
        } else {
            PlutusData::Constr(BigInt::from(0), Vec::with_capacity(0))
        }
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(false)
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(true)
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
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
                wanted: PlutusType::Integer,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl<T> IsPlutusData for Option<T>
where
    T: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            Some(val) => PlutusData::Constr(BigInt::from(0), vec![val.to_plutus_data()]),
            None => PlutusData::Constr(BigInt::from(1), Vec::with_capacity(0)),
        }
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Some(T::from_plutus_data(&fields[0])?))
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(None)
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl<T, E> IsPlutusData for Result<T, E>
where
    T: IsPlutusData,
    E: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            Err(val) => PlutusData::Constr(BigInt::from(0), vec![val.to_plutus_data()]),
            Ok(val) => PlutusData::Constr(BigInt::from(1), vec![val.to_plutus_data()]),
        }
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Err(E::from_plutus_data(&fields[0])?))
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Ok(T::from_plutus_data(&fields[0])?))
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(plutus_data),
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
        match plutus_data {
            PlutusData::List(vec) => vec
                .iter()
                .map(|val| T::from_plutus_data(val))
                .collect::<Result<Vec<T>, PlutusDataError>>(),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::List,
                got: PlutusType::from(plutus_data),
            }),
        }
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
                .into_iter()
                .map(|val| T::from_plutus_data(val))
                .collect::<Result<Self, PlutusDataError>>(),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Map,
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
                .into_iter()
                .map(|(key, val)| Ok((K::from_plutus_data(key)?, V::from_plutus_data(val)?)))
                .collect::<Result<Self, PlutusDataError>>(),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Map,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl IsPlutusData for () {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(BigInt::from(0), Vec::with_capacity(0))
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(())
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

impl IsPlutusData for PlutusData {
    fn to_plutus_data(&self) -> PlutusData {
        self.clone()
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        Ok(plutus_data.clone())
    }
}

impl<A, B> IsPlutusData for (A, B)
where
    A: IsPlutusData,
    B: IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![self.0.to_plutus_data(), self.1.to_plutus_data()],
        )
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        match plutus_data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok((
                        A::from_plutus_data(&fields[0])?,
                        B::from_plutus_data(&fields[1])?,
                    ))
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(plutus_data),
            }),
        }
    }
}

/// Verify the number of fields contained in a PlutusData::Constr
pub fn verify_constr_fields(
    fields: &Vec<PlutusData>,
    expected: usize,
) -> Result<(), PlutusDataError> {
    if fields.len() != expected {
        Err(PlutusDataError::UnexpectedPlutusInvariant {
            wanted: format!("Constr with {} fields", expected),
            got: format!("{:?}", fields),
        })
    } else {
        Ok(())
    }
}

pub fn parse_fixed_len_constr_fields<'a, const LEN: usize>(
    v: &'a [PlutusData],
) -> Result<&'a [PlutusData; LEN], PlutusDataError> {
    v.try_into()
        .map_err(|_| PlutusDataError::UnexpectedListLength {
            got: v.len(),
            wanted: LEN,
        })
}

pub fn parse_constr<'a>(
    data: &'a PlutusData,
) -> Result<(u32, &'a Vec<PlutusData>), PlutusDataError> {
    match data {
        PlutusData::Constr(tag, fields) => u32::try_from(tag)
            .map_err(|err| PlutusDataError::UnexpectedPlutusInvariant {
                got: err.to_string(),
                wanted: "Constr bigint tag within u32 range".into(),
            })
            .map(|tag| (tag, fields)),
        _ => Err(PlutusDataError::UnexpectedPlutusType {
            wanted: PlutusType::Constr,
            got: PlutusType::from(data),
        }),
    }
}

pub fn parse_constr_with_tag<'a>(
    data: &'a PlutusData,
    expected_tag: u32,
) -> Result<&'a Vec<PlutusData>, PlutusDataError> {
    let (tag, fields) = parse_constr(data)?;

    if tag != expected_tag {
        Err(PlutusDataError::UnexpectedPlutusInvariant {
            got: tag.to_string(),
            wanted: format!("Constr tag to be: {}", expected_tag),
        })
    } else {
        Ok(fields)
    }
}

pub fn singleton<T, C>(value: T) -> C
where
    C: FromIterator<T>,
{
    once(value).into_iter().collect()
}

pub fn none<T, C>() -> C
where
    C: FromIterator<T>,
{
    empty::<T>().into_iter().collect()
}
