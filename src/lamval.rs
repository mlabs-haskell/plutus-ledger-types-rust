use crate::plutus_data::{self, PlutusData, PlutusDataError};
use num_bigint::BigInt;

pub fn case_plutus_data<'a, T: 'a>(
    x0: Box<dyn FnOnce(BigInt) -> Box<dyn Fn(Vec<PlutusData>) -> T>>,
) -> Box<
    dyn FnOnce(
            Box<dyn FnOnce(Vec<PlutusData>) -> T>,
        ) -> Box<
            dyn FnOnce(
                    Box<dyn FnOnce(BigInt) -> T>,
                ) -> Box<
                    dyn FnOnce(
                            Box<dyn FnOnce(PlutusData) -> T>,
                        ) -> Box<dyn FnOnce(PlutusData) -> T + 'a>
                        + 'a,
                > + 'a,
        > + 'a,
> {
    Box::new(move |x1| {
        Box::new(move |x2| {
            Box::new(move |x3| {
                Box::new(move |x4| plutus_data::case_plutus_data(x0, x1, x2, x3, x4))
            })
        })
    })
}

/// Fail PlutusData parsing with an internal error
pub fn fail_parse<T>(err: &str) -> Result<T, PlutusDataError> {
    Err(PlutusDataError::InternalError(err.to_owned()))
}

/// Curried Result::and_then function
pub fn bind_parse<'a, A: 'a, B: 'a>(
    x: Result<A, PlutusDataError>,
) -> Box<dyn FnOnce(Box<dyn Fn(A) -> Result<B, PlutusDataError>>) -> Result<B, PlutusDataError> + 'a>
{
    Box::new(move |f| x.and_then(f))
}
