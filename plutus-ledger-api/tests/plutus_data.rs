#[cfg(test)]
mod plutusdata_roundtrip_tests {
    use plutus_ledger_api::plutus_data::{IsPlutusData, PlutusDataError};
    fn from_to_plutus_data<T>(val: &T) -> Result<T, PlutusDataError>
    where
        T: IsPlutusData + PartialEq,
    {
        T::from_plutus_data(&val.to_plutus_data())
    }

    mod golden_primitive {
        use std::collections::{BTreeMap, BTreeSet};

        use num_bigint::BigInt;
        use plutus_ledger_api::plutus_data::IsPlutusData;

        #[test]
        fn integer() {
            goldie::assert!(format!("{:?}", BigInt::from(123456789).to_plutus_data()))
        }

        #[test]
        fn bool() {
            goldie::assert!(format!("{:?}", true.to_plutus_data()))
        }

        #[test]
        fn char() {
            goldie::assert!(format!("{:?}", '凛'.to_plutus_data()))
        }

        #[test]
        fn bytes() {
            goldie::assert!(format!(
                "{:?}",
                [0u8, 1, 2, 3].repeat(10).to_vec().to_plutus_data()
            ))
        }

        #[test]
        fn text() {
            goldie::assert!(format!(
                "{:?}",
                String::from("Somethingsomething").to_plutus_data()
            ))
        }

        #[test]
        fn maybe_some() {
            goldie::assert!(format!("{:?}", Some(BigInt::from(1234)).to_plutus_data()))
        }

        #[test]
        fn maybe_none() {
            goldie::assert!(format!("{:?}", None::<BigInt>.to_plutus_data()))
        }

        #[test]
        fn result_ok() {
            goldie::assert!(format!("{:?}", Ok::<bool, BigInt>(false).to_plutus_data()))
        }

        #[test]
        fn result_err() {
            goldie::assert!(format!(
                "{:?}",
                Err::<bool, BigInt>(BigInt::from(1234)).to_plutus_data()
            ))
        }

        #[test]
        fn vec() {
            goldie::assert!(format!(
                "{:?}",
                [0, 1, 2, 3]
                    .repeat(20)
                    .into_iter()
                    .map(BigInt::from)
                    .collect::<Vec<BigInt>>()
                    .to_plutus_data()
            ))
        }

        #[test]
        fn set() {
            goldie::assert!(format!(
                "{:?}",
                [0, 1, 2]
                    .into_iter()
                    .map(BigInt::from)
                    .collect::<BTreeSet<BigInt>>()
                    .to_plutus_data()
            ))
        }

        #[test]
        fn map() {
            goldie::assert!(format!(
                "{:?}",
                [(0, "Hey"), (1, "There"), (2, "Foo"), (3, "Bar")]
                    .into_iter()
                    .map(|(k, v)| (BigInt::from(k), String::from(v)))
                    .collect::<BTreeMap<BigInt, String>>()
                    .to_plutus_data()
            ))
        }
    }

    mod prop_primitive {
        use super::from_to_plutus_data;
        use plutus_ledger_api::generators::correct::primitive::*;
        use proptest::collection::{btree_map, btree_set, vec};
        use proptest::option;
        use proptest::prelude::*;
        use proptest::result::maybe_err;

        proptest! {
            #[test]
            fn integer(val in arb_integer()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn bool(val in arb_bool() ) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn char(val in arb_char() ) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn bytes(val in arb_bytes() ) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn text(val in arb_text() ) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn maybe(val in option::of(arb_integer())) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn result(val in maybe_err(arb_bool(), arb_integer())) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn vector(val in vec(arb_integer(), 20)) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn set(val in btree_set(arb_integer(), 20)) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn map(val in btree_map(arb_integer(), arb_text(), 20)) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn complicated(val in arb_complicated()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }
        }
    }
    mod golden_v1 {
        use num_bigint::BigInt;
        use plutus_ledger_api::{
            plutus_data::{IsPlutusData, PlutusData},
            v1::{
                address::{Address, Credential, StakingCredential},
                assoc_map::AssocMap,
                crypto::{Ed25519PubKeyHash, LedgerBytes, PaymentPubKeyHash},
                datum::{Datum, DatumHash},
                interval::{Interval, PlutusInterval},
                redeemer::{Redeemer, RedeemerHash},
                script::{MintingPolicyHash, ScriptHash, ValidatorHash},
                transaction::{
                    DCert, POSIXTime, ScriptContext, ScriptPurpose, TransactionHash,
                    TransactionInfo, TransactionInput, TransactionOutput, TxInInfo,
                },
                value::{AssetClass, CurrencySymbol, TokenName, Value},
            },
        };

