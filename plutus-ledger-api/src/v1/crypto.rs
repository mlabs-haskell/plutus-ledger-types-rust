//! Types for cryptographic primitives, and other lower level building blocks
use cardano_serialization_lib as csl;
use data_encoding::HEXLOWER;
#[cfg(feature = "lbf")]
use lbr_prelude::json::{Error, Json};
use plutus_data::IsPlutusData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::csl::{
    csl_to_pla::FromCSL,
    pla_to_csl::{TryFromPLA, TryFromPLAError},
};

///////////////////////
// Ed25519PubKeyHash //
///////////////////////

/// ED25519 public key hash
/// This is the standard cryptography in Cardano, commonly referred to as `PubKeyHash` in Plutus
/// and other libraries
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Ed25519PubKeyHash(pub LedgerBytes);

impl FromCSL<csl::Ed25519KeyHash> for Ed25519PubKeyHash {
    fn from_csl(value: &csl::Ed25519KeyHash) -> Self {
        Ed25519PubKeyHash(LedgerBytes(value.to_bytes()))
    }
}

impl TryFromPLA<Ed25519PubKeyHash> for csl::Ed25519KeyHash {
    fn try_from_pla(val: &Ed25519PubKeyHash) -> Result<Self, TryFromPLAError> {
        csl::Ed25519KeyHash::from_bytes(val.0 .0.to_owned())
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct PaymentPubKeyHash(pub Ed25519PubKeyHash);

/////////////////////
// StakePubKeyHash //
/////////////////////

/// Standard public key hash used to verify a staking
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct StakePubKeyHash(pub Ed25519PubKeyHash);

/////////////////
// LedgerBytes //
/////////////////

/// A bytestring in the Cardano ledger context
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
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
