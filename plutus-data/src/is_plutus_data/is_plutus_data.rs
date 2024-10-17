use crate::{PlutusData, PlutusType};

pub trait IsPlutusData {
    fn to_plutus_data(&self) -> PlutusData;

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError>
    where
        Self: Sized;
}

// TODO(chfanghr): improve error reporting
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