        pub(super) fn sample_currency_symbol() -> CurrencySymbol {
            CurrencySymbol::NativeToken(MintingPolicyHash(ScriptHash(LedgerBytes(
                [0].repeat(28).to_vec(),
            ))))
        }

        pub(super) fn sample_token_name() -> TokenName {
            TokenName::from_string("Something")
        }

        pub(super) fn sample_value() -> Value {
            Value::token_value(
                &sample_currency_symbol(),
                &sample_token_name(),
                &BigInt::from(123),
            )
        }

        pub(super) fn sample_plutus_interval() -> PlutusInterval<POSIXTime> {
            PlutusInterval::from(Interval::StartAt(POSIXTime(BigInt::from(1723106785))))
        }

        pub(super) fn sample_ed25519_pub_key_hash() -> Ed25519PubKeyHash {
            Ed25519PubKeyHash(LedgerBytes([0].repeat(28).to_vec()))
        }

        pub(super) fn sample_staking_credential() -> StakingCredential {
            StakingCredential::Hash(Credential::Script(ValidatorHash(ScriptHash(LedgerBytes(
                [1].repeat(28).to_vec(),
            )))))
        }

        pub(super) fn sample_address() -> Address {
            Address {
                credential: Credential::PubKey(sample_ed25519_pub_key_hash()),
                staking_credential: Some(sample_staking_credential()),
            }
        }

        pub(super) fn sample_transaction_hash() -> TransactionHash {
            TransactionHash(LedgerBytes([0].repeat(32).to_vec()))
        }

        pub(super) fn sample_transaction_input() -> TransactionInput {
            TransactionInput {
                transaction_id: sample_transaction_hash(),
                index: BigInt::from(3),
            }
        }

        pub(super) fn sample_datum_hash() -> DatumHash {
            DatumHash(LedgerBytes([0].repeat(32).to_vec()))
        }

        pub(super) fn sample_datum() -> Datum {
            Datum(PlutusData::constr(
                1,
                vec![PlutusData::bytes("Something".as_bytes().to_vec())],
            ))
        }

        pub(super) fn sample_redeemer() -> Redeemer {
            Redeemer(PlutusData::Integer(BigInt::from(144)))
        }

        pub(super) fn sample_tx_in_info() -> TxInInfo {
            TxInInfo {
                reference: sample_transaction_input(),
                output: sample_transaction_output(),
            }
        }

        pub(super) fn sample_transaction_output() -> TransactionOutput {
            TransactionOutput {
                address: sample_address(),
                value: sample_value(),
                datum_hash: Some(sample_datum_hash()),
            }
        }

        pub(super) fn sample_payment_pub_key_hash() -> PaymentPubKeyHash {
            PaymentPubKeyHash(sample_ed25519_pub_key_hash())
        }

        pub(super) fn sample_script_purpose() -> ScriptPurpose {
            ScriptPurpose::Minting(sample_currency_symbol())
        }

        pub(super) fn sample_dcert() -> DCert {
            DCert::DelegDelegate(sample_staking_credential(), sample_payment_pub_key_hash())
        }

        pub(super) fn sample_transaction_info() -> TransactionInfo {
            TransactionInfo {
                inputs: vec![sample_tx_in_info()],
                outputs: vec![sample_transaction_output()],
                fee: sample_value(),
                mint: sample_value(),
                d_cert: vec![sample_dcert()],
                wdrl: vec![(sample_staking_credential(), BigInt::from(12))],
                valid_range: sample_plutus_interval(),
                signatories: vec![sample_payment_pub_key_hash()],
                datums: vec![(sample_datum_hash(), sample_datum())],
                id: sample_transaction_hash(),
            }
        }

