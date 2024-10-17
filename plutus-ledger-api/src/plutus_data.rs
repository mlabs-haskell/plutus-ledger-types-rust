//! Plutus Data related types and traits
use cardano_serialization_lib as csl;
use num_bigint::BigInt;

use crate::csl::csl_to_pla::{FromCSL, TryFromCSL, TryFromCSLError, TryToPLA};
use crate::csl::pla_to_csl::{TryFromPLA, TryFromPLAError, TryToCSL};

pub use plutus_data::{
    is_plutus_data::aux::*, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};

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

impl TryFromCSL<csl::PlutusData> for PlutusData {
    fn try_from_csl(value: &csl::PlutusData) -> Result<Self, TryFromCSLError> {
        Ok(match value.kind() {
            csl::PlutusDataKind::ConstrPlutusData => {
                let constr_data = value.as_constr_plutus_data().unwrap();
                let tag = BigInt::from_csl(&constr_data.alternative());
                let args = constr_data.data().try_to_pla()?;
                PlutusData::Constr(tag, args)
            }
            csl::PlutusDataKind::Map => PlutusData::Map(value.as_map().unwrap().try_to_pla()?),
            csl::PlutusDataKind::List => PlutusData::List(value.as_list().unwrap().try_to_pla()?),
            csl::PlutusDataKind::Integer => {
                PlutusData::Integer(value.as_integer().unwrap().try_to_pla()?)
            }
            csl::PlutusDataKind::Bytes => PlutusData::Bytes(value.as_bytes().unwrap()),
        })
    }
}

impl TryFromCSL<csl::PlutusList> for Vec<PlutusData> {
    fn try_from_csl(value: &csl::PlutusList) -> Result<Self, TryFromCSLError> {
        (0..value.len())
            .map(|idx| value.get(idx).try_to_pla())
            .collect()
    }
}

impl TryFromCSL<csl::PlutusMap> for Vec<(PlutusData, PlutusData)> {
    fn try_from_csl(c_map: &csl::PlutusMap) -> Result<Self, TryFromCSLError> {
        let keys = c_map.keys();
        (0..keys.len()).try_fold(Vec::new(), |mut vector, idx| {
            let key = keys.get(idx);
            let values = c_map.get(&key).unwrap();

            for value_idx in 0..values.len() {
                vector.push((
                    key.clone().try_to_pla()?,
                    values.get(value_idx).unwrap().try_to_pla()?,
                ))
            }

            Ok(vector)
        })
    }
}

impl TryFromPLA<PlutusData> for csl::PlutusData {
    fn try_from_pla(val: &PlutusData) -> Result<Self, TryFromPLAError> {
        match val {
            PlutusData::Constr(tag, args) => Ok(csl::PlutusData::new_constr_plutus_data(
                &csl::ConstrPlutusData::new(&tag.try_to_csl()?, &args.try_to_csl()?),
            )),
            PlutusData::Map(l) => Ok(csl::PlutusData::new_map(&l.try_to_csl()?)),
            PlutusData::List(l) => Ok(csl::PlutusData::new_list(&l.try_to_csl()?)),
            PlutusData::Integer(i) => Ok(csl::PlutusData::new_integer(&i.try_to_csl()?)),
            PlutusData::Bytes(b) => Ok(csl::PlutusData::new_bytes(b.to_owned())),
        }
    }
}

impl TryFromPLA<Vec<PlutusData>> for csl::PlutusList {
    fn try_from_pla(val: &Vec<PlutusData>) -> Result<Self, TryFromPLAError> {
        val.iter()
            // traverse
            .map(|x| x.try_to_csl())
            .collect::<Result<Vec<csl::PlutusData>, TryFromPLAError>>()
            .map(|x| x.into())
    }
}

impl TryFromPLA<Vec<(PlutusData, PlutusData)>> for csl::PlutusMap {
    fn try_from_pla(val: &Vec<(PlutusData, PlutusData)>) -> Result<Self, TryFromPLAError> {
        val.iter()
            .try_fold(csl::PlutusMap::new(), |mut acc, (k, v)| {
                let mut values = match acc.get(&k.try_to_csl()?) {
                    Some(existing_values) => existing_values,
                    None => csl::PlutusMapValues::new(),
                };
                values.add(&v.try_to_csl()?);
                acc.insert(&k.try_to_csl()?, &values);
                Ok(acc)
            })
    }
}
