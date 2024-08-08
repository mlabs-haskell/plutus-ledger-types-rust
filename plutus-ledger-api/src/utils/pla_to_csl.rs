use std::collections::BTreeMap;

use cardano_serialization_lib as csl;
use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::sign::Signed;

use crate::{
    plutus_data::PlutusData,
    v2::{
        address::{
            Address, CertificateIndex, ChainPointer, Credential, Slot, StakingCredential,
            TransactionIndex,
        },
        assoc_map::AssocMap,
        crypto::Ed25519PubKeyHash,
        datum::{Datum, DatumHash, OutputDatum},
        redeemer::Redeemer,
        script::{MintingPolicyHash, ScriptHash},
        transaction::{POSIXTimeRange, TransactionHash, TransactionInput, TransactionOutput},
        value::{CurrencySymbol, TokenName, Value},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum TryFromPLAError {
    #[error("{0}")]
    CSLDeserializeError(csl::error::DeserializeError),

    #[error("{0}")]
    CSLJsError(csl::error::JsError),

    #[error("Unable to cast BigInt {0} into type {1}: value is out of bound")]
    BigIntOutOfRange(BigInt, String),

    #[error("Unable to represent PLA value in CSL: ${0}")]
    ImpossibleConversion(String),

    #[error("Invalid valid transaction time range: ${0:?}")]
    InvalidTimeRange(POSIXTimeRange),

    #[error("Script is missing from context: {0:?}")]
    MissingScript(ScriptHash),
}

/// Convert a plutus-ledger-api type to its cardano-serialization-lib counterpart
/// `try_to_csl_with` accepts extra data where the PLA data itself is not enough
pub trait TryFromPLA<T> {
    fn try_from_pla(val: &T) -> Result<Self, TryFromPLAError>
    where
        Self: Sized;
}

/// Convert a plutus-ledger-api type to its cardano-serialization-lib counterpart
/// `try_to_csl_with` accepts extra data where the PLA data itself is not enough
///
/// DO NOT IMPLEMENT THIS DIRECTLY. Implement `TryFromPLA` instead.
pub trait TryToCSL<T> {
    fn try_to_csl(&self) -> Result<T, TryFromPLAError>;
}

impl<T, U> TryToCSL<U> for T
where
    U: TryFromPLA<T>,
{
    fn try_to_csl(&self) -> Result<U, TryFromPLAError> {
        TryFromPLA::try_from_pla(self)
    }
}

impl TryFromPLA<u64> for csl::utils::BigNum {
    fn try_from_pla(val: &u64) -> Result<Self, TryFromPLAError> {
        // BigNum(s) are u64 under the hood.

        Ok(csl::utils::BigNum::from(*val))
    }
}

impl TryFromPLA<BigInt> for csl::utils::BigNum {
    fn try_from_pla(val: &BigInt) -> Result<Self, TryFromPLAError> {
        // BigNum(s) are u64 under the hood.
        let x: u64 = val
            .to_owned()
            .try_into()
            .map_err(|err: TryFromBigIntError<BigInt>| {
                TryFromPLAError::BigIntOutOfRange(err.into_original(), "u64".into())
            })?;

        x.try_to_csl()
    }
}

impl TryFromPLA<BigInt> for csl::utils::BigInt {
    fn try_from_pla(val: &BigInt) -> Result<Self, TryFromPLAError> {
        Ok(val.to_owned().into())
    }
}

impl TryFromPLA<BigInt> for csl::utils::Int {
    fn try_from_pla(val: &BigInt) -> Result<Self, TryFromPLAError> {
        if val.is_negative() {
            Ok(csl::utils::Int::new_negative(&(val.abs()).try_to_csl()?))
        } else {
            Ok(csl::utils::Int::new(&val.try_to_csl()?))
        }
    }
}

impl TryFromPLA<i64> for csl::utils::Int {
    fn try_from_pla(val: &i64) -> Result<Self, TryFromPLAError> {
        if val.is_negative() {
            Ok(csl::utils::Int::new_negative(&csl::utils::to_bignum(
                val.unsigned_abs(),
            )))
        } else {
            Ok(csl::utils::Int::new(&csl::utils::to_bignum(*val as u64)))
        }
    }
}

impl TryFromPLA<Ed25519PubKeyHash> for csl::crypto::Ed25519KeyHash {
    fn try_from_pla(val: &Ed25519PubKeyHash) -> Result<Self, TryFromPLAError> {
        csl::crypto::Ed25519KeyHash::from_bytes(val.0 .0.to_owned())
            .map_err(TryFromPLAError::CSLDeserializeError)
    }
}

impl TryFromPLA<ScriptHash> for csl::crypto::ScriptHash {
    fn try_from_pla(val: &ScriptHash) -> Result<Self, TryFromPLAError> {
        csl::crypto::ScriptHash::from_bytes(val.0 .0.to_owned())
            .map_err(TryFromPLAError::CSLDeserializeError)
    }
}

impl TryFromPLA<TransactionHash> for csl::crypto::TransactionHash {
    fn try_from_pla(val: &TransactionHash) -> Result<Self, TryFromPLAError> {
        csl::crypto::TransactionHash::from_bytes(val.0 .0.to_owned())
            .map_err(TryFromPLAError::CSLDeserializeError)
    }
}

impl TryFromPLA<BigInt> for u32 /* TransactionIndex */ {
    fn try_from_pla(val: &BigInt) -> Result<Self, TryFromPLAError> {
        val.to_owned()
            .try_into()
            .map_err(|err: TryFromBigIntError<BigInt>| {
                TryFromPLAError::BigIntOutOfRange(err.into_original(), "u32".into())
            })
    }
}

impl TryFromPLA<TransactionInput> for csl::TransactionInput {
    fn try_from_pla(val: &TransactionInput) -> Result<Self, TryFromPLAError> {
        Ok(csl::TransactionInput::new(
            &val.transaction_id.try_to_csl()?,
            val.index.try_to_csl()?,
        ))
    }
}

impl TryFromPLA<Vec<TransactionInput>> for csl::TransactionInputs {
    fn try_from_pla(val: &Vec<TransactionInput>) -> Result<Self, TryFromPLAError> {
        val.iter()
            .try_fold(csl::TransactionInputs::new(), |mut acc, input| {
                acc.add(&input.try_to_csl()?);
                Ok(acc)
            })
    }
}

#[derive(Clone, Debug)]
pub struct TransactionOutputWithExtraInfo<'a> {
    pub transaction_output: &'a TransactionOutput,
    pub scripts: &'a BTreeMap<ScriptHash, csl::plutus::PlutusScript>,
    pub network_id: u8,
    pub data_cost: &'a csl::DataCost,
}

impl TryFromPLA<TransactionOutputWithExtraInfo<'_>> for csl::TransactionOutput {
    fn try_from_pla(val: &TransactionOutputWithExtraInfo<'_>) -> Result<Self, TryFromPLAError> {
        let mut output_builder = csl::output_builder::TransactionOutputBuilder::new().with_address(
            &AddressWithExtraInfo {
                address: &val.transaction_output.address,
                network_tag: val.network_id,
            }
            .try_to_csl()?,
        );

        output_builder = match &val.transaction_output.datum {
            OutputDatum::None => output_builder,
            OutputDatum::InlineDatum(Datum(d)) => output_builder.with_plutus_data(&d.try_to_csl()?),
            OutputDatum::DatumHash(dh) => output_builder.with_data_hash(&dh.try_to_csl()?),
        };

        let script_ref = val
            .transaction_output
            .reference_script
            .clone()
            .map(|script_hash| -> Result<_, TryFromPLAError> {
                let script = val
                    .scripts
                    .get(&script_hash)
                    .ok_or(TryFromPLAError::MissingScript(script_hash))?;
                Ok(csl::ScriptRef::new_plutus_script(script))
            })
            .transpose()?;

        if let Some(script_ref) = &script_ref {
            output_builder = output_builder.with_script_ref(script_ref);
        };

        let value_without_min_utxo = val.transaction_output.value.try_to_csl()?;

        let mut calc = csl::utils::MinOutputAdaCalculator::new_empty(val.data_cost)
            .map_err(TryFromPLAError::CSLJsError)?;
        calc.set_amount(&value_without_min_utxo);
        match &val.transaction_output.datum {
            OutputDatum::None => {}
            OutputDatum::InlineDatum(Datum(d)) => {
                calc.set_plutus_data(&d.try_to_csl()?);
            }
            OutputDatum::DatumHash(dh) => {
                calc.set_data_hash(&dh.try_to_csl()?);
            }
        };
        if let Some(script_ref) = script_ref {
            calc.set_script_ref(&script_ref);
        }

        let required_coin = calc.calculate_ada().map_err(TryFromPLAError::CSLJsError)?;
        let coin = std::cmp::max(value_without_min_utxo.coin(), required_coin);

        let value = match value_without_min_utxo.multiasset() {
            Some(multiasset) => csl::utils::Value::new_with_assets(&coin, &multiasset),
            None => csl::utils::Value::new(&coin),
        };

        output_builder
            .next()
            .map_err(TryFromPLAError::CSLJsError)?
            .with_value(&value)
            .build()
            .map_err(TryFromPLAError::CSLJsError)
    }
}

