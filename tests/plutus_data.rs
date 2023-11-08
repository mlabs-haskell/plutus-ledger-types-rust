#[cfg(test)]
mod plutusdata_roundtrip_tests {
    use plutus_ledger_types::generators::correct::*;
    use plutus_ledger_types::plutus_data::{FromPlutusData, PlutusDataError, ToPlutusData};
    use proptest::collection::{btree_map, btree_set, vec};
    use proptest::option;
    use proptest::prelude::*;
    use proptest::result::maybe_err;

    fn from_to_plutus_data<T>(val: &T) -> Result<T, PlutusDataError>
    where
        T: ToPlutusData + FromPlutusData + PartialEq,
    {
        T::from_plutus_data(val.to_plutus_data())
    }

    proptest! {
        #[test]
        fn test_integer(val in arb_integer()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_bool(val in arb_bool() ) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_char(val in arb_char() ) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_bytes(val in arb_bytes() ) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_text(val in arb_text() ) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_maybe(val in option::of(arb_integer())) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_result(val in maybe_err(arb_bool(), arb_integer())) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_vec(val in vec(arb_integer(), 20)) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_set(val in btree_set(arb_integer(), 20)) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_map(val in btree_map(arb_integer(), arb_text(), 20)) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_complicated(val in arb_complicated()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_asset_class(val in arb_asset_class()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_value(val in arb_value()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_plutus_data(val in arb_plutus_data()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_plutus_interval(val in arb_plutus_interval_posix_time()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_address(val in arb_address()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_transaction_input(val in arb_transaction_input()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_transaction_output(val in arb_transaction_output()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_tx_in_info(val in arb_tx_in_info()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_output_datum(val in arb_output_datum()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }

    proptest! {
        #[test]
        fn test_redeemeer(val in arb_redeemer()) {
            assert_eq!(val, from_to_plutus_data(&val)?);
        }
    }
}
