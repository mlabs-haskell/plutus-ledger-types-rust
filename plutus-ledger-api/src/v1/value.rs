//! Types related to Cardano values, such as Ada and native tokens.
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};

use crate::v1::crypto::LedgerBytes;
use crate::v1::script::{MintingPolicyHash, ScriptHash};
#[cfg(feature = "lbf")]
use lbr_prelude::json::{Error, Json, JsonType};
use num_bigint::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "lbf")]
use serde_json;
use std::collections::BTreeMap;

/// Identifier of a currency, which could be either Ada (or tAda), or a native token represented by
/// it's minting policy hash. A currency may be associated with multiple `AssetClass`es.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurrencySymbol {
    Ada,
    NativeToken(MintingPolicyHash),
}

impl IsPlutusData for CurrencySymbol {
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            CurrencySymbol::Ada => String::from("").to_plutus_data(),
            CurrencySymbol::NativeToken(policy_hash) => policy_hash.to_plutus_data(),
        }
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).and_then(|bytes: LedgerBytes| {
            if bytes.0.is_empty() {
                Ok(CurrencySymbol::Ada)
            } else {
                Ok(CurrencySymbol::NativeToken(MintingPolicyHash(ScriptHash(
                    bytes,
                ))))
            }
        })
    }
}

#[cfg(feature = "lbf")]
impl Json for CurrencySymbol {
    fn to_json(&self) -> serde_json::Value {
        match self {
            CurrencySymbol::Ada => serde_json::Value::String(String::new()),
            CurrencySymbol::NativeToken(policy_hash) => policy_hash.to_json(),
        }
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, Error> {
        match value.clone() {
            serde_json::Value::String(str) => {
                if str.is_empty() {
                    Ok(CurrencySymbol::Ada)
                } else {
                    Ok(CurrencySymbol::NativeToken(Json::from_json(value)?))
                }
            }
            _ => Err(Error::UnexpectedJsonType {
                wanted: JsonType::String,
                got: JsonType::from(value),
                parser: "Plutus.V1.CurrencySymbol".to_owned(),
            }),
        }
    }
}

/// A value that can contain multiple asset classes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Value(pub BTreeMap<CurrencySymbol, BTreeMap<TokenName, BigInt>>);

impl Value {
    pub fn new() -> Self {
        Value(BTreeMap::new())
    }
}

impl IsPlutusData for Value {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Name of a token. This can be any arbitrary bytearray
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TokenName(pub LedgerBytes);

impl TokenName {
    pub fn ada() -> TokenName {
        TokenName(LedgerBytes(Vec::with_capacity(0)))
    }
}

impl IsPlutusData for TokenName {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

/// AssetClass is uniquely identifying a specific asset
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct AssetClass {
    pub currency_symbol: CurrencySymbol,
    pub token_name: TokenName,
}

impl IsPlutusData for AssetClass {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.currency_symbol.to_plutus_data(),
                self.token_name.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(AssetClass {
                        currency_symbol: CurrencySymbol::from_plutus_data(&fields[0])?,
                        token_name: TokenName::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
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