impl TryFromPLA<MintingPolicyHash> for csl::PolicyID {
    fn try_from_pla(val: &MintingPolicyHash) -> Result<Self, TryFromPLAError> {
        val.0.try_to_csl()
    }
}

impl TryFromPLA<TokenName> for csl::AssetName {
    fn try_from_pla(val: &TokenName) -> Result<Self, TryFromPLAError> {
        csl::AssetName::new(val.0 .0.to_owned()).map_err(TryFromPLAError::CSLJsError)
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

impl TryFromPLA<BTreeMap<TokenName, BigInt>> for csl::MintAssets {
    fn try_from_pla(val: &BTreeMap<TokenName, BigInt>) -> Result<Self, TryFromPLAError> {
        val.iter()
            .try_fold(csl::MintAssets::new(), |mut acc, (k, v)| {
                acc.insert(&k.try_to_csl()?, v.try_to_csl()?);
                Ok(acc)
            })
    }
}

impl TryFromPLA<Value> for csl::utils::Value {
    fn try_from_pla(val: &Value) -> Result<Self, TryFromPLAError> {
        let coin: csl::utils::Coin = val
            .0
            .get(&CurrencySymbol::Ada)
            .and_then(|m| m.get(&TokenName::ada()))
            .map_or(Ok(csl::utils::BigNum::zero()), TryToCSL::try_to_csl)?;

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

        let mut v = csl::utils::Value::new(&coin);

        v.set_multiasset(&m_ass);

        Ok(v)
    }
}

impl TryFromPLA<PlutusData> for csl::plutus::PlutusData {
    fn try_from_pla(val: &PlutusData) -> Result<Self, TryFromPLAError> {
        match val {
            PlutusData::Constr(tag, args) => Ok(csl::plutus::PlutusData::new_constr_plutus_data(
                &csl::plutus::ConstrPlutusData::new(&tag.try_to_csl()?, &args.try_to_csl()?),
            )),
            PlutusData::Map(l) => Ok(csl::plutus::PlutusData::new_map(&l.try_to_csl()?)),
            PlutusData::List(l) => Ok(csl::plutus::PlutusData::new_list(&l.try_to_csl()?)),
            PlutusData::Integer(i) => Ok(csl::plutus::PlutusData::new_integer(&i.try_to_csl()?)),
            PlutusData::Bytes(b) => Ok(csl::plutus::PlutusData::new_bytes(b.to_owned())),
        }
    }
}

impl TryFromPLA<Vec<PlutusData>> for csl::plutus::PlutusList {
    fn try_from_pla(val: &Vec<PlutusData>) -> Result<Self, TryFromPLAError> {
        val.iter()
            // traverse
            .map(|x| x.try_to_csl())
            .collect::<Result<Vec<csl::plutus::PlutusData>, TryFromPLAError>>()
            .map(|x| x.into())
    }
}

impl TryFromPLA<Vec<(PlutusData, PlutusData)>> for csl::plutus::PlutusMap {
    fn try_from_pla(val: &Vec<(PlutusData, PlutusData)>) -> Result<Self, TryFromPLAError> {
        val.iter()
            .try_fold(csl::plutus::PlutusMap::new(), |mut acc, (k, v)| {
                acc.insert(&k.try_to_csl()?, &v.try_to_csl()?);
                Ok(acc)
            })
    }
}

impl TryFromPLA<DatumHash> for csl::crypto::DataHash {
    fn try_from_pla(val: &DatumHash) -> Result<Self, TryFromPLAError> {
        csl::crypto::DataHash::from_bytes(val.0 .0.to_owned())
            .map_err(TryFromPLAError::CSLDeserializeError)
    }
}

impl TryFromPLA<Datum> for csl::plutus::PlutusData {
    fn try_from_pla(val: &Datum) -> Result<Self, TryFromPLAError> {
        val.0.try_to_csl()
    }
}

impl TryFromPLA<ChainPointer> for csl::address::Pointer {
    fn try_from_pla(val: &ChainPointer) -> Result<Self, TryFromPLAError> {
        Ok(csl::address::Pointer::new_pointer(
            &val.slot_number.try_to_csl()?,
            &val.transaction_index.try_to_csl()?,
            &val.certificate_index.try_to_csl()?,
        ))
    }
}

impl TryFromPLA<Slot> for csl::utils::BigNum {
    fn try_from_pla(val: &Slot) -> Result<Self, TryFromPLAError> {
        val.0.try_to_csl()
    }
}

impl TryFromPLA<TransactionIndex> for csl::utils::BigNum {
    fn try_from_pla(val: &TransactionIndex) -> Result<Self, TryFromPLAError> {
        val.0.try_to_csl()
    }
}

impl TryFromPLA<CertificateIndex> for csl::utils::BigNum {
    fn try_from_pla(val: &CertificateIndex) -> Result<Self, TryFromPLAError> {
        val.0.try_to_csl()
    }
}

impl TryFromPLA<Credential> for csl::address::StakeCredential {
    fn try_from_pla(val: &Credential) -> Result<Self, TryFromPLAError> {
        match val {
            Credential::PubKey(pkh) => Ok(csl::address::StakeCredential::from_keyhash(
                &pkh.try_to_csl()?,
            )),
            Credential::Script(sh) => Ok(csl::address::StakeCredential::from_scripthash(
                &sh.0.try_to_csl()?,
            )),
        }
    }
}

impl TryFromPLA<StakingCredential> for csl::address::StakeCredential {
    fn try_from_pla(val: &StakingCredential) -> Result<Self, TryFromPLAError> {
        match val {
            StakingCredential::Hash(c) => c.try_to_csl(),
            StakingCredential::Pointer(_) => Err(TryFromPLAError::ImpossibleConversion(
                "cannot represent chain pointer".into(),
            )),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressWithExtraInfo<'a> {
    pub address: &'a Address,
    pub network_tag: u8,
}

impl TryFromPLA<AddressWithExtraInfo<'_>> for csl::address::Address {
    fn try_from_pla(val: &AddressWithExtraInfo<'_>) -> Result<Self, TryFromPLAError> {
        let payment = val.address.credential.try_to_csl()?;

        Ok(match val.address.staking_credential {
            None => csl::address::EnterpriseAddress::new(val.network_tag, &payment).to_address(),
            Some(ref sc) => match sc {
                StakingCredential::Hash(c) => {
                    csl::address::BaseAddress::new(val.network_tag, &payment, &c.try_to_csl()?)
                        .to_address()
                }
                StakingCredential::Pointer(ptr) => {
                    csl::address::PointerAddress::new(val.network_tag, &payment, &ptr.try_to_csl()?)
                        .to_address()
                }
            },
        })
    }
}

impl std::fmt::Display for AddressWithExtraInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bech32_addr: Option<String> = self
            .try_to_csl()
            .ok()
            .and_then(|csl_addr: csl::address::Address| csl_addr.to_bech32(None).ok());
        match bech32_addr {
            Some(addr) => write!(f, "{}", addr),
            None => write!(f, "INVALID ADDRESS {:?}", self),
        }
    }
}

#[derive(Clone, Debug)]
struct RewardAddressWithExtraInfo<'a> {
    pub staking_credential: &'a StakingCredential,
    pub network_tag: u8,
}

