//! Types related to Cardano values, such as Ada and native tokens.

use std::string::String;
use std::{
    collections::BTreeMap,
    iter::Sum,
    ops::{Add, Mul, Neg, Not, Sub},
};
use std::{fmt, ops};

use cardano_serialization_lib as csl;
#[cfg(feature = "lbf")]
use lbr_prelude::json::{Error, Json, JsonType};
use num_bigint::BigInt;
use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "lbf")]
use serde_json;

use crate as plutus_ledger_api;
use crate::csl::csl_to_pla::FromCSL;
use crate::csl::pla_to_csl::{TryFromPLA, TryFromPLAError, TryToCSL};
use crate::plutus_data::{IsPlutusData, PlutusData, PlutusDataError};
use crate::utils::aux::{singleton, union_b_tree_maps_with, union_btree_maps_with};
use crate::v1::crypto::LedgerBytes;
use crate::v1::script::{MintingPolicyHash, ScriptHash};

////////////////////
// CurrencySymbol //
////////////////////

/// Identifier of a currency, which could be either Ada (or tAda), or a native token represented by
/// it's minting policy hash. A currency may be associated with multiple `AssetClass`es.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurrencySymbol {
    Ada,
    NativeToken(MintingPolicyHash),
}

impl fmt::Display for CurrencySymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let asset_class_str = match self {
            CurrencySymbol::Ada => {
                if f.alternate() {
                    "lovelace".to_string()
                } else {
                    "".to_string()
                }
            }
            CurrencySymbol::NativeToken(symbol) => {
                format!("{}", symbol.0 .0)
            }
        };

        write!(f, "{}", asset_class_str)
    }
}

