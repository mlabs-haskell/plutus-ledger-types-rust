//! Types for cryptographic primitives, and other lower level building blocks
use cardano_serialization_lib as csl;
use data_encoding::HEXLOWER;
#[cfg(feature = "lbf")]
use lbr_prelude::json::{Error, Json};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    csl::{
        csl_to_pla::FromCSL,
        pla_to_csl::{TryFromPLA, TryFromPLAError},
    },
    plutus_data::{IsPlutusData, PlutusData, PlutusDataError, PlutusType},
};

///////////////////////
// Ed25519PubKeyHash //
///////////////////////

/// ED25519 public key hash
/// This is the standard cryptography in Cardano, commonly referred to as `PubKeyHash` in Plutus
/// and other libraries
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Ed25519PubKeyHash(pub LedgerBytes);

impl IsPlutusData for Ed25519PubKeyHash {
    fn to_plutus_data(&self) -> PlutusData {
        let Ed25519PubKeyHash(LedgerBytes(bytes)) = self;
        PlutusData::Bytes(bytes.clone())
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Bytes(bytes) => Ok(Self(LedgerBytes(bytes.clone()))),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Bytes,
                got: PlutusType::from(data),
            }),
        }
    }
}

impl FromCSL<csl::crypto::Ed25519KeyHash> for Ed25519PubKeyHash {
    fn from_csl(value: &csl::crypto::Ed25519KeyHash) -> Self {
        Ed25519PubKeyHash(LedgerBytes(value.to_bytes()))
    }
}

impl TryFromPLA<Ed25519PubKeyHash> for csl::crypto::Ed25519KeyHash {
    fn try_from_pla(val: &Ed25519PubKeyHash) -> Result<Self, TryFromPLAError> {
        csl::crypto::Ed25519KeyHash::from_bytes(val.0 .0.to_owned())
            .map_err(TryFromPLAError::CSLDeserializeError)
    }
}

impl FromCSL<csl::RequiredSigners> for Vec<Ed25519PubKeyHash> {
    fn from_csl(value: &csl::RequiredSigners) -> Self {
        (0..value.len())
            .map(|idx| Ed25519PubKeyHash::from_csl(&value.get(idx)))
            .collect()
    }
}

///////////////////////
// PaymentPubKeyHash //
///////////////////////

/// Standard public key hash used to verify a transaction witness
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct PaymentPubKeyHash(pub Ed25519PubKeyHash);

impl IsPlutusData for PaymentPubKeyHash {
    fn to_plutus_data(&self) -> PlutusData {
        let PaymentPubKeyHash(Ed25519PubKeyHash(LedgerBytes(bytes))) = self;
        PlutusData::Bytes(bytes.clone())
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Bytes(bytes) => Ok(Self(Ed25519PubKeyHash(LedgerBytes(bytes.clone())))),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Bytes,
                got: PlutusType::from(data),
            }),
        }
    }
}

/////////////////////
// StakePubKeyHash //
/////////////////////

/// Standard public key hash used to verify a staking
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct StakePubKeyHash(pub Ed25519PubKeyHash);

impl IsPlutusData for StakePubKeyHash {
    fn to_plutus_data(&self) -> PlutusData {
        let StakePubKeyHash(Ed25519PubKeyHash(LedgerBytes(bytes))) = self;
        PlutusData::Bytes(bytes.clone())
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Bytes(bytes) => Ok(Self(Ed25519PubKeyHash(LedgerBytes(bytes.clone())))),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Bytes,
                got: PlutusType::from(data),
            }),
        }
    }
}

/////////////////
// LedgerBytes //
/////////////////

/// A bytestring in the Cardano ledger context
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LedgerBytes(pub Vec<u8>);

impl std::fmt::Debug for LedgerBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HEXLOWER.encode(&self.0))
    }
}

impl std::fmt::Display for LedgerBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HEXLOWER.encode(&self.0))
    }
}

impl IsPlutusData for LedgerBytes {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Bytes(self.0.clone())
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Bytes(bytes) => Ok(Self(bytes.clone())),
            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Bytes,
                got: PlutusType::from(data),
            }),
        }
    }
}

#[cfg(feature = "lbf")]
impl Json for LedgerBytes {
    fn to_json(&self) -> serde_json::Value {
        String::to_json(&HEXLOWER.encode(&self.0))
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, Error> {
        let bytes = String::from_json(value).and_then(|str| {
            HEXLOWER
                .decode(&str.into_bytes())
                .map_err(|_| Error::UnexpectedJsonInvariant {
                    wanted: "base16 string".to_owned(),
                    got: "unexpected string".to_owned(),
                    parser: "Plutus.V1.Bytes".to_owned(),
                })
        })?;

        Ok(Self(bytes))
    }
}
