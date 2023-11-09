//! Proptest strategies for Plutus V2 types
//!
//! These strategies always return valid values.
use crate::generators::correct::v1::{
    arb_address, arb_datum, arb_datum_hash, arb_script_hash, arb_transaction_input, arb_value,
};
use crate::v2::datum::OutputDatum;
use crate::v2::transaction::{TransactionOutput, TxInInfo};
use proptest::option;
use proptest::prelude::{prop_oneof, Just};
use proptest::strategy::Strategy;

/// Strategy to generate transaction output
pub fn arb_transaction_output() -> impl Strategy<Value = TransactionOutput> {
    (
        arb_address(),
        arb_value(),
        arb_output_datum(),
        option::of(arb_script_hash()),
    )
        .prop_map(
            |(address, value, datum, reference_script)| TransactionOutput {
                address,
                value,
                datum,
                reference_script,
            },
        )
}

/// Strategy to generate an output datum
pub fn arb_output_datum() -> impl Strategy<Value = OutputDatum> {
    prop_oneof![
        Just(OutputDatum::None),
        arb_datum_hash().prop_map(OutputDatum::DatumHash),
        arb_datum().prop_map(OutputDatum::InlineDatum)
    ]
}

/// Strategy to generate a TxInInfo
pub fn arb_tx_in_info() -> impl Strategy<Value = TxInInfo> {
    (arb_transaction_input(), arb_transaction_output()).prop_map(|(transaction_input, resolved)| {
        TxInInfo {
            transaction_input,
            resolved,
        }
    })
}