impl IsPlutusData for CurrencySymbol {
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            CurrencySymbol::Ada => String::from("").to_plutus_data(),
            CurrencySymbol::NativeToken(policy_hash) => policy_hash.to_plutus_data(),
        }
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(|bytes: LedgerBytes| {
            if bytes.0.is_empty() {
                CurrencySymbol::Ada
            } else {
                CurrencySymbol::NativeToken(MintingPolicyHash(ScriptHash(bytes)))
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

///////////
// Value //
///////////

/// A value that can contain multiple asset classes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Value(pub BTreeMap<CurrencySymbol, BTreeMap<TokenName, BigInt>>);

#[cfg(feature = "serde")]
mod value_serde {
    use std::collections::BTreeMap;

    use num_bigint::BigInt;
    use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

    use super::{CurrencySymbol, TokenName, Value};

    struct Assets(BTreeMap<TokenName, BigInt>);

    impl Serialize for Value {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(
                self.0
                    .iter()
                    .map(|(cur_sym, assets)| (cur_sym, Assets(assets.to_owned()))),
            )
        }
    }

    impl<'de> Deserialize<'de> for Value {
        fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<(CurrencySymbol, Assets)> = Vec::deserialize(deserializer)?;

            Ok(Value(
                vec.into_iter().map(|(cs, assets)| (cs, assets.0)).collect(),
            ))
        }
    }

    impl Serialize for Assets {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(self.0.iter())
        }
    }

    impl<'de> Deserialize<'de> for Assets {
        fn deserialize<D>(deserializer: D) -> Result<Assets, D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<(TokenName, BigInt)> = Vec::deserialize(deserializer)?;

            Ok(Assets(vec.into_iter().collect()))
        }
    }
}

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
            .and_then(|tn_map| tn_map.get(tn))
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
                        old_a.clone_from(a);
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

    pub fn is_subset(&self, b: &Value) -> bool {
        (b - self)
            .clone()
            .normalize()
            // Has negative entries?
            .filter(|_, _, amount| amount < &BigInt::from(0u32))
            .is_empty()
    }

    pub fn is_pure_ada(self) -> bool {
        let inner = self.normalize().0;
        let inner: Vec<_> = inner.into_iter().collect();

        match inner.as_slice() {
            [] => true,
            [(cs, _)] => cs == &CurrencySymbol::Ada,
            _ => false,
        }
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = self
            .0
            .iter()
            .flat_map(|(currency_symbol, assets)| {
                assets.iter().map(move |(token_name, amount)| {
                    if token_name.is_empty() {
                        format!("{} {}", currency_symbol, amount)
                    } else {
                        format!("{}.{} {}", currency_symbol, token_name, amount)
                    }
                })
            })
            .collect::<Vec<_>>()
            .join("+");

        write!(f, "{}", value_str)
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

impl_op!(*|a: &BigInt, b: &Value| -> Value { b * a });
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
            self.0
                .iter()
                .map(|(cs, tn_map)| {
                    (
                        cs.clone(),
                        tn_map.iter().map(|(tn, q)| (tn.clone(), q * rhs)).collect(),
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

impl FromCSL<csl::Assets> for BTreeMap<TokenName, BigInt> {
    fn from_csl(value: &csl::Assets) -> Self {
        let keys = value.keys();
        (0..keys.len()).fold(BTreeMap::new(), |mut acc, idx| {
            let asset_name = keys.get(idx);
            if let Some(quantity) = value.get(&asset_name) {
                acc.insert(
                    TokenName::from_csl(&asset_name),
                    BigInt::from_csl(&quantity),
                );
            }
            acc
        })
    }
}

impl TryFromPLA<BTreeMap<TokenName, BigInt>> for csl::Assets {
    fn try_from_pla(val: &BTreeMap<TokenName, BigInt>) -> Result<Self, TryFromPLAError> {
        val.iter().try_fold(csl::Assets::new(), |mut acc, (k, v)| {
            acc.insert(&k.try_to_csl()?, &v.try_to_csl()?);
            Ok(acc)
        })
    }
}

impl FromCSL<csl::MultiAsset> for Value {
    fn from_csl(value: &csl::MultiAsset) -> Self {
        let keys = value.keys();
        Value((0..keys.len()).fold(BTreeMap::new(), |mut acc, idx| {
            let script_hash = keys.get(idx);
            if let Some(assets) = value.get(&script_hash) {
                let assets = BTreeMap::from_csl(&assets);
                acc.insert(
                    CurrencySymbol::NativeToken(MintingPolicyHash::from_csl(&script_hash)),
                    assets,
                );
            }
            acc
        }))
    }
}

impl FromCSL<csl::Value> for Value {
    fn from_csl(value: &csl::Value) -> Self {
        let lovelaces = BigInt::from_csl(&value.coin());
        let mut pla_value = Value::ada_value(&lovelaces);
        if let Some(multi_asset) = value.multiasset() {
            pla_value = &pla_value + &Value::from_csl(&multi_asset)
        }
        pla_value
    }
}

impl TryFromPLA<Value> for csl::Value {
    fn try_from_pla(val: &Value) -> Result<Self, TryFromPLAError> {
        let coin: csl::Coin = val
            .0
            .get(&CurrencySymbol::Ada)
            .and_then(|m| m.get(&TokenName::ada()))
            .map_or(Ok(csl::BigNum::zero()), TryToCSL::try_to_csl)?;

        let m_ass = val
            .0
            .iter()
            .filter_map(|(cs, tn_map)| match &cs {
                CurrencySymbol::Ada => None,
                CurrencySymbol::NativeToken(h) => Some((h, tn_map)),
            })
            .try_fold(csl::MultiAsset::new(), |mut acc, (cs, ass)| {
                acc.insert(&cs.try_to_csl()?, &ass.try_to_csl()?);
                Ok(acc)
            })?;

        let mut v = csl::Value::new(&coin);

        v.set_multiasset(&m_ass);

        Ok(v)
    }
}

impl FromCSL<csl::MintAssets> for BTreeMap<TokenName, BigInt> {
    fn from_csl(m_ass: &csl::MintAssets) -> Self {
        let keys = m_ass.keys();
        (0..keys.len())
            .map(|idx| {
                let key = keys.get(idx);
                let value = m_ass.get(&key).unwrap();
                (TokenName::from_csl(&key), BigInt::from_csl(&value))
            })
            .collect()
    }
}

impl FromCSL<csl::MintsAssets> for BTreeMap<TokenName, BigInt> {
    fn from_csl(value: &csl::MintsAssets) -> Self {
        let mut m_ass_vec = Vec::new();

        // This is so stupid. `MintsAssets` doesn't have a `len` method for some reason.
        for idx in 0.. {
            if let Some(m_ass) = value.get(idx) {
                m_ass_vec.push(m_ass);
            } else {
                break;
            }
        }

        m_ass_vec.into_iter().fold(BTreeMap::new(), |acc, m| {
            let ass = BTreeMap::from_csl(&m);
            union_b_tree_maps_with(|l, r| l + r, [&acc, &ass])
        })
    }
}

impl TryFromPLA<BTreeMap<TokenName, BigInt>> for csl::MintAssets {
    fn try_from_pla(val: &BTreeMap<TokenName, BigInt>) -> Result<Self, TryFromPLAError> {
        val.iter()
            .try_fold(csl::MintAssets::new(), |mut acc, (k, v)| {
                acc.insert(&k.try_to_csl()?, &v.try_to_csl()?)
                    .map_err(TryFromPLAError::CSLJsError)?;
                Ok(acc)
            })
    }
}

impl FromCSL<csl::Mint> for Value {
    fn from_csl(mint: &csl::Mint) -> Self {
        let keys = mint.keys();
        Value(
            (0..keys.len())
                .map(|idx| {
                    let sh = keys.get(idx);
                    let ass = mint.get(&sh).unwrap_or(csl::MintsAssets::new());
                    (
                        CurrencySymbol::NativeToken(MintingPolicyHash::from_csl(&sh)),
                        BTreeMap::from_csl(&ass),
                    )
                })
                .collect::<BTreeMap<CurrencySymbol, BTreeMap<TokenName, BigInt>>>(),
        )
    }
}

///////////////
// TokenName //
///////////////

/// Name of a token. This can be any arbitrary bytearray
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TokenName(pub LedgerBytes);

impl TokenName {
    pub fn ada() -> TokenName {
        TokenName(LedgerBytes(Vec::with_capacity(0)))
    }

    pub fn is_empty(&self) -> bool {
        self.0 .0.is_empty()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        TokenName(LedgerBytes(bytes))
    }

    pub fn from_string(str: &str) -> Self {
        TokenName(LedgerBytes(String::from(str).into_bytes()))
    }

    pub fn try_into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.0 .0)
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

impl fmt::Display for TokenName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let utf8_str = std::str::from_utf8(&self.0 .0);

            match utf8_str {
                Ok(str) => write!(f, "{}", str),
                Err(_) => write!(f, "0x{}", self.0),
            }
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl FromCSL<csl::AssetName> for TokenName {
    fn from_csl(value: &csl::AssetName) -> Self {
        TokenName(LedgerBytes(value.name()))
    }
}

impl TryFromPLA<TokenName> for csl::AssetName {
    fn try_from_pla(val: &TokenName) -> Result<Self, TryFromPLAError> {
        csl::AssetName::new(val.0 .0.to_owned()).map_err(TryFromPLAError::CSLJsError)
    }
}

////////////////
// AssetClass //
////////////////

/// AssetClass is uniquely identifying a specific asset
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct AssetClass {
    pub currency_symbol: CurrencySymbol,
    pub token_name: TokenName,
}

impl fmt::Display for AssetClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token_name.is_empty() {
            write!(f, "{}", self.currency_symbol)
        } else {
            write!(f, "{}.{}", self.currency_symbol, self.token_name)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_from_string_token_name() {
        let name = "Hello";
        let token_name = TokenName::from_string(name);

        assert_eq!(token_name.try_into_string().unwrap(), name);
    }
}

////////////////
// Lovelace //
////////////////

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Lovelace(pub BigInt);