        #[test]
        fn v1_asset_class() {
            goldie::assert!(format!(
                "{:?}",
                AssetClass {
                    currency_symbol: sample_currency_symbol(),
                    token_name: sample_token_name()
                }
                .to_plutus_data()
            ))
        }

        #[test]
        fn v1_value() {
            goldie::assert!(format!("{:?}", sample_value().to_plutus_data()))
        }

        #[test]
        fn v1_plutus_interval() {
            goldie::assert!(format!("{:?}", sample_plutus_interval().to_plutus_data()))
        }

        #[test]
        fn v1_address() {
            goldie::assert!(format!("{:?}", sample_address().to_plutus_data()))
        }

        #[test]
        fn v1_transaction_input() {
            goldie::assert!(format!("{:?}", sample_transaction_input().to_plutus_data()))
        }

        #[test]
        fn v1_transaction_output() {
            goldie::assert!(format!(
                "{:?}",
                sample_transaction_output().to_plutus_data()
            ))
        }

        #[test]
        fn v1_tx_in_info() {
            goldie::assert!(format!("{:?}", sample_tx_in_info().to_plutus_data()))
        }

        #[test]
        fn v1_redeemeer() {
            goldie::assert!(format!("{:?}", sample_redeemer().to_plutus_data()))
        }

        #[test]
        fn v1_redeemeer_hash() {
            goldie::assert!(format!(
                "{:?}",
                RedeemerHash(LedgerBytes([0].repeat(32).to_vec())).to_plutus_data()
            ))
        }

        #[test]
        fn v1_datum_hash() {
            goldie::assert!(format!("{:?}", sample_datum_hash().to_plutus_data()))
        }

        #[test]
        fn v1_bigint_assoc_map() {
            goldie::assert!(format!(
                "{:?}",
                AssocMap::from(
                    [(1, 123), (0, 321), (2, 456)]
                        .into_iter()
                        .map(|(k, v)| (BigInt::from(k), BigInt::from(v)))
                        .collect::<Vec<_>>()
                )
                .to_plutus_data()
            ))
        }

        #[test]
        fn v1_arb_payment_pub_key_hash() {
            goldie::assert!(format!(
                "{:?}",
                sample_payment_pub_key_hash().to_plutus_data()
            ))
        }

        #[test]
        fn v1_d_cert() {
            goldie::assert!(format!("{:?}", sample_dcert().to_plutus_data()))
        }

        #[test]
        fn v1_script_purpose() {
            goldie::assert!(format!("{:?}", sample_script_purpose().to_plutus_data()))
        }

        #[test]
        fn v1_transaction_info() {
            goldie::assert!(format!("{:?}", sample_transaction_info().to_plutus_data()))
        }

