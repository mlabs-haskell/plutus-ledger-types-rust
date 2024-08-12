//! Golden test data or Plutus V2 types
use crate::v2::{
    assoc_map::AssocMap,
    crypto::LedgerBytes,
    datum::OutputDatum,
    script::ScriptHash,
    transaction::{ScriptContext, TransactionInfo, TransactionOutput, TxInInfo},
};
use num_bigint::BigInt;

pub fn sample_output_datum() -> OutputDatum {
    OutputDatum::InlineDatum(super::v1::sample_datum())
}

pub fn sample_transaction_output() -> TransactionOutput {
    TransactionOutput {
        address: super::v1::sample_address(),
        value: super::v1::sample_value(),
        datum: sample_output_datum(),
        reference_script: Some(ScriptHash(LedgerBytes([0].repeat(28).to_vec()))),
    }
}

pub fn sample_tx_in_info() -> TxInInfo {
    TxInInfo {
        reference: super::v1::sample_transaction_input(),
        output: sample_transaction_output(),
    }
}

pub fn sample_transaction_info() -> TransactionInfo {
    TransactionInfo {
        inputs: vec![sample_tx_in_info()],
        outputs: vec![sample_transaction_output()],
        fee: super::v1::sample_value(),
        mint: super::v1::sample_value(),
        d_cert: vec![super::v1::sample_dcert()],
        wdrl: AssocMap::from([(super::v1::sample_staking_credential(), BigInt::from(12))]),
        valid_range: super::v1::sample_plutus_interval(),
        signatories: vec![super::v1::sample_payment_pub_key_hash()],
        datums: AssocMap::from([(super::v1::sample_datum_hash(), super::v1::sample_datum())]),
        redeemers: AssocMap::from([(
            super::v1::sample_script_purpose(),
            super::v1::sample_redeemer(),
        )]),
        id: super::v1::sample_transaction_hash(),
        reference_inputs: vec![sample_tx_in_info()],
    }
}

pub fn sample_script_context() -> ScriptContext {
    ScriptContext {
        tx_info: sample_transaction_info(),
        purpose: super::v1::sample_script_purpose(),
    }
}