impl TryFromPLA<RewardAddressWithExtraInfo<'_>> for csl::address::RewardAddress {
    fn try_from_pla(val: &RewardAddressWithExtraInfo<'_>) -> Result<Self, TryFromPLAError> {
        Ok(csl::address::RewardAddress::new(
            val.network_tag,
            &val.staking_credential.try_to_csl()?,
        ))
    }
}

#[derive(Clone, Debug)]
struct WithdrawalsWithExtraInfo<'a> {
    withdrawals: &'a AssocMap<StakingCredential, BigInt>,
    network_tag: u8,
}

impl TryFromPLA<WithdrawalsWithExtraInfo<'_>> for csl::Withdrawals {
    fn try_from_pla(val: &WithdrawalsWithExtraInfo<'_>) -> Result<Self, TryFromPLAError> {
        val.withdrawals
            .0
            .iter()
            .try_fold(csl::Withdrawals::new(), |mut acc, (s, q)| {
                acc.insert(
                    &RewardAddressWithExtraInfo {
                        staking_credential: s,
                        network_tag: val.network_tag,
                    }
                    .try_to_csl()?,
                    &q.try_to_csl()?,
                );
                Ok(acc)
            })
    }
}

#[derive(Clone, Debug)]
struct RedeemerWithExtraInfo<'a> {
    redeemer: &'a Redeemer,
    tag: &'a csl::plutus::RedeemerTag,
    index: u64,
}

impl TryFromPLA<RedeemerWithExtraInfo<'_>> for csl::plutus::Redeemer {
    fn try_from_pla<'a>(
        val: &RedeemerWithExtraInfo<'_>,
    ) -> Result<csl::plutus::Redeemer, TryFromPLAError> {
        let Redeemer(plutus_data) = val.redeemer;
        Ok(csl::plutus::Redeemer::new(
            val.tag,
            &val.index.try_to_csl()?,
            &plutus_data.try_to_csl()?,
            &csl::plutus::ExUnits::new(&csl::utils::to_bignum(0), &csl::utils::to_bignum(0)),
        ))
    }
}

impl TryFromPLA<OutputDatum> for Option<csl::OutputDatum> {
    fn try_from_pla(
        pla_output_datum: &OutputDatum,
    ) -> Result<Option<csl::OutputDatum>, TryFromPLAError> {
        Ok(match pla_output_datum {
            OutputDatum::None => None,
            OutputDatum::InlineDatum(Datum(d)) => {
                Some(csl::OutputDatum::new_data(&d.try_to_csl()?))
            }
            OutputDatum::DatumHash(dh) => Some(csl::OutputDatum::new_data_hash(&dh.try_to_csl()?)),
        })
    }
}
