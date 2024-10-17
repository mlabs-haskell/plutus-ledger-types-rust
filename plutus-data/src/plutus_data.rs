use num_bigint::BigInt;

#[cfg(feature = "lbf")]
use data_encoding::HEXLOWER;
#[cfg(feature = "lbf")]
use lbr_prelude::error::Error;
#[cfg(feature = "lbf")]
use lbr_prelude::json::{
    case_json_constructor, case_json_object, json_constructor, json_object, Json,
};

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
