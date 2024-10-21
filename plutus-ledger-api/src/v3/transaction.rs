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
    /// Register staking credential with an optional deposit amount
    RegStaking(StakingCredential, Option<Lovelace>),
    /// Un-Register staking credential with an optional refund amount
    UnRegStaking(StakingCredential, Option<Lovelace>),
    /// Delegate staking credential to a Delegatee
    DelegStaking(StakingCredential, Delegatee),
    /// Register and delegate staking credential to a Delegatee in one certificate. Note that deposit is mandatory.
    RegDeleg(StakingCredential, Delegatee, Lovelace),
    /// Register a DRep with a deposit value. The optional anchor is omitted.
    RegDRep(DRepCredential, Lovelace),
    /// Update a DRep. The optional anchor is omitted.
    UpdateDRep(DRepCredential),
    /// UnRegister a DRep with mandatory refund value
    UnRegDRep(DRepCredential, Lovelace),
    /// A digest of the PoolParams
    PoolRegister(
        /// pool id
        PaymentPubKeyHash,
        // pool vrf
        PaymentPubKeyHash,
    ),
    /// The retirement certificate and the Epoch in which the retirement will take place
    PoolRetire(PaymentPubKeyHash, BigInt),
    /// Authorize a Hot credential for a specific Committee member's cold credential
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

/// Similar to TransactionInput, but for GovernanceAction.
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
    /// Committee members with epoch number when each of them expires
    pub members: AssocMap<ColdCommitteeCredential, BigInt>,
    /// Quorum of the committee that is necessary for a successful vote
    pub quorum: Rational,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IsPlutusData)]
#[is_plutus_data_derive_strategy = "Constr"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "lbf", derive(Json))]
pub struct Constitution {
    /// Optional guardrail script
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
/// A Plutus Data object containing proposed parameter changes.
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
    /// Propose to change the protocol parameters
    ParameterChange(
        Option<GovernanceActionId>,
        ChangeParameters,
        // The hash of the constitution script
        Option<ScriptHash>,
    ),
    /// Propose to update protocol version
    HardForkInitiation(Option<GovernanceActionId>, ProtocolVersion),
    /// Propose to withdraw from the cardano treasury
    TreasuryWithdrawals(
        AssocMap<Credential, Lovelace>,
        // The hash of the constitution script
        Option<ScriptHash>,
    ),
    /// Propose to create a state of no-confidence in the current constitutional committee
    NoConfidence(Option<GovernanceActionId>),
    /// Propose to update the members of the constitutional committee and/or its signature threshold and/or terms
    UpdateCommittee(
        Option<GovernanceActionId>,
        /// Committee members to be removed
        Vec<ColdCommitteeCredential>,
        /// Committee members to be added
        AssocMap<ColdCommitteeCredential, BigInt>,
        /// New quorum
        Rational,
    ),
    /// Propose to modify the constitution or its guardrail script
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
    Certifying(
        /// 0-based index of the given `TxCert` in `the `tx_certs` field of the `TransactionInfo`
        BigInt,
        TxCert,
    ),
    Voting(Voter),
    Proposing(
        /// 0-based index of the given `ProposalProcedure` in `protocol_procedures` field of the `TransactionInfo`
        BigInt,
        ProtocolProcedure,
    ),
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
