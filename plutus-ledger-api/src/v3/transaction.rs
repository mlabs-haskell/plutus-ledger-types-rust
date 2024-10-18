#[cfg(feature = "lbf")]
use lbr_prelude::json::Json;
use num_bigint::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    self as plutus_ledger_api,
    plutus_data::{IsPlutusData, PlutusData},
    v1::{
        address::{Credential, StakingCredential},
        assoc_map::AssocMap,
        crypto::{PaymentPubKeyHash, StakePubKeyHash},
        datum::{Datum, DatumHash},
        redeemer::Redeemer,
        script::ScriptHash,
        transaction::{POSIXTimeRange, TransactionHash, TransactionInput},
        value::{CurrencySymbol, Lovelace, Value},
    },
    v2::transaction::{TransactionOutput, TxInInfo},
};

use super::ratio::Rational;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ColdCommitteeCredential(pub Credential);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct HotCommitteeCredential(pub Credential);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct DRepCredential(pub Credential);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum DRep {
    DRep(DRepCredential),
    AlwaysAbstain,
    AlwaysNoConfidence,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum Delegatee {
    Stake(StakePubKeyHash),
    Vote(DRep),
    StakeVote(StakePubKeyHash, DRep),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum TxCert {
    RegStaking(StakingCredential, Option<Lovelace>),
    UnRegStaking(StakingCredential, Option<Lovelace>),
    DelegStaking(StakingCredential, Delegatee),
    RegDeleg(StakingCredential, Delegatee, Lovelace),
    RegDRep(DRepCredential, Lovelace),
    UpdateDRep(DRepCredential),
    UnRegDRep(DRepCredential, Lovelace),
    PoolRegister(PaymentPubKeyHash, PaymentPubKeyHash),
    PoolRetire(PaymentPubKeyHash, BigInt),
    AuthHotCommittee(ColdCommitteeCredential, HotCommitteeCredential),
    ResignColdCommittee(ColdCommitteeCredential),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum Voter {
    CommitteeVoter(HotCommitteeCredential),
    DRepVoter(DRepCredential),
    StakePoolVoter(PaymentPubKeyHash),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum Vote {
    VoteNo,
    VoteYes,
    Abstain,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct GovernanceActionId {
    pub tx_id: TransactionHash,
    pub gov_action_id: BigInt,
}

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Committee {
    pub members: AssocMap<ColdCommitteeCredential, BigInt>,
    pub quorum: Rational,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Constitution {
    pub constitution_script: Option<ScriptHash>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ProtocolVersion {
    pub major: BigInt,
    pub minor: BigInt,
}

// TODO(chfanghr): check invariant according to https://github.com/IntersectMBO/plutus/blob/bb33f082d26f8b6576d3f0d423be53eddfb6abd8/plutus-ledger-api/src/PlutusLedgerApi/V3/Contexts.hs#L338-L364
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Newtype"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ChangeParameters(pub PlutusData);

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum GovernanceAction {
    ParameterChange(
        Option<GovernanceActionId>,
        ChangeParameters,
        Option<ScriptHash>,
    ),
    HardForkInitiation(Option<GovernanceActionId>, ProtocolVersion),
    TreasuryWithdrawals(AssocMap<Credential, Lovelace>, Option<ScriptHash>),
    NoConfidence(Option<GovernanceActionId>),
    UpdateCommittee(
        Option<GovernanceActionId>,
        Vec<ColdCommitteeCredential>,
        AssocMap<ColdCommitteeCredential, BigInt>,
    ),
    NewConstitution(Option<GovernanceActionId>, Constitution),
    InfoAction,
}

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ProtocolProcedure {
    pub deposit: Lovelace,
    pub return_addr: Credential,
    pub governance_action: GovernanceAction,
}

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum ScriptPurpose {
    Minting(CurrencySymbol),
    Spending(TransactionInput),
    Rewarding(Credential),
    Certifying(BigInt, TxCert),
    Voting(Voter),
    Proposing(BigInt, ProtocolProcedure),
}

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub enum ScriptInfo {
    Minting(CurrencySymbol),
    Spending(TransactionInput, Option<Datum>),
    Rewarding(Credential),
    Certifying(BigInt, TxCert),
    Voting(Voter),
    Proposing(BigInt, ProtocolProcedure),
}

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct TransactionInfo {
    pub inputs: Vec<TxInInfo>,
    pub reference_inputs: Vec<TxInInfo>,
    pub outputs: Vec<TransactionOutput>,
    pub fee: Lovelace,
    pub mint: Value,
    pub tx_certs: Vec<TxCert>,
    pub wdrl: AssocMap<StakingCredential, BigInt>,
    pub valid_range: POSIXTimeRange,
    pub signatories: Vec<PaymentPubKeyHash>,
    pub redeemers: AssocMap<ScriptPurpose, Redeemer>,
    pub datums: AssocMap<DatumHash, Datum>,
    pub id: TransactionHash,
    pub votes: AssocMap<Voter, AssocMap<GovernanceActionId, Vote>>,
    pub protocol_procedures: Vec<ProtocolProcedure>,
    pub current_treasury_amount: Option<Lovelace>,
    pub treasury_donation: Option<Lovelace>,
}

#[derive(Clone, Debug, PartialEq, Eq, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct ScriptContext {
    pub tx_info: TransactionInfo,
    pub redeemer: Redeemer,
    pub script_info: ScriptInfo,
}
