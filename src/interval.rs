//! Types related to PlutusInterval
use crate::feature_traits::FeatureTraits;
use crate::plutus_data::{
    verify_constr_fields, PlutusData, PlutusDataError, PlutusType, IsPlutusData,
};
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An abstraction over `PlutusInterval`, allowing valid values only
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Interval<T> {
    Finite(T, T),
    StartAt(T),
    EndAt(T),
    Always,
    Never,
}

/// Loosely following the CTL implementation of `intervalToPlutusInterval`
/// However, as we don't have Semiring classes, the interval upper bounds are always closed
impl<T> From<Interval<T>> for PlutusInterval<T>
where
    T: FeatureTraits,
{
    fn from(interval: Interval<T>) -> Self {
        match interval {
            Interval::Finite(start, end) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::Finite(start),
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::Finite(end),
                    closed: true,
                },
            },
            Interval::StartAt(end) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::NegInf,
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::Finite(end),
                    closed: true,
                },
            },
            Interval::EndAt(start) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::Finite(start),
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::PosInf,
                    closed: true,
                },
            },
            Interval::Always => PlutusInterval {
                from: LowerBound {
                    bound: Extended::NegInf,
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::PosInf,
                    closed: true,
                },
            },
            Interval::Never => PlutusInterval {
                from: LowerBound {
                    bound: Extended::PosInf,
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::NegInf,
                    closed: true,
                },
            },
        }
    }
}

/// An interval of `T`s.
///
/// The interval may be either closed or open at either end, meaning
/// that the endpoints may or may not be included in the interval.
///
/// The interval can also be unbounded on either side.
///
/// The 'Eq' instance gives equality of the intervals, not structural equality.
/// There is no 'Ord' instance, but 'contains' gives a partial order.
///
/// Note that some of the functions on `Interval` rely on `Enum` in order to
/// handle non-inclusive endpoints. For this reason, it may not be safe to
/// use `Interval`s with non-inclusive endpoints on types whose `Enum`
/// instances have partial methods.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlutusInterval<T>
where
    T: FeatureTraits,
{
    pub from: LowerBound<T>,
    pub to: UpperBound<T>,
}

impl<T> IsPlutusData for PlutusInterval<T>
where
    T: FeatureTraits + IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![self.from.to_plutus_data(), self.to.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(PlutusInterval {
                        from: <LowerBound<T>>::from_plutus_data(fields[0].clone())?,
                        to: <UpperBound<T>>::from_plutus_data(fields[1].clone())?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(&data),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpperBound<T>
where
    T: FeatureTraits,
{
    pub bound: Extended<T>,
    pub closed: bool,
}

impl<T> IsPlutusData for UpperBound<T>
where
    T: FeatureTraits + IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![self.bound.to_plutus_data(), self.closed.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(UpperBound {
                        bound: <Extended<T>>::from_plutus_data(fields[0].clone())?,
                        closed: bool::from_plutus_data(fields[1].clone())?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(&data),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LowerBound<T>
where
    T: FeatureTraits,
{
    pub bound: Extended<T>,
    pub closed: bool,
}

impl<T> IsPlutusData for LowerBound<T>
where
    T: FeatureTraits + IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![self.bound.to_plutus_data(), self.closed.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(LowerBound {
                        bound: <Extended<T>>::from_plutus_data(fields[0].clone())?,
                        closed: bool::from_plutus_data(fields[1].clone())?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(&data),
            }),
        }
    }
}

/// A set extended with a positive and negative infinity.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "lbf", derive(Json))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Extended<T>
where
    T: FeatureTraits,
{
    NegInf,
    Finite(T),
    PosInf,
}

impl<T> IsPlutusData for Extended<T>
where
    T: FeatureTraits + IsPlutusData,
{
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            Extended::NegInf => PlutusData::Constr(BigInt::from(0), Vec::with_capacity(0)),
            Extended::Finite(value) => {
                PlutusData::Constr(BigInt::from(1), vec![value.to_plutus_data()])
            }
            Extended::PosInf => PlutusData::Constr(BigInt::from(2), Vec::with_capacity(0)),
        }
    }

    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(Extended::NegInf)
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Extended::Finite(IsPlutusData::from_plutus_data(
                        fields[0].clone(),
                    )?))
                }
                Ok(2) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(Extended::PosInf)
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(&data),
            }),
        }
    }
}
