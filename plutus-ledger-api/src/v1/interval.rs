//! Types related to PlutusInterval
use crate::feature_traits::FeatureTraits;
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp;

/// An abstraction over `PlutusInterval`, allowing valid values only
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Interval<T> {
    Finite(T, T),
    StartAt(T),
    StartAfter(T),
    EndAt(T),
    EndBefore(T),
    Always,
    Never,
}

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
            Interval::StartAt(start) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::Finite(start),
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::PosInf,
                    closed: true,
                },
            },
            Interval::StartAfter(start) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::Finite(start),
                    closed: false,
                },
                to: UpperBound {
                    bound: Extended::PosInf,
                    closed: true,
                },
            },
            Interval::EndAt(end) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::NegInf,
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::Finite(end),
                    closed: true,
                },
            },
            Interval::EndBefore(end) => PlutusInterval {
                from: LowerBound {
                    bound: Extended::NegInf,
                    closed: true,
                },
                to: UpperBound {
                    bound: Extended::Finite(end),
                    closed: false,
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

#[derive(thiserror::Error, Debug)]
pub enum TryFromPlutusIntervalError {
    #[error("Interval is invalid.")]
    InvalidInterval,
    #[error("Interval with open bound could not be converted.")]
    UnexpectedOpenBound,
}

impl<T> TryFrom<PlutusInterval<T>> for Interval<T>
where
    T: FeatureTraits + PartialOrd,
{
    type Error = TryFromPlutusIntervalError;

    fn try_from(interval: PlutusInterval<T>) -> Result<Self, Self::Error> {
        Ok(match interval {
            PlutusInterval {
                from:
                    LowerBound {
                        bound: Extended::Finite(start),
                        closed: lc,
                    },
                to:
                    UpperBound {
                        bound: Extended::Finite(end),
                        closed: uc,
                    },
            } => {
                if lc && uc {
                    if start > end {
                        Err(TryFromPlutusIntervalError::InvalidInterval)?
                    } else {
                        Interval::Finite(start, end)
                    }
                } else {
                    Err(TryFromPlutusIntervalError::UnexpectedOpenBound)?
                }
            }
            PlutusInterval {
                from:
                    LowerBound {
                        bound: Extended::Finite(start),
                        closed: lc,
                    },
                to:
                    UpperBound {
                        bound: Extended::PosInf,
                        closed: uc,
                    },
            } => {
                if lc && uc {
                    Interval::StartAt(start)
                } else if !lc && uc {
                    Interval::StartAfter(start)
                } else {
                    Err(TryFromPlutusIntervalError::UnexpectedOpenBound)?
                }
            }
            PlutusInterval {
                from:
                    LowerBound {
                        bound: Extended::NegInf,
                        closed: lc,
                    },
                to:
                    UpperBound {
                        bound: Extended::Finite(end),
                        closed: uc,
                    },
            } => {
                if uc && lc {
                    Interval::EndAt(end)
                } else if !uc && lc {
                    Interval::EndBefore(end)
                } else {
                    Err(TryFromPlutusIntervalError::UnexpectedOpenBound)?
                }
            }
            PlutusInterval {
                from:
                    LowerBound {
                        bound: Extended::NegInf,
                        closed: lc,
                    },
                to:
                    UpperBound {
                        bound: Extended::PosInf,
                        closed: uc,
                    },
            } => {
                if lc && uc {
                    Interval::Always
                } else {
                    Err(TryFromPlutusIntervalError::UnexpectedOpenBound)?
                }
            }
            PlutusInterval {
                from:
                    LowerBound {
                        bound: Extended::PosInf,
                        closed: lc,
                    },
                to:
                    UpperBound {
                        bound: Extended::NegInf,
                        closed: uc,
                    },
            } => {
                if lc && uc {
                    Interval::Never
                } else {
                    Err(TryFromPlutusIntervalError::UnexpectedOpenBound)?
                }
            }

            _ => Err(TryFromPlutusIntervalError::InvalidInterval)?,
        })
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(PlutusInterval {
                        from: <LowerBound<T>>::from_plutus_data(&fields[0])?,
                        to: <UpperBound<T>>::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(UpperBound {
                        bound: <Extended<T>>::from_plutus_data(&fields[0])?,
                        closed: bool::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(LowerBound {
                        bound: <Extended<T>>::from_plutus_data(&fields[0])?,
                        closed: bool::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
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

/// A set extended with a positive and negative infinity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl<T> Ord for Extended<T>
where
    T: FeatureTraits + Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Extended::PosInf, Extended::PosInf) => cmp::Ordering::Equal,
            (Extended::PosInf, _) => cmp::Ordering::Greater,

            (Extended::NegInf, Extended::NegInf) => cmp::Ordering::Equal,
            (Extended::NegInf, _) => cmp::Ordering::Less,

            (Extended::Finite(_), Extended::NegInf) => cmp::Ordering::Greater,
            (Extended::Finite(self_val), Extended::Finite(other_val)) => self_val.cmp(other_val),
            (Extended::Finite(_), Extended::PosInf) => cmp::Ordering::Less,
        }
    }
}

impl<T> PartialOrd for Extended<T>
where
    T: FeatureTraits + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self {
            Extended::PosInf => match other {
                Extended::PosInf => Some(cmp::Ordering::Equal),
                _ => Some(cmp::Ordering::Greater),
            },
            Extended::NegInf => match other {
                Extended::NegInf => Some(cmp::Ordering::Equal),
                _ => Some(cmp::Ordering::Less),
            },
            Extended::Finite(self_val) => match other {
                Extended::NegInf => Some(cmp::Ordering::Greater),
                Extended::Finite(other_val) => self_val.partial_cmp(other_val),
                Extended::PosInf => Some(cmp::Ordering::Less),
            },
        }
    }
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

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(Extended::NegInf)
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Extended::Finite(IsPlutusData::from_plutus_data(
                        &fields[0],
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
                got: PlutusType::from(data),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{generators::correct::v1::arb_interval_posix_time, v1::transaction::POSIXTime};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn interval_to_from_plutus_interval(interval in arb_interval_posix_time()) {
            let plutus_interval: PlutusInterval<POSIXTime> = interval.clone().into();
            let interval2: Interval<POSIXTime> = plutus_interval.clone().try_into().unwrap();

            assert_eq!(interval, interval2);
        }
    }
}
