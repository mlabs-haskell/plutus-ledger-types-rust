//! Types related to Cardano values, such as Ada and native tokens.
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
use crate::utils::{singleton, union_btree_maps_with};
use crate::v1::crypto::LedgerBytes;
use crate::v1::script::{MintingPolicyHash, ScriptHash};
#[cfg(feature = "lbf")]
use lbr_prelude::json::{Error, Json, JsonType};
use num_bigint::BigInt;
use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "lbf")]
use serde_json;
use std::ops;
use std::string::String;
use std::{
    collections::BTreeMap,
    iter::Sum,
    ops::{Add, Mul, Neg, Not, Sub},
};

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
    /// Create a Value containing only ada tokens, given the quantity in lovelace.
    pub fn ada_value(amount: &BigInt) -> Self {
        Self::token_value(&CurrencySymbol::Ada, &TokenName::ada(), amount)
    }

    /// Create a Value containing only the given quantity of the given token.
    pub fn token_value(cs: &CurrencySymbol, tn: &TokenName, amount: &BigInt) -> Self {
        Value(singleton((
            cs.clone(),
            singleton((tn.clone(), amount.clone())),
        )))
    }

    /// Lookup the quantity of the given token.
    pub fn get_token_amount(&self, cs: &CurrencySymbol, tn: &TokenName) -> BigInt {
        self.0
            .get(cs)
            .and_then(|tn_map| tn_map.get(&tn))
            .map_or(BigInt::zero(), Clone::clone)
    }

    /// Lookup the quantity of ada(unit: lovelace).
    pub fn get_ada_amount(&self) -> BigInt {
        self.get_token_amount(&CurrencySymbol::Ada, &TokenName::ada())
    }

    /// Insert a new token into the value, or replace the existing quantity.
    pub fn insert_token(&self, cs: &CurrencySymbol, tn: &TokenName, a: &BigInt) -> Self {
        let mut result_map = self.0.clone();

        result_map
            .entry(cs.clone())
            .and_modify(|tn_map| {
                tn_map
                    .entry(tn.clone())
                    .and_modify(|old_a| {
                        *old_a = a.clone();
                    })
                    .or_insert_with(|| a.clone());
            })
            .or_insert_with(|| singleton((tn.clone(), a.clone())));

        Self(result_map)
    }

    /// Return true if the value don't have any entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Remove all tokens whose quantity is zero.
    pub fn normalize(self) -> Self {
        self.filter(|_, _, a| a.is_zero().not())
    }

    /// Apply a function to each token of the value, and use its result as the new amount.
    pub fn map_amount<F>(self, mut f: F) -> Self
    where
        F: FnMut(&CurrencySymbol, &TokenName, &BigInt) -> BigInt,
    {
        self.filter_map_amount(|cs, tn, a| Some(f(cs, tn, a)))
    }

    /// Apply a predicate to tokens.
    pub fn filter<F>(self, mut f: F) -> Self
    where
        F: FnMut(&CurrencySymbol, &TokenName, &BigInt) -> bool,
    {
        self.filter_map_amount(|cs, tn, a| f(cs, tn, a).then(|| a.clone()))
    }

    /// Apply a function to each token of the value. If the result is None, the token entry will be
    /// removed.
    ///
    /// Note that if the name-quantity map of any given currency symbols is empty, the currency entry
    /// will be removed from the top-level map entirely.
    pub fn filter_map_amount<F>(self, mut f: F) -> Self
    where
        F: FnMut(&CurrencySymbol, &TokenName, &BigInt) -> Option<BigInt>,
    {
        Value(
            (self.0)
                .into_iter()
                .filter_map(|(cs, tn_map)| {
                    let filtered_tn_map = tn_map
                        .into_iter()
                        .filter_map(|(tn, a)| f(&cs, &tn, &a).map(|a| (tn, a)))
                        .collect::<BTreeMap<TokenName, BigInt>>();

                    if filtered_tn_map.is_empty() {
                        None
                    } else {
                        Some((cs.clone(), filtered_tn_map))
                    }
                })
                .collect(),
        )
    }
}

impl Default for Value {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl Zero for Value {
    fn zero() -> Self {
        Default::default()
    }

    fn is_zero(&self) -> bool {
        self.is_empty()
    }
}

impl_op!(+ |a: &Value, b: &Value| -> Value { a.clone() + b.clone() });
impl_op!(+ |a: &Value, b: Value| -> Value { a.clone() + b });
impl_op!(+ |a: Value, b: &Value| -> Value { a + b.clone() });

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        Value(union_btree_maps_with(
            |lhs, rhs| union_btree_maps_with(|lhs, rhs| lhs + rhs, lhs, rhs),
            self.0,
            rhs.0,
        ))
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        self.map_amount(|_, _, a| a.neg())
    }
}

impl Neg for &Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        self.clone().neg()
    }
}

impl_op!(-|a: &Value, b: &Value| -> Value { a.clone() - b.clone() });
impl_op!(-|a: &Value, b: Value| -> Value { a.clone() - b });
impl_op!(-|a: Value, b: &Value| -> Value { a - b.clone() });

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl_op_commutative!(*|a: Value, b: BigInt| -> Value { &a * &b });
impl_op_commutative!(*|a: &Value, b: BigInt| -> Value { a * &b });
impl_op_commutative!(*|a: Value, b: &BigInt| -> Value { &a * b });

impl_op_commutative!(*|a: &Value, b: i8| -> Value { a * BigInt::from(b) });
impl_op_commutative!(*|a: &Value, b: i16| -> Value { a * BigInt::from(b) });
impl_op_commutative!(*|a: &Value, b: i32| -> Value { a * BigInt::from(b) });
impl_op_commutative!(*|a: &Value, b: i64| -> Value { a * BigInt::from(b) });

impl_op_commutative!(*|a: &Value, b: u8| -> Value { a * BigInt::from(b) });
impl_op_commutative!(*|a: &Value, b: u16| -> Value { a * BigInt::from(b) });
impl_op_commutative!(*|a: &Value, b: u32| -> Value { a * BigInt::from(b) });
impl_op_commutative!(*|a: &Value, b: u64| -> Value { a * BigInt::from(b) });

impl Mul<&BigInt> for &Value {
    type Output = Value;

    fn mul(self, rhs: &BigInt) -> Self::Output {
        Value(
            (&self.0)
                .into_iter()
                .map(|(cs, tn_map)| {
                    (
                        cs.clone(),
                        tn_map
                            .into_iter()
                            .map(|(tn, q)| (tn.clone(), q * rhs))
                            .collect(),
                    )
                })
                .collect(),
        )
    }
}

impl Sum<Value> for Value {
    fn sum<I: Iterator<Item = Value>>(iter: I) -> Self {
        iter.fold(Zero::zero(), Add::add)
    }
}

impl<'a> Sum<&'a Value> for Value {
    fn sum<I: Iterator<Item = &'a Value>>(iter: I) -> Self {
        iter.fold(Zero::zero(), Add::add)
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

impl TokenName {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        TokenName(LedgerBytes(bytes))
    }

    pub fn from_string(str: &str) -> Self {
        TokenName(LedgerBytes(String::from(str).into_bytes()))
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