        #[test]
        fn v1_script_context() {
            goldie::assert!(format!(
                "{:?}",
                ScriptContext {
                    tx_info: sample_transaction_info(),
                    purpose: sample_script_purpose()
                }
                .to_plutus_data()
            ))
        }
    }
    mod prop_v1 {
        use super::from_to_plutus_data;
        use plutus_ledger_api::generators::correct::{primitive::arb_integer, v1::*};
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn v1_asset_class(val in arb_asset_class()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_value(val in arb_value()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_plutus_data(val in arb_plutus_data()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_plutus_interval(val in arb_plutus_interval_posix_time()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_address(val in arb_address()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_transaction_input(val in arb_transaction_input()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_transaction_output(val in arb_transaction_output()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_tx_in_info(val in arb_tx_in_info()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_redeemeer(val in arb_redeemer()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_redeemeer_hash(val in arb_redeemer_hash()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_bigint_assoc_map(val in arb_assoc_map(arb_integer(), arb_integer())) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v1_arb_payment_pub_key_hash(val in arb_payment_pub_key_hash()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }

            #[test]
            fn v1_d_cert(val in arb_d_cert()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }

            #[test]
            fn v1_script_purpose(val in arb_script_purpose()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }

            #[test]
            fn v1_transaction_info(val in arb_transaction_info()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }

            #[test]
            fn v1_script_context(val in arb_script_context()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }
        }
    }
    mod golden_v2 {
        use super::from_to_plutus_data;
        use plutus_ledger_api::generators::correct::v2::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn v2_transaction_output(val in arb_transaction_output()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v2_tx_in_info(val in arb_tx_in_info()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v2_output_datum(val in arb_output_datum()) {
                assert_eq!(val, from_to_plutus_data(&val)?);
            }

            #[test]
            fn v2_transaction_info(val in arb_transaction_info()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }

            #[test]
            fn v2_script_context(val in arb_script_context()) {
                assert_eq!(val, from_to_plutus_data(&val)?)
            }
        }
    }
    mod prop_v2 {
        use num_bigint::BigInt;
        use plutus_ledger_api::{
            plutus_data::IsPlutusData,
            v2::{
                assoc_map::AssocMap,
                crypto::LedgerBytes,
                datum::OutputDatum,
                script::ScriptHash,
                transaction::{ScriptContext, TransactionInfo, TransactionOutput, TxInInfo},
            },
        };

        fn sample_output_datum() -> OutputDatum {
            OutputDatum::InlineDatum(super::golden_v1::sample_datum())
        }

        fn sample_transaction_output() -> TransactionOutput {
            TransactionOutput {
                address: super::golden_v1::sample_address(),
                value: super::golden_v1::sample_value(),
                datum: sample_output_datum(),
                reference_script: Some(ScriptHash(LedgerBytes([0].repeat(28).to_vec()))),
            }
        }

        pub fn sample_tx_in_info() -> TxInInfo {
            TxInInfo {
                reference: super::golden_v1::sample_transaction_input(),
                output: sample_transaction_output(),
            }
        }

        pub fn sample_transaction_info() -> TransactionInfo {
            TransactionInfo {
                inputs: vec![sample_tx_in_info()],
                outputs: vec![sample_transaction_output()],
                fee: super::golden_v1::sample_value(),
                mint: super::golden_v1::sample_value(),
                d_cert: vec![super::golden_v1::sample_dcert()],
                wdrl: AssocMap::from([(
                    super::golden_v1::sample_staking_credential(),
                    BigInt::from(12),
                )]),
                valid_range: super::golden_v1::sample_plutus_interval(),
                signatories: vec![super::golden_v1::sample_payment_pub_key_hash()],
                datums: AssocMap::from([(
                    super::golden_v1::sample_datum_hash(),
                    super::golden_v1::sample_datum(),
                )]),
                redeemers: AssocMap::from([(
                    super::golden_v1::sample_script_purpose(),
                    super::golden_v1::sample_redeemer(),
                )]),
                id: super::golden_v1::sample_transaction_hash(),
                reference_inputs: vec![sample_tx_in_info()],
            }
        }

        pub fn sample_script_context() -> ScriptContext {
            ScriptContext {
                tx_info: sample_transaction_info(),
                purpose: super::golden_v1::sample_script_purpose(),
            }
        }

        #[test]
        fn v2_transaction_output() {
            goldie::assert!(format!(
                "{:?}",
                sample_transaction_output().to_plutus_data()
            ))
        }

        #[test]
        fn v2_tx_in_info() {
            goldie::assert!(format!("{:?}", sample_tx_in_info().to_plutus_data()))
        }

        #[test]
        fn v2_output_datum() {
            goldie::assert!(format!("{:?}", sample_output_datum().to_plutus_data()))
        }

        #[test]
        fn v2_transaction_info() {
            goldie::assert!(format!("{:?}", sample_transaction_info().to_plutus_data()))
        }

        #[test]
        fn v2_script_context() {
            goldie::assert!(format!("{:?}", sample_script_context().to_plutus_data()))
        }
    }
}
