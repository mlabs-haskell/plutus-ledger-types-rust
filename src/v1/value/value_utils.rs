use crate::utils::{singleton, union_b_tree_maps_with};
use num_bigint::BigInt;
use num_traits::Zero;
use std::{
    collections::BTreeMap,
    iter::Sum,
    ops::{Add, Mul, Neg, Not, Sub},
};

use super::{CurrencySymbol, TokenName, Value};

impl Value {
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
    pub fn normalize(&self) -> Self {
        self.filter(|_, _, a| a.is_zero().not())
    }

    /// Apply a function to each token of the value, and use its result as the new amount.
    pub fn map_amount<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&CurrencySymbol, &TokenName, &BigInt) -> BigInt,
    {
        self.filter_map_amount(|cs, tn, a| Some(f(cs, tn, a)))
    }

    /// Apply a predicate to tokens.
    pub fn filter<F>(&self, mut f: F) -> Self
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
    pub fn filter_map_amount<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&CurrencySymbol, &TokenName, &BigInt) -> Option<BigInt>,
    {
        Value(
            (&self.0)
                .into_iter()
                .filter_map(|(cs, tn_map)| {
                    let filtered_tn_map = tn_map
                        .into_iter()
                        .filter_map(|(tn, a)| f(cs, tn, a).map(|a| (tn.clone(), a)))
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

forward_val_val_binop!(impl Add for Value, add);
forward_ref_val_binop!(impl Add for Value, add);
forward_val_ref_binop!(impl Add for Value, add);

impl Add<&Value> for &Value {
    type Output = Value;

    fn add(self, rhs: &Value) -> Self::Output {
        Value(union_b_tree_maps_with(
            |lhs, rhs| union_b_tree_maps_with(|lhs, rhs| lhs + rhs, lhs, rhs),
            &self.0,
            &rhs.0,
        ))
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Neg for &Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        self.map_amount(|_, _, a| a.neg())
    }
}

forward_val_val_binop!(impl Sub for Value, sub);
forward_ref_val_binop!(impl Sub for Value, sub);
forward_val_ref_binop!(impl Sub for Value, sub);

impl Sub<&Value> for &Value {
    type Output = Value;

    fn sub(self, rhs: &Value) -> Self::Output {
        self.add(rhs.neg())
    }
}

forward_scalar_val_val_binop_to_ref_val!(impl Mul<BigInt> for Value, mul);
forward_scalar_ref_val_binop_to_ref_ref!(impl Mul<BigInt> for Value, mul);
forward_scalar_val_ref_binop_to_ref_val!(impl Mul<BigInt> for Value, mul);
forward_scalar_ref_ref_binop_commutative!(impl Mul<BigInt> for Value, mul);

forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<i8> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<i16> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<i32> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<i64> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<i128> for Value, mul);

forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<u8> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<u16> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<u32> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<u64> for Value, mul);
forward_into_bigint_scalar_ref_val_binop_to_ref_ref!(impl Mul<u128> for Value, mul);

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
