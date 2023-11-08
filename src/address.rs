use crate::crypto::Ed25519PubKeyHash;
use crate::ledger_state::Slot;
use crate::plutus_data::{
    verify_constr_fields, FromPlutusData, PlutusData, PlutusDataError, PlutusType, ToPlutusData,
};
use crate::script::ValidatorHash;
#[cfg(feature = "lbf")]
use lbr_prelude::json::{self, Error, Json};
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Shelley Address for wallets or validators
///
/// An address consists of a payment part (credential) and a delegation part (staking_credential).
/// In order to serialize an address to `bech32`, the network kind must be known.
/// For a better understanding of all the Cardano address types, read [CIP 19](https://cips.cardano.org/cips/cip19/)
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Address {
    pub credential: Credential,
    pub staking_credential: Option<StakingCredential>,
}

impl ToPlutusData for Address {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.credential.to_plutus_data(),
                self.staking_credential.to_plutus_data(),
            ],
        )
    }
}

impl FromPlutusData for Address {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(Address {
                        credential: Credential::from_plutus_data(fields[0].clone())?,
                        staking_credential: <Option<StakingCredential>>::from_plutus_data(
                            fields[1].clone(),
                        )?,
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

/// A public key hash or validator hash credential (used as a payment or a staking credential)
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Credential {
    PubKey(Ed25519PubKeyHash),
    Script(ValidatorHash),
}

impl ToPlutusData for Credential {
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            Credential::PubKey(pkh) => {
                PlutusData::Constr(BigInt::from(0), vec![pkh.to_plutus_data()])
            }
            Credential::Script(val_hash) => {
                PlutusData::Constr(BigInt::from(1), vec![val_hash.to_plutus_data()])
            }
        }
    }
}

impl FromPlutusData for Credential {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Credential::PubKey(Ed25519PubKeyHash::from_plutus_data(
                        fields[0].clone(),
                    )?))
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(Credential::Script(ValidatorHash::from_plutus_data(
                        fields[0].clone(),
                    )?))
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

#[cfg(feature = "lbf")]
impl Json for Credential {
    fn to_json(&self) -> Result<serde_json::Value, Error> {
        match self {
            Credential::PubKey(pkh) => Ok(json::sum_constructor(
                "PubKeyCredential",
                vec![pkh.to_json()?],
            )),
            Credential::Script(val_hash) => Ok(json::sum_constructor(
                "ScriptCredential",
                vec![val_hash.to_json()?],
            )),
        }
    }

    fn from_json(value: serde_json::Value) -> Result<Self, Error> {
        json::sum_parser(&value).and_then(|obj| match obj {
            ("PubKeyCredential", ctor_fields) => match &ctor_fields[..] {
                [pkh] => Ok(Credential::PubKey(Json::from_json(pkh.clone())?)),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 1,
                    got: ctor_fields.len(),
                }),
            },
            ("ScriptCredential", ctor_fields) => match &ctor_fields[..] {
                [val_hash] => Ok(Credential::Script(Json::from_json(val_hash.clone())?)),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 1,
                    got: ctor_fields.len(),
                }),
            },
            _ => Err(Error::UnexpectedJsonInvariant {
                wanted: "constructor names (Nothing, Just)".to_owned(),
                got: "unknown constructor name".to_owned(),
            }),
        })
    }
}

/// Credential (public key hash or pointer) used for staking
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StakingCredential {
    Hash(Credential),
    Pointer(ChainPointer),
}

impl ToPlutusData for StakingCredential {
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            StakingCredential::Hash(credential) => {
                PlutusData::Constr(BigInt::from(0), vec![credential.to_plutus_data()])
            }
            StakingCredential::Pointer(ChainPointer {
                slot_number,
                transaction_index,
                certificate_index,
            }) => PlutusData::Constr(
                BigInt::from(1),
                vec![
                    slot_number.to_plutus_data(),
                    transaction_index.to_plutus_data(),
                    certificate_index.to_plutus_data(),
                ],
            ),
        }
    }
}

impl FromPlutusData for StakingCredential {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(&flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(StakingCredential::Hash(Credential::from_plutus_data(
                        fields[0].clone(),
                    )?))
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 3)?;
                    Ok(StakingCredential::Pointer(ChainPointer {
                        slot_number: Slot::from_plutus_data(fields[0].clone())?,
                        transaction_index: TransactionIndex::from_plutus_data(fields[1].clone())?,
                        certificate_index: CertificateIndex::from_plutus_data(fields[2].clone())?,
                    }))
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

#[cfg(feature = "lbf")]
impl Json for StakingCredential {
    fn to_json(&self) -> Result<serde_json::Value, Error> {
        match self {
            StakingCredential::Hash(pkh) => {
                Ok(json::sum_constructor("StakingHash", vec![pkh.to_json()?]))
            }
            StakingCredential::Pointer(val_hash) => Ok(json::sum_constructor(
                "StakingPtr",
                vec![val_hash.to_json()?],
            )),
        }
    }

    fn from_json(value: serde_json::Value) -> Result<Self, Error> {
        json::sum_parser(&value).and_then(|obj| match obj {
            ("StakingHash", ctor_fields) => match &ctor_fields[..] {
                [pkh] => Ok(StakingCredential::Hash(Json::from_json(pkh.clone())?)),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 1,
                    got: ctor_fields.len(),
                }),
            },
            ("StakingPtr", ctor_fields) => match &ctor_fields[..] {
                [val_hash] => Ok(StakingCredential::Pointer(Json::from_json(
                    val_hash.clone(),
                )?)),
                _ => Err(Error::UnexpectedArrayLength {
                    wanted: 1,
                    got: ctor_fields.len(),
                }),
            },
            _ => Err(Error::UnexpectedJsonInvariant {
                wanted: "constructor names (Nothing, Just)".to_owned(),
                got: "unknown constructor name".to_owned(),
            }),
        })
    }
}

/// In an address, a chain pointer refers to a point of the chain containing a stake key
/// registration certificate. A point is identified by 3 coordinates:
/// - An absolute slot number
/// - A transaction inder (within that slot)
/// - A (delegation) certificate index (within that transacton)
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ChainPointer {
    pub slot_number: Slot,
    pub transaction_index: TransactionIndex,
    pub certificate_index: CertificateIndex,
}

/// Position of the certificate in a certain transaction
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct CertificateIndex(pub BigInt);

impl ToPlutusData for CertificateIndex {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for CertificateIndex {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}

/// Position of a transaction in a given slot
/// This is not identical to the index of a `TransactionInput`
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionIndex(pub BigInt);

impl ToPlutusData for TransactionIndex {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }
}

impl FromPlutusData for TransactionIndex {
    fn from_plutus_data(data: PlutusData) -> Result<Self, PlutusDataError> {
        FromPlutusData::from_plutus_data(data).map(Self)
    }
}
